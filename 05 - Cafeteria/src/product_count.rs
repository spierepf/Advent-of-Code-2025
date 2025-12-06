use std::fmt::Display;
use std::ops::Add;

#[derive(Debug, PartialEq, Default)]
pub struct ProductCount(pub u64);

impl Add<Self> for ProductCount {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Display for ProductCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[test]
fn can_add_product_counts() {
    assert_eq!(ProductCount(0) + ProductCount(0), ProductCount(0));
    assert_eq!(ProductCount(0) + ProductCount(1), ProductCount(1));
    assert_eq!(ProductCount(1) + ProductCount(0), ProductCount(1));
    assert_eq!(ProductCount(3) + ProductCount(5), ProductCount(8));
}
