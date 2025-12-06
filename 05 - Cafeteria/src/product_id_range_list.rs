use crate::ProductIdRange;
use crate::product_id_range::ParseProductIdRangeError;
use crate::{ProductCount, ProductId};
use std::str::FromStr;

#[derive(Debug, PartialEq, Default)]
pub struct ProductIdRangeList {
    ranges: Vec<ProductIdRange>,
}

impl ProductIdRangeList {
    pub fn contains(&self, id: ProductId) -> bool {
        self.ranges.iter().any(|r| r.contains(id))
    }

    pub fn total_count(&self) -> ProductCount {
        self.ranges
            .iter()
            .fold(ProductCount::default(), |partial_sum, range| {
                partial_sum + range.count()
            })
    }
}

impl FromIterator<ProductIdRange> for ProductIdRangeList {
    fn from_iter<T: IntoIterator<Item = ProductIdRange>>(iter: T) -> Self {
        let mut ranges: Vec<ProductIdRange> = vec![];

        for new_product_id_range in iter {
            let (overlapping, mut non_overlapping) = ranges
                .iter()
                .cloned()
                .partition::<Vec<_>, _>(|r| r.overlaps(&new_product_id_range));
            let combined = overlapping
                .iter()
                .fold(new_product_id_range, |r1, r2| r1.merge(r2));

            non_overlapping.push(combined);
            ranges = non_overlapping;
        }

        Self { ranges }
    }
}

#[test]
fn a_product_id_range_list_can_count_how_many_ids_are_in_its_ranges() {
    assert_eq!(ProductIdRangeList::default().total_count(), ProductCount(0));
    assert_eq!(
        ProductIdRangeList::from_str("3-5").unwrap().total_count(),
        ProductCount(3)
    );
    assert_eq!(
        ProductIdRangeList::from_str("3-5\n10-14\n16-20")
            .unwrap()
            .total_count(),
        ProductCount(13)
    );
    assert_eq!(
        ProductIdRangeList::from_str("3-5\n10-14\n16-20\n12-18")
            .unwrap()
            .total_count(),
        ProductCount(14)
    );
}

#[derive(Debug, PartialEq)]
pub struct ParseProductIdRangeListError;

impl From<ParseProductIdRangeError> for ParseProductIdRangeListError {
    fn from(_: ParseProductIdRangeError) -> Self {
        Self
    }
}

impl FromStr for ProductIdRangeList {
    type Err = ParseProductIdRangeListError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(<ProductIdRange>::from_str)
            .map(|res| res.map_err(|_| ParseProductIdRangeListError))
            .collect::<Result<Self, _>>()
    }
}

#[test]
fn test_parse_a_product_id_range_list_from_a_string() {
    assert_eq!(
        "".parse::<ProductIdRangeList>(),
        Ok(ProductIdRangeList::default())
    );
    assert_eq!(
        "-".parse::<ProductIdRangeList>(),
        Err(ParseProductIdRangeListError)
    );
    assert_eq!(
        "3-5".parse::<ProductIdRangeList>(),
        Ok(ProductIdRangeList::from_iter(["3-5"
            .parse::<ProductIdRange>()
            .unwrap()]))
    );
    assert_ne!(
        "3-5".parse::<ProductIdRangeList>(),
        Ok(ProductIdRangeList::from_iter(["10-14"
            .parse::<ProductIdRange>()
            .unwrap()]))
    );
    assert_eq!(
        "10-14".parse::<ProductIdRangeList>(),
        Ok(ProductIdRangeList::from_iter(["10-14"
            .parse::<ProductIdRange>()
            .unwrap()]))
    );
    assert_ne!(
        "3-5".parse::<ProductIdRangeList>(),
        Ok(ProductIdRangeList::default())
    );
    assert_eq!(
        "3-5\n10-14\n".parse::<ProductIdRangeList>(),
        Ok(ProductIdRangeList::from_iter([
            "3-5".parse::<ProductIdRange>().unwrap(),
            "10-14".parse::<ProductIdRange>().unwrap()
        ]))
    );
    assert_eq!(
        "3-5\n10-14\n16-20\n12-18\n".parse::<ProductIdRangeList>(),
        Ok(ProductIdRangeList::from_iter([
            "3-5".parse::<ProductIdRange>().unwrap(),
            "10-14".parse::<ProductIdRange>().unwrap(),
            "16-20".parse::<ProductIdRange>().unwrap(),
            "12-18".parse::<ProductIdRange>().unwrap()
        ]))
    );
}

#[test]
fn test_a_product_id_range_list_can_tell_us_if_a_product_id_is_contained_in_one_of_its_ranges() {
    assert!(!ProductIdRangeList::default().contains(ProductId(1)));
    assert!(
        "3-5"
            .parse::<ProductIdRangeList>()
            .unwrap()
            .contains(ProductId(4))
    );
    assert!(
        !"3-5\n10-14\n16-20\n12-18"
            .parse::<ProductIdRangeList>()
            .unwrap()
            .contains(ProductId(1))
    );
    assert!(
        "3-5\n10-14\n16-20\n12-18"
            .parse::<ProductIdRangeList>()
            .unwrap()
            .contains(ProductId(5))
    );
    assert!(
        !"3-5\n10-14\n16-20\n12-18"
            .parse::<ProductIdRangeList>()
            .unwrap()
            .contains(ProductId(8))
    );
    assert!(
        "3-5\n10-14\n16-20\n12-18"
            .parse::<ProductIdRangeList>()
            .unwrap()
            .contains(ProductId(11))
    );
    assert!(
        "3-5\n10-14\n16-20\n12-18"
            .parse::<ProductIdRangeList>()
            .unwrap()
            .contains(ProductId(17))
    );
    assert!(
        !"3-5\n10-14\n16-20\n12-18"
            .parse::<ProductIdRangeList>()
            .unwrap()
            .contains(ProductId(32))
    );
}
