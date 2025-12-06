use crate::ProductId;
use std::str::FromStr;

#[derive(PartialEq, Debug, Default)]
pub struct ProductIdList {
    product_ids: Vec<ProductId>,
}

#[derive(Debug, PartialEq)]
pub struct ParseProductIdListError;

impl From<std::num::ParseIntError> for ParseProductIdListError {
    fn from(_: std::num::ParseIntError) -> Self {
        Self
    }
}

impl FromStr for ProductIdList {
    type Err = ParseProductIdListError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            product_ids: s
                .lines()
                .map(<ProductId as FromStr>::from_str)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl FromIterator<ProductId> for ProductIdList {
    fn from_iter<T: IntoIterator<Item = ProductId>>(iter: T) -> Self {
        Self {
            product_ids: Vec::<ProductId>::from_iter(iter),
        }
    }
}

impl IntoIterator for ProductIdList {
    type Item = ProductId;
    type IntoIter = std::vec::IntoIter<ProductId>;
    fn into_iter(self) -> Self::IntoIter {
        self.product_ids.into_iter()
    }
}

#[test]
fn test_we_can_parse_a_product_id_list_from_a_str() {
    assert_eq!("".parse::<ProductIdList>(), Ok(ProductIdList::default()));
    assert_ne!("1".parse::<ProductIdList>(), Ok(ProductIdList::default()));
    assert_eq!(
        "1".parse::<ProductIdList>(),
        Ok(ProductIdList::from_iter([ProductId(1)]))
    );
    assert_eq!(
        "1\n5\n8\n11\n17\n32\n".parse::<ProductIdList>(),
        Ok(ProductIdList::from_iter([
            ProductId(1),
            ProductId(5),
            ProductId(8),
            ProductId(11),
            ProductId(17),
            ProductId(32),
        ]))
    );
}

#[test]
fn we_can_iterate_product_ids_from_a_product_id_list() {
    assert_eq!(
        "".parse::<ProductIdList>()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        vec![]
    );
    assert_eq!(
        "1".parse::<ProductIdList>()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        vec![ProductId(1)]
    );
    assert_eq!(
        "1\n5\n8\n11\n17\n32\n"
            .parse::<ProductIdList>()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        vec![
            ProductId(1),
            ProductId(5),
            ProductId(8),
            ProductId(11),
            ProductId(17),
            ProductId(32),
        ]
    );
}
