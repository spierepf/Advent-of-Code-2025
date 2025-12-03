use std::cmp::max;

#[test]
fn we_can_find_the_max_joltage_of_a_bank() {
    assert_eq!(max_joltage("987654321111111"), 98);
    assert_eq!(max_joltage("811111111111119"), 89);
    assert_eq!(max_joltage("234234234234278"), 78);
    assert_eq!(max_joltage("818181911112111"), 92);
}

pub fn max_joltage(bank: &str) -> u32 {
    let bank = bank.as_bytes();
    let mut max_joltage = 0;

    for first_digit in 0..bank.len() {
        for second_digit in first_digit+1..bank.len() {
            let joltage = ((bank[first_digit] - b'0') as u32) * 10 + ((bank[second_digit] - b'0') as u32);
            max_joltage = max(max_joltage, joltage);
        }
    }
    max_joltage
}

pub fn calculate_total_joltage(input: &mut dyn std::io::BufRead) -> u32 {
    use std::io::BufRead;
    input
        .lines()
        .map(|line| max_joltage(&line.unwrap()))
        .sum()
}
#[test]
fn we_can_calculate_total_joltage_of_a_series_of_banks() {
    assert_eq!(
        calculate_total_joltage(&mut std::io::Cursor::new(
            "".as_bytes()
        )),
        0
    );
    assert_eq!(
        calculate_total_joltage(&mut std::io::Cursor::new(
            "987654321111111\n".as_bytes()
        )),
        98
    );
    assert_eq!(
        calculate_total_joltage(&mut std::io::Cursor::new(
            "987654321111111\n811111111111119\n234234234234278\n818181911112111\n".as_bytes()
        )),
        357
    );
}
