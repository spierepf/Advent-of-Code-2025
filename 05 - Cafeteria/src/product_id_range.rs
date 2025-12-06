use crate::ProductCount;
use crate::ProductId;
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct ProductIdRange(RangeInclusive<ProductId>);

impl ProductIdRange {
    pub fn new(start: ProductId, end: ProductId) -> ProductIdRange {
        ProductIdRange(start..=end)
    }

    pub fn contains(&self, id: ProductId) -> bool {
        self.0.contains(&id)
    }

    pub fn count(&self) -> ProductCount {
        self.0
            .start()
            .count_product_ids_up_to_and_including(*self.0.end())
    }

    pub fn overlaps(&self, other: &ProductIdRange) -> bool {
        self.0.end() >= other.0.start() && self.0.start() <= other.0.end()
    }

    pub fn merge(&self, other: &Self) -> Self {
        let new_start = std::cmp::min(*self.0.start(), *other.0.start());
        let new_end = std::cmp::max(*self.0.end(), *other.0.end());

        Self(new_start..=new_end)
    }
}

#[test]
fn we_can_detect_overlapping_product_id_ranges() {
    assert!(
        "3-5"
            .parse::<ProductIdRange>()
            .unwrap()
            .overlaps(&"4-7".parse::<ProductIdRange>().unwrap())
    );
    assert!(
        !"3-5"
            .parse::<ProductIdRange>()
            .unwrap()
            .overlaps(&"7-9".parse::<ProductIdRange>().unwrap())
    );
    assert!(
        !"7-9"
            .parse::<ProductIdRange>()
            .unwrap()
            .overlaps(&"3-5".parse::<ProductIdRange>().unwrap())
    );
    assert!(
        "3-5"
            .parse::<ProductIdRange>()
            .unwrap()
            .overlaps(&"5-7".parse::<ProductIdRange>().unwrap())
    );
    assert!(
        "3-5"
            .parse::<ProductIdRange>()
            .unwrap()
            .overlaps(&"1-3".parse::<ProductIdRange>().unwrap())
    );
    assert!(
        "3-5"
            .parse::<ProductIdRange>()
            .unwrap()
            .overlaps(&"1-4".parse::<ProductIdRange>().unwrap())
    );
    assert!(
        "1-5"
            .parse::<ProductIdRange>()
            .unwrap()
            .overlaps(&"2-4".parse::<ProductIdRange>().unwrap())
    );
    assert!(
        "2-4"
            .parse::<ProductIdRange>()
            .unwrap()
            .overlaps(&"1-5".parse::<ProductIdRange>().unwrap())
    );
}

#[test]
fn we_can_merge_two_product_id_ranges() {
    assert_eq!(
        "0-0"
            .parse::<ProductIdRange>()
            .unwrap()
            .merge(&"0-0".parse::<ProductIdRange>().unwrap()),
        "0-0".parse::<ProductIdRange>().unwrap()
    );
    assert_eq!(
        "0-0"
            .parse::<ProductIdRange>()
            .unwrap()
            .merge(&"0-1".parse::<ProductIdRange>().unwrap()),
        "0-1".parse::<ProductIdRange>().unwrap()
    );
    assert_eq!(
        "0-1"
            .parse::<ProductIdRange>()
            .unwrap()
            .merge(&"0-0".parse::<ProductIdRange>().unwrap()),
        "0-1".parse::<ProductIdRange>().unwrap()
    );
    assert_eq!(
        "0-1"
            .parse::<ProductIdRange>()
            .unwrap()
            .merge(&"1-1".parse::<ProductIdRange>().unwrap()),
        "0-1".parse::<ProductIdRange>().unwrap()
    );
    assert_eq!(
        "10-14"
            .parse::<ProductIdRange>()
            .unwrap()
            .merge(&"12-18".parse::<ProductIdRange>().unwrap()),
        "10-18".parse::<ProductIdRange>().unwrap()
    );
    assert_eq!(
        "16-20"
            .parse::<ProductIdRange>()
            .unwrap()
            .merge(&"12-18".parse::<ProductIdRange>().unwrap()),
        "12-20".parse::<ProductIdRange>().unwrap()
    );
    assert_eq!(
        "12-18"
            .parse::<ProductIdRange>()
            .unwrap()
            .merge(&"16-20".parse::<ProductIdRange>().unwrap()),
        "12-20".parse::<ProductIdRange>().unwrap()
    );
}

