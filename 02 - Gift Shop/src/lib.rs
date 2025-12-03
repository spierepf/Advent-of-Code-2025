use std::{num::ParseIntError, str::FromStr};

#[test]
fn we_can_parse_a_single_range() {
    assert_eq!(ProductIdRange(11, 22), ProductIdRange(11, 22));
    assert_eq!(
        ProductIdRange(5858547751, 585862602011),
        ProductIdRange(5858547751, 585862602011)
    );

    assert_eq!(
        "11-22".parse::<ProductIdRange>(),
        Ok(ProductIdRange(11, 22))
    );
    assert_eq!(
        "95-115".parse::<ProductIdRange>(),
        Ok(ProductIdRange(95, 115))
    );
    assert_eq!("".parse::<ProductIdRange>(), Err(ParseProductIdRangeError));
    assert_eq!("-".parse::<ProductIdRange>(), Err(ParseProductIdRangeError));
    assert_eq!(
        "11-".parse::<ProductIdRange>(),
        Err(ParseProductIdRangeError)
    );
    assert_eq!(
        "-22".parse::<ProductIdRange>(),
        Err(ParseProductIdRangeError)
    );
}

#[derive(Debug, PartialEq)]
pub struct ProductIdRange(u64, u64);

#[derive(Debug, PartialEq)]
pub struct ParseProductIdRangeError;

impl From<ParseIntError> for ParseProductIdRangeError {
    fn from(_value: ParseIntError) -> Self {
        Self
    }
}

impl FromStr for ProductIdRange {
    type Err = ParseProductIdRangeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once("-").ok_or(ParseProductIdRangeError)?;
        Ok(ProductIdRange(start.parse()?, end.parse()?))
    }
}

#[test]
fn we_can_parse_a_comma_separated_list_of_product_id_ranges() {
    assert_eq!(
        parse_comma_separated_list_of_product_id_ranges(""),
        Ok(vec![])
    );
    assert_eq!(
        parse_comma_separated_list_of_product_id_ranges("11-22"),
        Ok(vec![ProductIdRange(11, 22)])
    );
    assert_eq!(
        parse_comma_separated_list_of_product_id_ranges(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
        ),
        Ok(vec![
            ProductIdRange(11, 22),
            ProductIdRange(95, 115),
            ProductIdRange(998, 1012),
            ProductIdRange(1188511880, 1188511890),
            ProductIdRange(222220, 222224),
            ProductIdRange(1698522, 1698528),
            ProductIdRange(446443, 446449),
            ProductIdRange(38593856, 38593862),
            ProductIdRange(565653, 565659),
            ProductIdRange(824824821, 824824827),
            ProductIdRange(2121212118, 2121212124)
        ])
    );
}

pub fn parse_comma_separated_list_of_product_id_ranges(
    input: &str,
) -> Result<Vec<ProductIdRange>, ParseProductIdRangeError> {
    input
        .split_terminator(',')
        .map(ProductIdRange::from_str)
        .collect()
}

#[test]
fn we_can_detect_a_level_one_invalid_product_id() {
    assert!(!LevelOneProductIdValidator::is_valid(11));
    assert!(LevelOneProductIdValidator::is_valid(115));
    assert!(LevelOneProductIdValidator::is_valid(12));
    assert!(!LevelOneProductIdValidator::is_valid(99));
    assert!(!LevelOneProductIdValidator::is_valid(1010));
    assert!(!LevelOneProductIdValidator::is_valid(1188511885));
    assert!(!LevelOneProductIdValidator::is_valid(222222));
    assert!(LevelOneProductIdValidator::is_valid(13));
    assert!(LevelOneProductIdValidator::is_valid(1011));
    assert!(LevelOneProductIdValidator::is_valid(98));
    assert!(LevelOneProductIdValidator::is_valid(1885518856));
    assert!(LevelOneProductIdValidator::is_valid(222223));
    assert!(LevelOneProductIdValidator::is_valid(1));
}

