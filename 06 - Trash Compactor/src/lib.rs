use std::str::FromStr;

pub const SAMPLE_INPUT: &str = "123 328  51 64\n 45 64  387 23\n  6 98  215 314\n*   +   *   +  ";

#[derive(Debug, PartialEq)]
pub enum Operator {
    Product,
    Sum,
}

impl FromStr for Operator {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Sum),
            "*" => Ok(Operator::Product),
            _ => anyhow::bail!("Unknown operator: {}", s),
        }
    }
}

#[test]
fn operator_can_be_parsed_from_a_str() {
    assert_eq!("+".parse::<Operator>().unwrap(), Operator::Sum);
    assert_eq!("*".parse::<Operator>().unwrap(), Operator::Product);
    assert!("1".parse::<Operator>().is_err());
}

#[derive(Debug, PartialEq)]
pub struct Problem {
    operands: Vec<u64>,
    operator: Operator,
}

#[derive(Debug, PartialEq)]
pub struct Homework {
    problems: Vec<Problem>,
}

pub trait Transpose: Sized {
    fn transpose(self) -> Self;
}

impl<ElementType: Clone> Transpose for Vec<Vec<ElementType>> {
    fn transpose(self) -> Vec<Vec<ElementType>> {
        let inner_input_len = self.iter().map(|row| row.len()).max().unwrap_or_default();
        let mut result = vec![];
        for i in 0..inner_input_len {
            let mut tmp = vec![];
            for row in &self {
                tmp.push(row[i].clone());
            }
            result.push(tmp);
        }
        result
    }
}

#[test]
fn test_2d_array_transposition() {
    assert_eq!(Vec::<Vec<char>>::new().transpose(), Vec::<Vec<char>>::new());
    assert_eq!(vec![vec![1, 2]].transpose(), vec![vec![1], vec![2]]);
    assert_eq!(vec![vec![1], vec![2]].transpose(), vec![vec![1, 2]]);
    assert_eq!(
        vec![vec![1, 2], vec![3, 4]].transpose(),
        vec![vec![1, 3], vec![2, 4]]
    );
    assert_eq!(
        vec![vec![1, 2], vec![3, 4]].transpose(),
        vec![vec![1, 3], vec![2, 4]]
    );
}

