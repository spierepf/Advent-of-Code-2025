use std::io::Read;
use std::io::Write;

#[test]
fn test_that_the_level_one_executable_exists() {
    assert!(std::fs::exists(toolbox::binary_path!("gift_shop")).unwrap());
}

#[test]
fn test_that_the_level_two_executable_exists() {
    assert!(std::fs::exists(toolbox::binary_path!("gift_shop_2")).unwrap());
}

#[test]
fn test_can_use_add_fuction_from_toolbox() {
    assert_eq!(toolbox::add(1, 2), 3);
}

fn run_executable_with_input_and_capture_output(path: &str, input: &str) -> String {
    let mut child = std::process::Command::new(path)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect(&format!("failed to run {}", path));

    let mut child_stdin = child.stdin.take().unwrap();
    let mut child_stdout = child.stdout.take().unwrap();

    write!(&mut child_stdin, "{input}").expect("failed to write to child");

    drop(child_stdin);

    let mut child_output = String::new();
    (&mut child_stdout)
        .read_to_string(&mut child_output)
        .expect("failed to read child output");

    let res = child.wait().expect("failed to wait for child");
    assert!(res.success());

    child_output
}

#[test]
fn test_that_the_level_one_executable_processes_the_input_and_provides_thw_correct_output() {
    for (input, expected_result) in [
        ("", "0\n"),
        (
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
            "1227775554\n",
        ),
        (
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124\n",
            "1227775554\n",
        ),
    ] {
        // special env var at compile time:
        let exe = toolbox::binary_path!("gift_shop");
        let child_output = run_executable_with_input_and_capture_output(exe, input);
        assert_eq!(child_output, expected_result);
    }
}

#[test]
fn test_that_the_level_two_executable_processes_the_input_and_provides_thw_correct_output() {
    for (input, expected_result) in [
        ("", "0\n"),
        (
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
            "4174379265\n",
        ),
        (
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124\n",
            "4174379265\n",
        ),
    ] {
        // special env var at compile time:
        let exe = toolbox::binary_path!("gift_shop_2");
        let child_output = run_executable_with_input_and_capture_output(exe, input);
        assert_eq!(child_output, expected_result);
    }
}
