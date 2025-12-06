use crate::product_id_list::ParseProductIdListError;
use crate::product_id_range_list::ParseProductIdRangeListError;
use crate::{ProductIdList, ProductIdRangeList};
use std::str::FromStr;

#[derive(Debug, Default, PartialEq)]
pub struct Input {
    fresh_product_ranges: ProductIdRangeList,
    available_products: ProductIdList,
}

impl Input {
    pub fn count_fresh_products(self) -> usize {
        self.available_products
            .into_iter()
            .filter(|&product_id| self.fresh_product_ranges.contains(product_id))
            .count()
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseInputError;

impl From<ParseProductIdRangeListError> for ParseInputError {
    fn from(_: ParseProductIdRangeListError) -> Self {
        Self
    }
}

impl From<ParseProductIdListError> for ParseInputError {
    fn from(_: ParseProductIdListError) -> Self {
        Self
    }
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (fresh_product_ranges, available_products) =
            s.split_once("\n\n").ok_or(ParseInputError)?;
        Ok(Self {
            fresh_product_ranges: fresh_product_ranges.parse()?,
            available_products: available_products.parse()?,
        })
    }
}

#[test]
fn we_can_read_our_input_from_a_buf_read() {
    assert_eq!(Input::from_str("\n\n"), Ok(Input::default()));
    assert_eq!(
        Input::from_str("3-5\n\n"),
        Ok(Input {
            fresh_product_ranges: "3-5".parse().unwrap(),
            available_products: ProductIdList::default()
        })
    );
    assert_eq!(
        Input::from_str("\n\n1\n"),
        Ok(Input {
            fresh_product_ranges: Default::default(),
            available_products: "1".parse().unwrap()
        })
    );
    assert_eq!(
        Input::from_str("3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32\n"),
        Ok(Input {
            fresh_product_ranges: "3-5\n10-14\n16-20\n12-18".parse().unwrap(),
            available_products: "1\n5\n8\n11\n17\n32\n".parse().unwrap()
        })
    );
    assert_eq!(
        Input::from_str("3\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32\n"),
        Err(ParseInputError)
    );
    assert_eq!(
        Input::from_str("3-5\n10-14\n16-20\n12-18\n\n1\nQ\n8\n11\n17\n32\n"),
        Err(ParseInputError)
    );
}

#[test]
fn input_can_count_the_fresh_products() {
    assert_eq!(Input::default().count_fresh_products(), 0);
    assert_eq!(
        Input::from_str("3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32\n")
            .unwrap()
            .count_fresh_products(),
        3
    );
}
