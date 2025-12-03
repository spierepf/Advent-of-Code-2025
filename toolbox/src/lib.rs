use std::io::Read;
use std::io::Write;

#[macro_export]
macro_rules! binary_path {
    ( $path:literal ) => {
        env!(concat!("CARGO_BIN_EXE_", $path))
    };
}

pub fn invoke_executable(path: &str, input: &str) -> String {
    let mut child = std::process::Command::new(path)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|_| panic!("failed to run {}", path));

    let mut child_stdin = child.stdin.take().unwrap();
    let mut child_stdout = child.stdout.take().unwrap();

    write!(&mut child_stdin, "{input}").expect("failed to write to child");

    drop(child_stdin);

    let mut child_output = String::new();
    child_stdout
        .read_to_string(&mut child_output)
        .expect("failed to read child output");

    let res = child.wait().expect("failed to wait for child");
    assert!(res.success());

    child_output
}