pub trait ProductIdValidator {
    fn is_valid(product_id: u64) -> bool;
}

pub struct LevelOneProductIdValidator;

impl ProductIdValidator for LevelOneProductIdValidator {
    fn is_valid(product_id: u64) -> bool {
        let back_again = format!("{}", product_id);
        let (left, right) = back_again.split_at(back_again.len() / 2);
        left != right
    }
}

#[test]
#[rustfmt::skip]
fn we_can_sum_level_one_invalid_product_ids_in_a_range() {
    assert_eq!(ProductIdRange(11, 22).sum_invalid_product_ids::<LevelOneProductIdValidator>(), 33);
    assert_eq!(ProductIdRange(95, 115).sum_invalid_product_ids::<LevelOneProductIdValidator>(), 99);
    assert_eq!(ProductIdRange(998, 1012).sum_invalid_product_ids::<LevelOneProductIdValidator>(), 1010);
    assert_eq!(ProductIdRange(1188511880, 1188511890).sum_invalid_product_ids::<LevelOneProductIdValidator>(), 1188511885);
    assert_eq!(ProductIdRange(222220, 222224).sum_invalid_product_ids::<LevelOneProductIdValidator>(), 222222);
    assert_eq!(ProductIdRange(1698522, 1698528).sum_invalid_product_ids::<LevelOneProductIdValidator>(), 0);
    assert_eq!(ProductIdRange(446443, 446449).sum_invalid_product_ids::<LevelOneProductIdValidator>(), 446446);
    assert_eq!(ProductIdRange(38593856, 38593862).sum_invalid_product_ids::<LevelOneProductIdValidator>(), 38593859);
    assert_eq!(ProductIdRange(565653, 565659).sum_invalid_product_ids::<LevelOneProductIdValidator>(), 0);
    assert_eq!(ProductIdRange(824824821, 824824827).sum_invalid_product_ids::<LevelOneProductIdValidator>(), 0);
    assert_eq!(ProductIdRange(2121212118, 2121212124).sum_invalid_product_ids::<LevelOneProductIdValidator>(), 0);
}

impl ProductIdRange {
    pub fn sum_invalid_product_ids<V: ProductIdValidator>(&self) -> u64 {
        let mut sum = 0u64;
        for product_id in self.0..=self.1 {
            if !V::is_valid(product_id) {
                sum += product_id;
            }
        }
        sum
    }
}

#[test]
fn can_sum_all_level_one_invalid_product_ids_from_buf_read() {
    assert_eq!(
        sum_all_invalid_product_ids_from_input::<LevelOneProductIdValidator>(
            &mut std::io::Cursor::new("".as_bytes())
        ),
        0
    );
    assert_eq!(
        sum_all_invalid_product_ids_from_input::<LevelOneProductIdValidator>(
            &mut std::io::Cursor::new("11-22".as_bytes())
        ),
        33
    );

    assert_eq!(sum_all_invalid_product_ids_from_input::<LevelOneProductIdValidator>(&mut std::io::Cursor::new("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
        .as_bytes())), 1227775554);
}

pub fn sum_all_invalid_product_ids_from_input<V: ProductIdValidator>(
    input: &mut dyn std::io::Read,
) -> u64 {
    let mut all_input = String::new();
    input.read_to_string(&mut all_input).unwrap();
    let ranges = parse_comma_separated_list_of_product_id_ranges(all_input.trim())
        .expect("failed to parse product id ranges");
    ranges
        .iter()
        .map(ProductIdRange::sum_invalid_product_ids::<V>)
        .sum()
}

pub struct LevelTwoProductIdValidator;

impl ProductIdValidator for LevelTwoProductIdValidator {
    fn is_valid(product_id: u64) -> bool {
        let back_again = format!("{product_id}");

        let count = back_again.len();

        for i in 1..=count / 2 {
            let mut chunks = back_again.as_bytes().chunks(i);
            let first_chunk = chunks.next().unwrap();
            if chunks.all(|other| first_chunk == other) {
                return false;
            }
        }
        true
    }
}

