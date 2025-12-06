use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ProductId(pub u64);

#[test]
fn test_can_compare_two_product_ids() {
    assert_eq!(ProductId(0), ProductId(0));
    assert_ne!(ProductId(0), ProductId(1));
    assert!(ProductId(0) < ProductId(1));
    assert!(ProductId(1) > ProductId(0));
    assert!(ProductId(0) <= ProductId(1));
    assert!(ProductId(1) >= ProductId(0));
}

impl FromStr for ProductId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ProductId(s.parse()?))
    }
}

#[test]
fn test_can_parse_a_product_id_from_str() {
    assert_eq!("0".parse::<ProductId>(), Ok(ProductId(0)));
    assert_eq!("3".parse::<ProductId>(), Ok(ProductId(3)));
    assert_eq!("5".parse::<ProductId>(), Ok(ProductId(5)));
    assert_eq!("10".parse::<ProductId>(), Ok(ProductId(10)));
    assert_eq!("14".parse::<ProductId>(), Ok(ProductId(14)));
    assert_eq!("16".parse::<ProductId>(), Ok(ProductId(16)));
    assert!("".parse::<ProductId>().is_err());
    assert!("q".parse::<ProductId>().is_err());
    assert!("1q".parse::<ProductId>().is_err());
    assert!("-2".parse::<ProductId>().is_err());
}
