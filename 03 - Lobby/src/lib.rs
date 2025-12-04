pub fn max_joltage(bank: &str, connection_count: usize) -> u64 {
    let digits = max_joltage_digits(
        &bank
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<_>>(),
        connection_count,
    );

    let mut joltage: u64 = 0;
    for digit in digits.iter() {
        joltage = joltage * 10 + digit;
    }
    joltage
}

fn max_joltage_digits(digits: &[u64], connection_count: usize) -> Vec<u64> {
    if digits.len() == connection_count {
        digits.to_vec()
    } else if digits.len() == connection_count + 1 {
        // figure out what the largest number is when dropping one digit
        (0..digits.len())
            .map(|i| {
                let mut tmp = digits.to_vec();
                tmp.remove(i);
                tmp
            })
            .max()
            .unwrap()
    } else {
        let mut subcase = max_joltage_digits(&digits[1..], connection_count);
        subcase.insert(0, digits[0]);
        max_joltage_digits(&subcase, connection_count)
    }
}

#[test]
fn we_can_find_the_max_joltage_of_a_bank() {
    assert_eq!(max_joltage("1", 1), 1);
    assert_eq!(max_joltage("2", 1), 2);
    assert_eq!(max_joltage("12", 2), 12);
    assert_eq!(max_joltage("12", 1), 2);
    assert_eq!(max_joltage("21", 1), 2);
    assert_eq!(max_joltage("213", 2), 23);
    assert_eq!(max_joltage("2113", 2), 23);
    assert_eq!(max_joltage("987654321111111", 2), 98);
    assert_eq!(max_joltage("811111111111119", 2), 89);
    assert_eq!(max_joltage("234234234234278", 2), 78);
    assert_eq!(max_joltage("818181911112111", 2), 92);
    assert_eq!(max_joltage("987654321111111", 3), 987);
    assert_eq!(max_joltage("987654321111111", 12), 987654321111);
    assert_eq!(max_joltage("811111111111119", 12), 811111111119);
    assert_eq!(max_joltage("234234234234278", 12), 434234234278);
    assert_eq!(max_joltage("818181911112111", 12), 888911112111);
}

pub fn calculate_total_joltage(input: &mut dyn std::io::BufRead, battery_count: usize) -> u64 {
    use std::io::BufRead;
    input
        .lines()
        .map(|line| max_joltage(&line.unwrap(), battery_count))
        .sum()
}
#[test]
fn we_can_calculate_total_joltage_of_a_series_of_banks() {
    assert_eq!(
        calculate_total_joltage(&mut std::io::Cursor::new("".as_bytes()), 2),
        0
    );
    assert_eq!(
        calculate_total_joltage(&mut std::io::Cursor::new("987654321111111\n".as_bytes()), 2),
        98
    );
    assert_eq!(
        calculate_total_joltage(
            &mut std::io::Cursor::new(
                "987654321111111\n811111111111119\n234234234234278\n818181911112111\n".as_bytes()
            ),
            2
        ),
        357
    );
    assert_eq!(
        calculate_total_joltage(
            &mut std::io::Cursor::new(
                "987654321111111\n811111111111119\n234234234234278\n818181911112111\n".as_bytes()
            ),
            12
        ),
        3121910778619
    );
}
