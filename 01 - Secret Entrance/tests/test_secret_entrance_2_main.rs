use std::io::Read;
use std::io::Write;

#[test]
fn test_that_the_executable_exists() {
    let exe = env!("CARGO_BIN_EXE_secret_entrance_2");
    assert_eq!(true, std::fs::exists(exe).unwrap(), "{exe}");
}

#[test]
fn test_that_the_executable_processes_the_rotations_and_outputs_the_passcode() {
    for (input, expected_result) in [
        ("", "0\n"),
        ("L68\n", "1\n"),
        ("L68\nL30\nR48\n", "2\n"),
        ("L68\nL30\nR48\nL5\nR60\n", "3\n"),
        ("L68\nL30\nR48\nL5\nR60\nL55\n", "4\n"),
        ("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\n", "5\n"),
        ("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n", "6\n"),
    ] {
        // special env var at compile time:
        let exe = env!("CARGO_BIN_EXE_secret_entrance_2");

        let mut child = std::process::Command::new(exe)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("failed to run secret_entrance");

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

        assert_eq!(child_output, expected_result);
    }
}
