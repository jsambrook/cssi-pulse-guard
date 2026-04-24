use pulse_guard_core::{transition, Command, TherapyInputs, TherapyState};

const STATES: [TherapyState; 6] = [
    TherapyState::Off,
    TherapyState::Standby,
    TherapyState::Monitoring,
    TherapyState::Armed,
    TherapyState::Delivering,
    TherapyState::Faulted,
];

const COMMANDS: [Command; 10] = [
    Command::NoCommand,
    Command::PowerOn,
    Command::PowerOff,
    Command::StartMonitoring,
    Command::Arm,
    Command::Disarm,
    Command::DeliverTherapy,
    Command::TherapyComplete,
    Command::FaultDetected,
    Command::ResetFault,
];

#[test]
fn generated_input_matrix_preserves_delivery_safety_invariants() {
    for state in STATES {
        for command in COMMANDS {
            for inputs in adversarial_inputs(command) {
                let outcome = transition(state, inputs);

                if outcome.therapy_delivery_requested {
                    assert_eq!(state, TherapyState::Armed);
                    assert_eq!(command, Command::DeliverTherapy);
                    assert!(inputs.patient_connected);
                    assert!(!inputs.therapy_inhibit);
                    assert!(inputs.valid_therapy_detection);
                    assert!(inputs.hardware_ready);
                    assert!(inputs.self_test_passed);
                    assert_eq!(outcome.state, TherapyState::Delivering);
                    assert!(!outcome.therapy_output_enabled);
                    assert!(!outcome.fault_latched);
                }

                if state == TherapyState::Faulted {
                    assert!(!outcome.therapy_delivery_requested);
                    assert!(!outcome.therapy_output_enabled);
                }

                if inputs.command == Command::PowerOff {
                    assert_eq!(outcome.state, TherapyState::Off);
                    assert!(!outcome.therapy_output_enabled);
                    assert!(!outcome.therapy_delivery_requested);
                    assert!(!outcome.fault_latched);
                }

                if inputs.command == Command::FaultDetected {
                    assert_eq!(outcome.state, TherapyState::Faulted);
                    assert!(!outcome.therapy_output_enabled);
                    assert!(!outcome.therapy_delivery_requested);
                    assert!(outcome.fault_latched);
                }
            }
        }
    }
}

#[test]
fn invalid_command_matrix_never_requests_therapy() {
    for state in STATES {
        for command in invalid_commands_for(state) {
            for inputs in adversarial_inputs(command) {
                let outcome = transition(state, inputs);

                assert!(!outcome.therapy_delivery_requested);

                if state == TherapyState::Faulted {
                    assert!(!outcome.therapy_output_enabled);
                }
            }
        }
    }
}

#[test]
fn armed_state_drops_out_of_armed_when_any_delivery_precondition_fails() {
    for inputs in adversarial_inputs(Command::NoCommand) {
        let delivery_preconditions_hold = inputs.patient_connected
            && !inputs.therapy_inhibit
            && inputs.valid_therapy_detection
            && inputs.hardware_ready
            && inputs.self_test_passed;

        let outcome = transition(TherapyState::Armed, inputs);

        if delivery_preconditions_hold {
            assert_eq!(outcome.state, TherapyState::Armed);
            assert!(outcome.therapy_output_enabled);
        } else {
            assert_eq!(outcome.state, TherapyState::Monitoring);
            assert!(!outcome.therapy_output_enabled);
            assert!(!outcome.therapy_delivery_requested);
        }
    }
}

fn adversarial_inputs(command: Command) -> Vec<TherapyInputs> {
    let mut cases = Vec::new();

    for patient_connected in [false, true] {
        for therapy_inhibit in [false, true] {
            for valid_therapy_detection in [false, true] {
                for hardware_ready in [false, true] {
                    for self_test_passed in [false, true] {
                        cases.push(TherapyInputs {
                            command,
                            patient_connected,
                            therapy_inhibit,
                            valid_therapy_detection,
                            hardware_ready,
                            self_test_passed,
                        });
                    }
                }
            }
        }
    }

    cases
}

fn invalid_commands_for(state: TherapyState) -> Vec<Command> {
    COMMANDS
        .into_iter()
        .filter(|command| !is_valid_command_for(state, *command))
        .collect()
}

fn is_valid_command_for(state: TherapyState, command: Command) -> bool {
    match state {
        TherapyState::Off => matches!(
            command,
            Command::NoCommand | Command::PowerOn | Command::PowerOff | Command::FaultDetected
        ),
        TherapyState::Standby => matches!(
            command,
            Command::NoCommand
                | Command::StartMonitoring
                | Command::PowerOff
                | Command::FaultDetected
        ),
        TherapyState::Monitoring => matches!(
            command,
            Command::NoCommand | Command::Arm | Command::PowerOff | Command::FaultDetected
        ),
        TherapyState::Armed => matches!(
            command,
            Command::NoCommand
                | Command::Disarm
                | Command::DeliverTherapy
                | Command::PowerOff
                | Command::FaultDetected
        ),
        TherapyState::Delivering => matches!(
            command,
            Command::NoCommand
                | Command::TherapyComplete
                | Command::PowerOff
                | Command::FaultDetected
        ),
        TherapyState::Faulted => matches!(
            command,
            Command::NoCommand | Command::ResetFault | Command::PowerOff | Command::FaultDetected
        ),
    }
}
