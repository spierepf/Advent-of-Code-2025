use trash_compactor::SAMPLE_INPUT;

const EXECUTABLE_UNDER_TEST: &str = toolbox::binary_path!("trash_compactor");

const TEST_CASES: &[(&str, &str)] = &[(SAMPLE_INPUT, "4277556\n")];

#[test]
fn test_that_the_executable_exists() {
    assert!(std::fs::exists(EXECUTABLE_UNDER_TEST).unwrap());
}

#[test]
fn test_that_the_executable_processes_the_input_and_produces_the_output() {
    for &(input, expected_result) in TEST_CASES {
        let child_output = toolbox::invoke_executable(EXECUTABLE_UNDER_TEST, input);
        assert_eq!(child_output, expected_result);
    }
}
