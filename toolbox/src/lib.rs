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

pub fn read_to_string(input: &mut dyn Read) -> Result<String, std::io::Error> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    Ok(buf)
}

#[test]
fn test_we_can_read_to_a_string() {
    assert_eq!(read_to_string(&mut std::io::Cursor::new(b"")).unwrap(), "");
    assert_eq!(
        read_to_string(&mut std::io::Cursor::new(b"Hello, World!")).unwrap(),
        "Hello, World!"
    );

    struct ReadFailer;
    impl Read for ReadFailer {
        fn read(&mut self, _: &mut [u8]) -> Result<usize, std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, ""))
        }
    }

    assert!(read_to_string(&mut ReadFailer).is_err());
}

pub fn read_stdin_to_string() -> String {
    read_to_string(&mut std::io::stdin().lock()).unwrap()
}