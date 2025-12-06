const EXECUTABLE_UNDER_TEST: &str = toolbox::binary_path!("cafeteria_2");

const TEST_CASES: &[(&str, &str)] = &[
    ("\n\n", "0\n"),
    ("4-4\n\n4\n", "1\n"),
    ("3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32\n", "14\n"),
];

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