#[derive(Debug, PartialEq)]
pub struct ParseProductIdRangeError;

impl From<ParseIntError> for ParseProductIdRangeError {
    fn from(_: ParseIntError) -> ParseProductIdRangeError {
        Self
    }
}

impl FromStr for ProductIdRange {
    type Err = ParseProductIdRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or(ParseProductIdRangeError)?;
        let start = start.parse()?;
        let end = end.parse()?;
        if start > end {
            return Err(ParseProductIdRangeError);
        }
        Ok(ProductIdRange::new(start, end))
    }
}

#[test]
fn test_can_parse_product_id_range_from_str() {
    assert_eq!(
        "3-5".parse::<ProductIdRange>(),
        Ok(ProductIdRange::new(ProductId(3), ProductId(5)))
    );
    assert_eq!(
        "10-14".parse::<ProductIdRange>(),
        Ok(ProductIdRange::new(ProductId(10), ProductId(14)))
    );
    assert_eq!(
        "16-20".parse::<ProductIdRange>(),
        Ok(ProductIdRange::new(ProductId(16), ProductId(20)))
    );
    assert_eq!(
        "12-18".parse::<ProductIdRange>(),
        Ok(ProductIdRange::new(ProductId(12), ProductId(18)))
    );
    assert_eq!("".parse::<ProductIdRange>(), Err(ParseProductIdRangeError));
    assert_eq!(
        "12".parse::<ProductIdRange>(),
        Err(ParseProductIdRangeError)
    );
    assert_eq!("-".parse::<ProductIdRange>(), Err(ParseProductIdRangeError));
    assert_eq!(
        "12-".parse::<ProductIdRange>(),
        Err(ParseProductIdRangeError)
    );
    assert_eq!(
        "-12".parse::<ProductIdRange>(),
        Err(ParseProductIdRangeError)
    );
    assert_eq!(
        "12--12".parse::<ProductIdRange>(),
        Err(ParseProductIdRangeError)
    );
    assert_eq!(
        "2-1".parse::<ProductIdRange>(),
        Err(ParseProductIdRangeError)
    );
}

#[test]
fn test_product_id_range_can_determine_if_it_contains_a_specified_product_id() {
    assert!(
        "3-5"
            .parse::<ProductIdRange>()
            .unwrap()
            .contains(ProductId(3))
    );
    assert!(
        "3-5"
            .parse::<ProductIdRange>()
            .unwrap()
            .contains(ProductId(4))
    );
    assert!(
        "3-5"
            .parse::<ProductIdRange>()
            .unwrap()
            .contains(ProductId(5))
    );
    assert!(
        !"3-5"
            .parse::<ProductIdRange>()
            .unwrap()
            .contains(ProductId(2))
    );
    assert!(
        !"3-5"
            .parse::<ProductIdRange>()
            .unwrap()
            .contains(ProductId(6))
    );
}

#[test]
fn we_can_count_the_ids_in_a_product_id_range() {
    assert_eq!(
        "1-1".parse::<ProductIdRange>().unwrap().count(),
        ProductCount(1)
    );
    assert_eq!(
        "3-5".parse::<ProductIdRange>().unwrap().count(),
        ProductCount(3)
    );
    assert_eq!(
        "10-14".parse::<ProductIdRange>().unwrap().count(),
        ProductCount(5)
    );
    assert_eq!(
        "16-20".parse::<ProductIdRange>().unwrap().count(),
        ProductCount(5)
    );
    assert_eq!(
        "12-18".parse::<ProductIdRange>().unwrap().count(),
        ProductCount(7)
    );
}
