use pulse_guard_core::{
    transition, BlockedReason, Command, TherapyInputs, TherapyState, TransitionResult,
};

#[derive(Copy, Clone, Debug)]
struct StepExpectation {
    command: Command,
    state: TherapyState,
    therapy_output_enabled: bool,
    therapy_delivery_requested: bool,
    fault_latched: bool,
    blocked_reason: Option<BlockedReason>,
}

impl StepExpectation {
    const fn new(command: Command, state: TherapyState) -> Self {
        Self {
            command,
            state,
            therapy_output_enabled: false,
            therapy_delivery_requested: false,
            fault_latched: false,
            blocked_reason: None,
        }
    }

    const fn output_enabled(mut self) -> Self {
        self.therapy_output_enabled = true;
        self
    }

    const fn delivery_requested(mut self) -> Self {
        self.therapy_delivery_requested = true;
        self
    }

    const fn fault_latched(mut self) -> Self {
        self.fault_latched = true;
        self
    }

    const fn blocked(mut self, reason: BlockedReason) -> Self {
        self.blocked_reason = Some(reason);
        self
    }
}

#[test]
fn nominal_sequence_powers_on_monitors_arms_delivers_and_completes() {
    let final_state = run_sequence(
        TherapyState::Off,
        &[
            StepExpectation::new(Command::PowerOn, TherapyState::Standby),
            StepExpectation::new(Command::StartMonitoring, TherapyState::Monitoring),
            StepExpectation::new(Command::Arm, TherapyState::Armed).output_enabled(),
            StepExpectation::new(Command::DeliverTherapy, TherapyState::Delivering)
                .delivery_requested(),
            StepExpectation::new(Command::TherapyComplete, TherapyState::Monitoring),
        ],
    );

    assert_eq!(final_state, TherapyState::Monitoring);
}

#[test]
fn blocked_arming_sequence_stays_in_monitoring_until_preconditions_recover() {
    let mut state = TherapyState::Off;

    state = expect_step(
        state,
        TherapyInputs::nominal(Command::PowerOn),
        StepExpectation::new(Command::PowerOn, TherapyState::Standby),
    )
    .state;
    state = expect_step(
        state,
        TherapyInputs::nominal(Command::StartMonitoring),
        StepExpectation::new(Command::StartMonitoring, TherapyState::Monitoring),
    )
    .state;
    state = expect_step(
        state,
        TherapyInputs {
            valid_therapy_detection: false,
            ..TherapyInputs::nominal(Command::Arm)
        },
        StepExpectation::new(Command::Arm, TherapyState::Monitoring)
            .blocked(BlockedReason::DetectionAbsent),
    )
    .state;
    state = expect_step(
        state,
        TherapyInputs::nominal(Command::Arm),
        StepExpectation::new(Command::Arm, TherapyState::Armed).output_enabled(),
    )
    .state;

    assert_eq!(state, TherapyState::Armed);
}

#[test]
fn inhibit_during_armed_sequence_disarms_without_requesting_therapy() {
    let mut state = run_sequence(
        TherapyState::Off,
        &[
            StepExpectation::new(Command::PowerOn, TherapyState::Standby),
            StepExpectation::new(Command::StartMonitoring, TherapyState::Monitoring),
            StepExpectation::new(Command::Arm, TherapyState::Armed).output_enabled(),
        ],
    );

    let inhibited = expect_step(
        state,
        TherapyInputs {
            therapy_inhibit: true,
            ..TherapyInputs::nominal(Command::DeliverTherapy)
        },
        StepExpectation::new(Command::DeliverTherapy, TherapyState::Monitoring)
            .blocked(BlockedReason::TherapyInhibited),
    );
    state = inhibited.state;

    assert_eq!(state, TherapyState::Monitoring);
    assert!(!inhibited.therapy_delivery_requested);
    assert!(!inhibited.therapy_output_enabled);
}

#[test]
fn fault_sequence_latches_fault_then_resets_only_after_readiness_returns() {
    let mut state = run_sequence(
        TherapyState::Off,
        &[
            StepExpectation::new(Command::PowerOn, TherapyState::Standby),
            StepExpectation::new(Command::StartMonitoring, TherapyState::Monitoring),
            StepExpectation::new(Command::Arm, TherapyState::Armed).output_enabled(),
        ],
    );

    state = expect_step(
        state,
        TherapyInputs::nominal(Command::FaultDetected),
        StepExpectation::new(Command::FaultDetected, TherapyState::Faulted).fault_latched(),
    )
    .state;
    state = expect_step(
        state,
        TherapyInputs {
            hardware_ready: false,
            ..TherapyInputs::nominal(Command::ResetFault)
        },
        StepExpectation::new(Command::ResetFault, TherapyState::Faulted)
            .fault_latched()
            .blocked(BlockedReason::HardwareNotReady),
    )
    .state;
    state = expect_step(
        state,
        TherapyInputs::nominal(Command::ResetFault),
        StepExpectation::new(Command::ResetFault, TherapyState::Standby),
    )
    .state;

    assert_eq!(state, TherapyState::Standby);
}

#[test]
fn power_off_from_any_active_sequence_disables_outputs_and_clears_fault_latch() {
    let armed = run_sequence(
        TherapyState::Off,
        &[
            StepExpectation::new(Command::PowerOn, TherapyState::Standby),
            StepExpectation::new(Command::StartMonitoring, TherapyState::Monitoring),
            StepExpectation::new(Command::Arm, TherapyState::Armed).output_enabled(),
        ],
    );

    let powered_off = expect_step(
        armed,
        TherapyInputs::nominal(Command::PowerOff),
        StepExpectation::new(Command::PowerOff, TherapyState::Off),
    );

    assert_eq!(powered_off.state, TherapyState::Off);
    assert!(!powered_off.therapy_output_enabled);
    assert!(!powered_off.therapy_delivery_requested);
    assert!(!powered_off.fault_latched);
}

fn run_sequence(initial_state: TherapyState, expectations: &[StepExpectation]) -> TherapyState {
    expectations
        .iter()
        .fold(initial_state, |state, expectation| {
            expect_step(
                state,
                TherapyInputs::nominal(expectation.command),
                *expectation,
            )
            .state
        })
}

fn expect_step(
    state: TherapyState,
    inputs: TherapyInputs,
    expectation: StepExpectation,
) -> TransitionResult {
    let outcome = transition(state, inputs);

    assert_eq!(outcome.state, expectation.state);
    assert_eq!(
        outcome.therapy_output_enabled,
        expectation.therapy_output_enabled
    );
    assert_eq!(
        outcome.therapy_delivery_requested,
        expectation.therapy_delivery_requested
    );
    assert_eq!(outcome.fault_latched, expectation.fault_latched);
    assert_eq!(outcome.blocked_reason, expectation.blocked_reason);

    outcome
}
