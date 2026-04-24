//! Pure therapy-control state machine for the CSSI Pulse Guard portfolio slice.
//!
//! The proof boundary is intentionally narrow: this crate models deterministic
//! state transitions and command outputs only. It does not model clinical
//! detection, hardware energy delivery, operator workflow, timing, or alarms.

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TherapyState {
    Off,
    Standby,
    Monitoring,
    Armed,
    Delivering,
    Faulted,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Command {
    NoCommand,
    PowerOn,
    PowerOff,
    StartMonitoring,
    Arm,
    Disarm,
    DeliverTherapy,
    TherapyComplete,
    FaultDetected,
    ResetFault,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TherapyInputs {
    pub command: Command,
    pub patient_connected: bool,
    pub therapy_inhibit: bool,
    pub valid_therapy_detection: bool,
    pub hardware_ready: bool,
    pub self_test_passed: bool,
}

impl TherapyInputs {
    pub const fn nominal(command: Command) -> Self {
        Self {
            command,
            patient_connected: true,
            therapy_inhibit: false,
            valid_therapy_detection: true,
            hardware_ready: true,
            self_test_passed: true,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BlockedReason {
    SystemOff,
    InvalidCommandForState,
    PatientDisconnected,
    TherapyInhibited,
    DetectionAbsent,
    HardwareNotReady,
    SelfTestFailed,
    FaultLatched,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TransitionResult {
    pub state: TherapyState,
    pub therapy_output_enabled: bool,
    pub therapy_delivery_requested: bool,
    pub fault_latched: bool,
    pub blocked_reason: Option<BlockedReason>,
}

pub fn transition(current: TherapyState, inputs: TherapyInputs) -> TransitionResult {
    if inputs.command == Command::PowerOff {
        return result(TherapyState::Off, false, None);
    }

    if inputs.command == Command::FaultDetected {
        return result(TherapyState::Faulted, false, None);
    }

    match current {
        TherapyState::Off => transition_from_off(inputs),
        TherapyState::Standby => transition_from_standby(inputs),
        TherapyState::Monitoring => transition_from_monitoring(inputs),
        TherapyState::Armed => transition_from_armed(inputs),
        TherapyState::Delivering => transition_from_delivering(inputs),
        TherapyState::Faulted => transition_from_faulted(inputs),
    }
}

fn transition_from_off(inputs: TherapyInputs) -> TransitionResult {
    match inputs.command {
        Command::PowerOn if inputs.hardware_ready && inputs.self_test_passed => {
            result(TherapyState::Standby, false, None)
        }
        Command::PowerOn if !inputs.hardware_ready => result(
            TherapyState::Faulted,
            false,
            Some(BlockedReason::HardwareNotReady),
        ),
        Command::PowerOn if !inputs.self_test_passed => result(
            TherapyState::Faulted,
            false,
            Some(BlockedReason::SelfTestFailed),
        ),
        Command::NoCommand => result(TherapyState::Off, false, None),
        _ => result(TherapyState::Off, false, Some(BlockedReason::SystemOff)),
    }
}

fn transition_from_standby(inputs: TherapyInputs) -> TransitionResult {
    match inputs.command {
        Command::StartMonitoring if inputs.patient_connected => {
            result(TherapyState::Monitoring, false, None)
        }
        Command::StartMonitoring => result(
            TherapyState::Standby,
            false,
            Some(BlockedReason::PatientDisconnected),
        ),
        Command::NoCommand => result(TherapyState::Standby, false, None),
        _ => result(
            TherapyState::Standby,
            false,
            Some(BlockedReason::InvalidCommandForState),
        ),
    }
}

fn transition_from_monitoring(inputs: TherapyInputs) -> TransitionResult {
    match inputs.command {
        Command::Arm if arming_preconditions(inputs).is_none() => {
            result(TherapyState::Armed, true, None)
        }
        Command::Arm => result(
            TherapyState::Monitoring,
            false,
            arming_preconditions(inputs),
        ),
        Command::NoCommand => result(TherapyState::Monitoring, false, None),
        _ => result(
            TherapyState::Monitoring,
            false,
            Some(BlockedReason::InvalidCommandForState),
        ),
    }
}

fn transition_from_armed(inputs: TherapyInputs) -> TransitionResult {
    match inputs.command {
        Command::Disarm => result(TherapyState::Monitoring, false, None),
        Command::DeliverTherapy if delivery_preconditions(inputs).is_none() => TransitionResult {
            state: TherapyState::Delivering,
            therapy_output_enabled: false,
            therapy_delivery_requested: true,
            fault_latched: false,
            blocked_reason: None,
        },
        Command::DeliverTherapy => result(
            TherapyState::Monitoring,
            false,
            delivery_preconditions(inputs),
        ),
        Command::NoCommand if delivery_preconditions(inputs).is_none() => {
            result(TherapyState::Armed, true, None)
        }
        Command::NoCommand => result(
            TherapyState::Monitoring,
            false,
            delivery_preconditions(inputs),
        ),
        _ if delivery_preconditions(inputs).is_none() => result(
            TherapyState::Armed,
            true,
            Some(BlockedReason::InvalidCommandForState),
        ),
        _ => result(
            TherapyState::Monitoring,
            false,
            delivery_preconditions(inputs),
        ),
    }
}

fn transition_from_delivering(inputs: TherapyInputs) -> TransitionResult {
    match inputs.command {
        Command::TherapyComplete => result(TherapyState::Monitoring, false, None),
        Command::NoCommand => result(TherapyState::Delivering, false, None),
        _ => result(
            TherapyState::Delivering,
            false,
            Some(BlockedReason::InvalidCommandForState),
        ),
    }
}

fn transition_from_faulted(inputs: TherapyInputs) -> TransitionResult {
    match inputs.command {
        Command::ResetFault if inputs.hardware_ready && inputs.self_test_passed => {
            result(TherapyState::Standby, false, None)
        }
        Command::ResetFault if !inputs.hardware_ready => result(
            TherapyState::Faulted,
            false,
            Some(BlockedReason::HardwareNotReady),
        ),
        Command::ResetFault if !inputs.self_test_passed => result(
            TherapyState::Faulted,
            false,
            Some(BlockedReason::SelfTestFailed),
        ),
        Command::NoCommand => result(TherapyState::Faulted, false, None),
        _ => result(
            TherapyState::Faulted,
            false,
            Some(BlockedReason::FaultLatched),
        ),
    }
}

fn arming_preconditions(inputs: TherapyInputs) -> Option<BlockedReason> {
    if !inputs.patient_connected {
        Some(BlockedReason::PatientDisconnected)
    } else if inputs.therapy_inhibit {
        Some(BlockedReason::TherapyInhibited)
    } else if !inputs.valid_therapy_detection {
        Some(BlockedReason::DetectionAbsent)
    } else if !inputs.hardware_ready {
        Some(BlockedReason::HardwareNotReady)
    } else if !inputs.self_test_passed {
        Some(BlockedReason::SelfTestFailed)
    } else {
        None
    }
}

fn delivery_preconditions(inputs: TherapyInputs) -> Option<BlockedReason> {
    arming_preconditions(inputs)
}

fn result(
    state: TherapyState,
    therapy_output_enabled: bool,
    blocked_reason: Option<BlockedReason>,
) -> TransitionResult {
    TransitionResult {
        state,
        therapy_output_enabled,
        therapy_delivery_requested: false,
        fault_latched: state == TherapyState::Faulted,
        blocked_reason,
    }
}

#[cfg(kani)]
mod proofs {
    use super::*;

    #[kani::proof]
    fn inv_001_inhibit_blocks_delivery_request() {
        let state = any_state();
        let inputs = TherapyInputs {
            command: any_command(),
            patient_connected: kani::any(),
            therapy_inhibit: true,
            valid_therapy_detection: kani::any(),
            hardware_ready: kani::any(),
            self_test_passed: kani::any(),
        };

        let outcome = transition(state, inputs);

        assert!(!outcome.therapy_delivery_requested);
    }

    #[kani::proof]
    fn inv_002_delivery_requires_armed_deliver_command() {
        let state = any_state();
        let inputs = any_inputs();

        let outcome = transition(state, inputs);

        if outcome.therapy_delivery_requested {
            assert_eq!(state, TherapyState::Armed);
            assert_eq!(inputs.command, Command::DeliverTherapy);
        }
    }

    #[kani::proof]
    fn inv_003_delivery_requires_valid_detection() {
        let state = any_state();
        let mut inputs = any_inputs();
        inputs.valid_therapy_detection = false;

        let outcome = transition(state, inputs);

        assert!(!outcome.therapy_delivery_requested);
    }

    #[kani::proof]
    fn inv_004_delivery_requires_connection_readiness_and_self_test() {
        let state = any_state();
        let inputs = any_inputs();

        let outcome = transition(state, inputs);

        if outcome.therapy_delivery_requested {
            assert!(inputs.patient_connected);
            assert!(inputs.hardware_ready);
            assert!(inputs.self_test_passed);
        }
    }

    #[kani::proof]
    fn inv_005_faulted_never_requests_or_enables_therapy() {
        let inputs = any_inputs();

        let outcome = transition(TherapyState::Faulted, inputs);

        assert!(!outcome.therapy_delivery_requested);
        assert!(!outcome.therapy_output_enabled);
    }

    fn any_inputs() -> TherapyInputs {
        TherapyInputs {
            command: any_command(),
            patient_connected: kani::any(),
            therapy_inhibit: kani::any(),
            valid_therapy_detection: kani::any(),
            hardware_ready: kani::any(),
            self_test_passed: kani::any(),
        }
    }

    fn any_state() -> TherapyState {
        match kani::any::<u8>() % 6 {
            0 => TherapyState::Off,
            1 => TherapyState::Standby,
            2 => TherapyState::Monitoring,
            3 => TherapyState::Armed,
            4 => TherapyState::Delivering,
            _ => TherapyState::Faulted,
        }
    }

    fn any_command() -> Command {
        match kani::any::<u8>() % 10 {
            0 => Command::NoCommand,
            1 => Command::PowerOn,
            2 => Command::PowerOff,
            3 => Command::StartMonitoring,
            4 => Command::Arm,
            5 => Command::Disarm,
            6 => Command::DeliverTherapy,
            7 => Command::TherapyComplete,
            8 => Command::FaultDetected,
            _ => Command::ResetFault,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inhibited_delivery_attempt_is_blocked_and_disarmed() {
        let inputs = TherapyInputs {
            therapy_inhibit: true,
            ..TherapyInputs::nominal(Command::DeliverTherapy)
        };

        let outcome = transition(TherapyState::Armed, inputs);

        assert_eq!(outcome.state, TherapyState::Monitoring);
        assert!(!outcome.therapy_delivery_requested);
        assert!(!outcome.therapy_output_enabled);
        assert_eq!(
            outcome.blocked_reason,
            Some(BlockedReason::TherapyInhibited)
        );
    }

    #[test]
    fn armed_state_drops_output_enable_when_interlock_becomes_false() {
        let outcome = transition(
            TherapyState::Armed,
            TherapyInputs {
                therapy_inhibit: true,
                ..TherapyInputs::nominal(Command::NoCommand)
            },
        );

        assert_eq!(outcome.state, TherapyState::Monitoring);
        assert!(!outcome.therapy_output_enabled);
        assert!(!outcome.therapy_delivery_requested);
        assert_eq!(
            outcome.blocked_reason,
            Some(BlockedReason::TherapyInhibited)
        );
    }

    #[test]
    fn therapy_request_requires_armed_state_and_delivery_command() {
        let from_monitoring = transition(
            TherapyState::Monitoring,
            TherapyInputs::nominal(Command::DeliverTherapy),
        );
        let from_armed = transition(
            TherapyState::Armed,
            TherapyInputs::nominal(Command::DeliverTherapy),
        );

        assert!(!from_monitoring.therapy_delivery_requested);
        assert_eq!(from_armed.state, TherapyState::Delivering);
        assert!(from_armed.therapy_delivery_requested);
    }

    #[test]
    fn arming_requires_detection_connection_readiness_and_self_test() {
        let disconnected = transition(
            TherapyState::Monitoring,
            TherapyInputs {
                patient_connected: false,
                ..TherapyInputs::nominal(Command::Arm)
            },
        );
        let absent_detection = transition(
            TherapyState::Monitoring,
            TherapyInputs {
                valid_therapy_detection: false,
                ..TherapyInputs::nominal(Command::Arm)
            },
        );
        let nominal = transition(
            TherapyState::Monitoring,
            TherapyInputs::nominal(Command::Arm),
        );

        assert_eq!(disconnected.state, TherapyState::Monitoring);
        assert_eq!(
            disconnected.blocked_reason,
            Some(BlockedReason::PatientDisconnected)
        );
        assert_eq!(absent_detection.state, TherapyState::Monitoring);
        assert_eq!(
            absent_detection.blocked_reason,
            Some(BlockedReason::DetectionAbsent)
        );
        assert_eq!(nominal.state, TherapyState::Armed);
        assert!(nominal.therapy_output_enabled);
    }

    #[test]
    fn fault_detection_latches_fault_and_disables_outputs() {
        let outcome = transition(
            TherapyState::Armed,
            TherapyInputs::nominal(Command::FaultDetected),
        );

        assert_eq!(outcome.state, TherapyState::Faulted);
        assert!(outcome.fault_latched);
        assert!(!outcome.therapy_output_enabled);
        assert!(!outcome.therapy_delivery_requested);
    }

    #[test]
    fn fault_reset_requires_readiness_and_self_test() {
        let failed_reset = transition(
            TherapyState::Faulted,
            TherapyInputs {
                hardware_ready: false,
                ..TherapyInputs::nominal(Command::ResetFault)
            },
        );
        let successful_reset = transition(
            TherapyState::Faulted,
            TherapyInputs::nominal(Command::ResetFault),
        );

        assert_eq!(failed_reset.state, TherapyState::Faulted);
        assert_eq!(
            failed_reset.blocked_reason,
            Some(BlockedReason::HardwareNotReady)
        );
        assert_eq!(successful_reset.state, TherapyState::Standby);
    }
}