impl Homework {
    pub fn parse_v1(s: &str) -> anyhow::Result<Self> {
        let mut lines = s.lines();
        let last_line = lines.next_back().unwrap();
        let operators: Vec<Operator> = last_line
            .split(' ')
            .filter(|col| !col.is_empty())
            .map(|col| col.parse::<Operator>())
            .collect::<Result<Vec<Operator>, _>>()?;
        Ok(Self {
            problems: lines
                .map(|l| {
                    l.split(' ')
                        .filter(|col| !col.is_empty())
                        .map(|col| col.parse())
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<Vec<u64>>, _>>()?
                .transpose()
                .into_iter()
                .zip(operators)
                .map(|(col, op)| Problem {
                    operands: col,
                    operator: op,
                })
                .collect(),
        })
    }
}
#[test]
fn we_can_parse_an_input_v1() {
    assert_eq!(
        Homework::parse_v1("1\n+").unwrap(),
        Homework {
            problems: vec![Problem {
                operands: vec![1],
                operator: Operator::Sum
            }]
        }
    );

    assert_eq!(
        Homework::parse_v1("2\n+").unwrap(),
        Homework {
            problems: vec![Problem {
                operands: vec![2],
                operator: Operator::Sum
            }]
        }
    );
    assert_eq!(
        Homework::parse_v1("2\n*").unwrap(),
        Homework {
            problems: vec![Problem {
                operands: vec![2],
                operator: Operator::Product
            }]
        }
    );
    assert!(Homework::parse_v1("2\n!").is_err());
    assert_eq!(
        Homework::parse_v1("2\n3\n*").unwrap(),
        Homework {
            problems: vec![Problem {
                operands: vec![2, 3],
                operator: Operator::Product
            }]
        }
    );
    assert!(Homework::parse_v1("2\nz\n*").is_err());
    assert_eq!(
        Homework::parse_v1("1 2\n+ *").unwrap(),
        Homework {
            problems: vec![
                Problem {
                    operands: vec![1],
                    operator: Operator::Sum
                },
                Problem {
                    operands: vec![2],
                    operator: Operator::Product
                },
            ]
        }
    );
    assert_eq!(
        Homework::parse_v1(SAMPLE_INPUT).unwrap(),
        Homework {
            problems: vec![
                Problem {
                    operands: vec![123, 45, 6],
                    operator: Operator::Product,
                },
                Problem {
                    operands: vec![328, 64, 98],
                    operator: Operator::Sum,
                },
                Problem {
                    operands: vec![51, 387, 215],
                    operator: Operator::Product,
                },
                Problem {
                    operands: vec![64, 23, 314],
                    operator: Operator::Sum,
                },
            ]
        }
    );
}

impl Operator {
    fn operate(&self, operands: &[u64]) -> u64 {
        match self {
            Operator::Product => operands.iter().product(),
            Operator::Sum => operands.iter().sum(),
        }
    }
}
impl Problem {
    pub fn solve(&self) -> u64 {
        self.operator.operate(&self.operands)
    }
}

#[test]
fn problem_can_give_us_its_solution() {
    assert_eq!(
        Problem {
            operands: vec![1],
            operator: Operator::Product,
        }
        .solve(),
        1
    );

    assert_eq!(
        Problem {
            operands: vec![2],
            operator: Operator::Product,
        }
        .solve(),
        2
    );

    assert_eq!(
        Problem {
            operands: vec![2, 2],
            operator: Operator::Product,
        }
        .solve(),
        4
    );

    assert_eq!(
        Problem {
            operands: vec![2, 3],
            operator: Operator::Sum,
        }
        .solve(),
        5
    );

    assert_eq!(
        Problem {
            operands: vec![6, 9],
            operator: Operator::Sum,
        }
        .solve(),
        15
    );

    assert_eq!(
        Problem {
            operands: vec![3, 6],
            operator: Operator::Product,
        }
        .solve(),
        18
    );
}

impl Homework {
    pub fn sum_of_problems(&self) -> u64 {
        self.problems.iter().map(|p| p.solve()).sum()
    }
}

#[test]
fn we_can_do_our_homework() {
    assert_eq!(
        Homework {
            problems: vec![
                Problem {
                    operands: vec![123, 45, 6],
                    operator: Operator::Product,
                },
                Problem {
                    operands: vec![328, 64, 98],
                    operator: Operator::Sum,
                },
                Problem {
                    operands: vec![51, 387, 215],
                    operator: Operator::Product,
                },
                Problem {
                    operands: vec![64, 23, 314],
                    operator: Operator::Sum,
                },
            ]
        }
        .sum_of_problems(),
        4277556
    );
}

impl Homework {
    pub fn parse_v2(s: &str) -> anyhow::Result<Self> {
        let mut lines = s.lines();
        let operator_line = lines.next_back().unwrap();

        let operators = operator_line
            .split(' ')
            .filter(|col| !col.is_empty())
            .map(|col| col.parse::<Operator>().unwrap());

        let mut operand_rows = lines
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();
        let max_len = operand_rows
            .iter()
            .map(|r| r.len())
            .max()
            .unwrap_or_default();

        for operand_row in &mut operand_rows {
            operand_row.resize_with(max_len, || ' ');
        }

        let operand_cols = operand_rows.transpose();

        let operand_strings_all_together = operand_cols
            .into_iter()
            .map(|c| c.into_iter().collect::<String>())
            .collect::<Vec<String>>();
        let problems = operand_strings_all_together
            .split(|x| x.trim().is_empty())
            .map(|x| {
                x.iter()
                    .map(|o| o.trim().parse().unwrap())
                    .rev()
                    .collect::<Vec<u64>>()
            })
            .rev()
            .zip(operators.rev())
            .map(|(operands, operator)| Problem { operands, operator })
            .collect::<Vec<_>>();

        Ok(Homework { problems })
    }
}

#[test]
fn we_can_parse_an_input_v2() {
    assert_eq!(
        Homework::parse_v2("2\n3\n*").unwrap(),
        Homework {
            problems: vec![Problem {
                operands: vec![23],
                operator: Operator::Product
            }]
        }
    );

    assert_eq!(
        Homework::parse_v2(SAMPLE_INPUT).unwrap(),
        Homework {
            problems: vec![
                Problem {
                    operands: vec![4, 431, 623],
                    operator: Operator::Sum,
                },
                Problem {
                    operands: vec![175, 581, 32],
                    operator: Operator::Product,
                },
                Problem {
                    operands: vec![8, 248, 369],
                    operator: Operator::Sum,
                },
                Problem {
                    operands: vec![356, 24, 1],
                    operator: Operator::Product,
                },
            ]
        }
    );

    assert_eq!(
        Homework::parse_v2("1\n+").unwrap(),
        Homework {
            problems: vec![Problem {
                operands: vec![1],
                operator: Operator::Sum
            }]
        }
    );

    assert_eq!(
        Homework::parse_v2("2\n+").unwrap(),
        Homework {
            problems: vec![Problem {
                operands: vec![2],
                operator: Operator::Sum
            }]
        }
    );
    assert_eq!(
        Homework::parse_v2("2\n*").unwrap(),
        Homework {
            problems: vec![Problem {
                operands: vec![2],
                operator: Operator::Product
            }]
        }
    );
    assert_eq!(
        Homework::parse_v2("1 2\n+ *").unwrap(),
        Homework {
            problems: vec![
                Problem {
                    operands: vec![2],
                    operator: Operator::Product
                },
                Problem {
                    operands: vec![1],
                    operator: Operator::Sum
                },
            ]
        }
    );
}