#[test]
fn we_can_detect_a_level_two_invalid_product_ids() {
    assert!(!LevelTwoProductIdValidator::is_valid(11));
    assert!(!LevelTwoProductIdValidator::is_valid(22));
    assert!(LevelTwoProductIdValidator::is_valid(1));
    assert!(LevelTwoProductIdValidator::is_valid(115));
    assert!(LevelTwoProductIdValidator::is_valid(12));
    assert!(!LevelTwoProductIdValidator::is_valid(99));
    assert!(!LevelTwoProductIdValidator::is_valid(1010));
    assert!(!LevelTwoProductIdValidator::is_valid(1188511885));
    assert!(!LevelTwoProductIdValidator::is_valid(222222));
    assert!(LevelTwoProductIdValidator::is_valid(13));
    assert!(LevelTwoProductIdValidator::is_valid(1011));
    assert!(LevelTwoProductIdValidator::is_valid(98));
    assert!(LevelTwoProductIdValidator::is_valid(1885518856));
    assert!(LevelTwoProductIdValidator::is_valid(222223));
    assert!(LevelTwoProductIdValidator::is_valid(1));
    assert!(!LevelTwoProductIdValidator::is_valid(333));
    assert!(!LevelTwoProductIdValidator::is_valid(123123123123123));
}

#[test]
#[rustfmt::skip]
fn we_can_sum_level_two_invalid_product_ids_in_a_range() {
    assert_eq!(ProductIdRange(11, 22).sum_invalid_product_ids::<LevelTwoProductIdValidator>(), 33);
    assert_eq!(ProductIdRange(95, 115).sum_invalid_product_ids::<LevelTwoProductIdValidator>(), 99+111);
    assert_eq!(ProductIdRange(998, 1012).sum_invalid_product_ids::<LevelTwoProductIdValidator>(), 999+1010);
    assert_eq!(ProductIdRange(1188511880, 1188511890).sum_invalid_product_ids::<LevelTwoProductIdValidator>(), 1188511885);
    assert_eq!(ProductIdRange(222220, 222224).sum_invalid_product_ids::<LevelTwoProductIdValidator>(), 222222);
    assert_eq!(ProductIdRange(1698522, 1698528).sum_invalid_product_ids::<LevelTwoProductIdValidator>(), 0);
    assert_eq!(ProductIdRange(446443, 446449).sum_invalid_product_ids::<LevelTwoProductIdValidator>(), 446446);
    assert_eq!(ProductIdRange(38593856, 38593862).sum_invalid_product_ids::<LevelTwoProductIdValidator>(), 38593859);
    assert_eq!(ProductIdRange(565653, 565659).sum_invalid_product_ids::<LevelTwoProductIdValidator>(), 565656);
    assert_eq!(ProductIdRange(824824821, 824824827).sum_invalid_product_ids::<LevelTwoProductIdValidator>(), 824824824);
    assert_eq!(ProductIdRange(2121212118, 2121212124).sum_invalid_product_ids::<LevelTwoProductIdValidator>(), 2121212121);
}

#[test]
fn can_sum_all_level_two_invalid_product_ids_from_buf_read() {
    assert_eq!(
        sum_all_invalid_product_ids_from_input::<LevelTwoProductIdValidator>(
            &mut std::io::Cursor::new("".as_bytes())
        ),
        0
    );
    assert_eq!(
        sum_all_invalid_product_ids_from_input::<LevelTwoProductIdValidator>(
            &mut std::io::Cursor::new("11-22".as_bytes())
        ),
        33
    );

    assert_eq!(sum_all_invalid_product_ids_from_input::<LevelTwoProductIdValidator>(&mut std::io::Cursor::new("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
        .as_bytes())), 4174379265);
}
