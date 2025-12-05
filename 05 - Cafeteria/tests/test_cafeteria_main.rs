const EXECUTABLE_UNDER_TEST: &str = toolbox::binary_path!("cafeteria");

const TEST_CASES: &[(&str, &str)] = &[("", "Hello, world!\n")];

#[test]
fn test_that_the_executable_exists() {
    assert!(std::fs::exists(EXECUTABLE_UNDER_TEST).unwrap());
}

#[test]
fn test_that_the_executable_processes_the_rotations_and_outputs_the_passcode() {
    for &(input, expected_result) in TEST_CASES {
        let child_output = toolbox::invoke_executable(EXECUTABLE_UNDER_TEST, input);
        assert_eq!(child_output, expected_result);
    }
}
