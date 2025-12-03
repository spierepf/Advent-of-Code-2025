#[macro_export]
macro_rules! binary_path {
    ( $path:literal ) => {
        env!(concat!("CARGO_BIN_EXE_", $path))
    };
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
