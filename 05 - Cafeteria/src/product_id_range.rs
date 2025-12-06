use crate::ProductId;
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct ProductIdRange(RangeInclusive<ProductId>);

impl ProductIdRange {
    pub fn new(start: ProductId, end: ProductId) -> ProductIdRange {
        ProductIdRange(start..=end)
    }

    pub fn contains(&self, id: ProductId) -> bool {
        self.0.contains(&id)
    }
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
