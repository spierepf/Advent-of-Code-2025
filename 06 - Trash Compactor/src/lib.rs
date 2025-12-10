use std::str::FromStr;

pub const SAMPLE_INPUT: &str = "123 328  51 64\n 45 64  387 23\n  6 98  215 314\n*   +   *   +  ";

#[derive(Debug, PartialEq)]
pub enum Operator {
    Product,
    Sum,
}

#[derive(Debug, PartialEq)]
pub struct ParseOperatorError;
impl FromStr for Operator {
    type Err = ParseOperatorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Sum),
            "*" => Ok(Operator::Product),
            _ => Err(ParseOperatorError),
        }
    }
}

#[test]
fn operator_can_be_parsed_from_a_str() {
    assert_eq!("+".parse::<Operator>(), Ok(Operator::Sum));
    assert_eq!("*".parse::<Operator>(), Ok(Operator::Product));
    assert_eq!("1".parse::<Operator>(), Err(ParseOperatorError));
}

#[derive(Debug, PartialEq)]
pub struct Input {
    pub rows: Vec<Vec<u64>>,
    pub operators: Vec<Operator>,
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let last_line = lines.next_back().unwrap();
        Ok(Self {
            rows: lines
                .map(|l| {
                    l.split(' ')
                        .filter(|col| !col.is_empty())
                        .map(|col| col.parse().unwrap())
                        .collect()
                })
                .collect(),
            operators: last_line
                .split(' ')
                .filter(|col| !col.is_empty())
                .map(|col| col.parse::<Operator>().unwrap())
                .collect(),
        })
    }
}

#[test]
fn we_can_read_the_input() {
    assert_eq!(
        "1\n*".parse::<Input>(),
        Ok(Input {
            rows: vec![vec![1]],
            operators: vec![Operator::Product]
        })
    );
    assert_eq!(
        "1\n+".parse::<Input>(),
        Ok(Input {
            rows: vec![vec![1]],
            operators: vec![Operator::Sum]
        })
    );
    assert_eq!(
        "2\n+".parse::<Input>(),
        Ok(Input {
            rows: vec![vec![2]],
            operators: vec![Operator::Sum]
        })
    );
    assert_eq!(
        "1\n2\n+".parse::<Input>(),
        Ok(Input {
            rows: vec![vec![1], vec![2]],
            operators: vec![Operator::Sum]
        })
    );
    assert_eq!(
        "1 3\n2 4\n+ *".parse::<Input>(),
        Ok(Input {
            rows: vec![vec![1, 3], vec![2, 4]],
            operators: vec![Operator::Sum, Operator::Product]
        })
    );
    assert_eq!(
        "1  3\n2 4\n+ *".parse::<Input>(),
        Ok(Input {
            rows: vec![vec![1, 3], vec![2, 4]],
            operators: vec![Operator::Sum, Operator::Product]
        })
    );
    assert_eq!(
        "1  3\n2 4\n +   *".parse::<Input>(),
        Ok(Input {
            rows: vec![vec![1, 3], vec![2, 4]],
            operators: vec![Operator::Sum, Operator::Product]
        })
    );
    assert_eq!(
        SAMPLE_INPUT.parse(),
        Ok(Input {
            rows: vec![
                vec![123, 328, 51, 64],
                vec![45, 64, 387, 23],
                vec![6, 98, 215, 314],
            ],
            operators: vec![
                Operator::Product,
                Operator::Sum,
                Operator::Product,
                Operator::Sum
            ]
        })
    );
}

impl Input {
    fn do_one_column_of_homework_1(&self, col_index: usize) -> u64 {
        let iter = self.rows.iter().map(|r| r[col_index]);
        match self.operators[col_index] {
            Operator::Product => iter.product(),
            Operator::Sum => iter.sum(),
        }
    }
    pub fn do_homework_1(&self) -> u64 {
        (0..self.operators.len())
            .map(|col_index| self.do_one_column_of_homework_1(col_index))
            .sum()
    }
}
#[test]
fn input_can_do_cephalopod_math() {
    assert_eq!("1\n+".parse::<Input>().unwrap().do_homework_1(), 1);
    assert_eq!("2\n*".parse::<Input>().unwrap().do_homework_1(), 2);
    assert_eq!("1\n1\n+".parse::<Input>().unwrap().do_homework_1(), 2);
    assert_eq!("1\n2\n*".parse::<Input>().unwrap().do_homework_1(), 2);
    assert_eq!("1 1\n1 1\n+ +".parse::<Input>().unwrap().do_homework_1(), 4);
    assert_eq!(
        SAMPLE_INPUT.parse::<Input>().unwrap().do_homework_1(),
        4277556
    );
}
