const EXECUTABLE_UNDER_TEST: &str = toolbox::binary_path!("secret_entrance");

const TEST_CASES: &[(&str, &str)] = &[
    ("", "0\n"),
    ("L68\n", "0\n"),
    ("L68\nL30\nR48\n", "1\n"),
    ("L68\nL30\nR48\nL5\nR60\nL55\nL1\n", "2\n"),
    ("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n", "3\n"),
];

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
