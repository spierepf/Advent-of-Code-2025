use std::{num::ParseIntError, str::FromStr};

#[derive(PartialEq, Debug)]
pub enum Rotation {
    Left(u32),
    Right(u32),
}

#[derive(PartialEq, Debug)]
pub struct ParseRotationError;

impl From<ParseIntError> for ParseRotationError {
    fn from(_value: ParseIntError) -> Self {
        Self
    }
}

impl FromStr for Rotation {
    type Err = ParseRotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_letter = s.chars().next().ok_or(ParseRotationError)?;

        let skip_first_letter = &s[1..];
        let rotate_amount = skip_first_letter.parse()?;

        return match first_letter {
            'L' => Ok(Rotation::Left(rotate_amount)),
            'R' => Ok(Rotation::Right(rotate_amount)),
            _ => Err(ParseRotationError),
        };
    }
}

#[test]
fn test_we_can_parse_a_rotation() {
    assert_eq!("L68".parse::<Rotation>(), Ok(Rotation::Left(68)));
    assert_eq!("L30".parse::<Rotation>(), Ok(Rotation::Left(30)));
    assert_eq!("R48".parse::<Rotation>(), Ok(Rotation::Right(48)));
    assert_eq!("Q48".parse::<Rotation>(), Err(ParseRotationError));
    assert_eq!("".parse::<Rotation>(), Err(ParseRotationError));
    assert_eq!("L256".parse::<Rotation>(), Ok(Rotation::Left(256)));
    assert_eq!("L256".parse::<Rotation>(), Ok(Rotation::Left(256)));
    assert_eq!("R987".parse::<Rotation>(), Ok(Rotation::Right(987)));
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Dial(pub u8);

impl Dial {
    const POSITIONS: u32 = 100;
    pub fn rotate(&self, rotation: Rotation) -> Dial {
        match rotation {
            Rotation::Left(amount) => Dial(
                ((self.0 as u32 + Self::POSITIONS - (amount % Self::POSITIONS)) % Self::POSITIONS)
                    as u8,
            ),
            Rotation::Right(amount) => {
                Dial(((self.0 as u32 + (amount % Self::POSITIONS)) % Self::POSITIONS) as u8)
            }
        }
    }
}

#[test]
fn test_our_dial_knows_where_its_at() {
    assert_eq!(Dial(50), Dial(50));
    assert_eq!(Dial(50).rotate(Rotation::Left(68)), Dial(82));
    assert_eq!(Dial(82).rotate(Rotation::Left(30)), Dial(52));
    assert_eq!(Dial(52).rotate(Rotation::Right(48)), Dial(0));
    assert_eq!(Dial(0).rotate(Rotation::Left(5)), Dial(95));
    assert_eq!(
        Dial(99).rotate(Rotation::Left(u32::MAX - (u32::MAX % 100))),
        Dial(99)
    );
    assert_eq!(
        Dial(99).rotate(Rotation::Right(u32::MAX - (u32::MAX % 100))),
        Dial(99)
    );
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct DialAndCount {
    pub dial: Dial,
    pub count: u32,
}

impl DialAndCount {
    pub fn rotate(&self, rotation: Rotation) -> DialAndCount {
        let crossings = match rotation {
            Rotation::Left(amount) => {
                amount / 100
                    + ((self.dial.0 != 0) && ((self.dial.0 as u32) < (amount % 100))) as u32
            }
            Rotation::Right(amount) => {
                amount / 100 + (self.dial.0 as u32 + (amount % 100) > 100) as u32
            }
        };

        let new_dial = self.dial.rotate(rotation);
        Self {
            dial: new_dial,
            count: self.count + crossings + (new_dial == Dial(0)) as u32,
        }
    }
}

#[test]
fn test_our_dial_knows_how_often_it_has_hit_zero() {
    assert_eq!(
        DialAndCount {
            dial: Dial(50),
            count: 0
        },
        DialAndCount {
            dial: Dial(50),
            count: 0
        }
    );
    assert_eq!(
        DialAndCount {
            dial: Dial(50),
            count: 0
        }
        .rotate(Rotation::Left(1)),
        DialAndCount {
            dial: Dial(49),
            count: 0
        }
    );
    assert_eq!(
        DialAndCount {
            dial: Dial(50),
            count: 0
        }
        .rotate(Rotation::Left(2)),
        DialAndCount {
            dial: Dial(48),
            count: 0
        }
    );
    assert_eq!(
        DialAndCount {
            dial: Dial(50),
            count: 0
        }
        .rotate(Rotation::Right(1)),
        DialAndCount {
            dial: Dial(51),
            count: 0
        }
    );
    assert_eq!(
        DialAndCount {
            dial: Dial(50),
            count: 1
        }
        .rotate(Rotation::Right(1)),
        DialAndCount {
            dial: Dial(51),
            count: 1
        }
    );
    assert_eq!(
        DialAndCount {
            dial: Dial(50),
            count: 0
        }
        .rotate(Rotation::Left(68)),
        DialAndCount {
            dial: Dial(82),
            count: 1
        }
    );
    assert_eq!(
        DialAndCount {
            dial: Dial(95),
            count: 0
        }
        .rotate(Rotation::Right(60)),
        DialAndCount {
            dial: Dial(55),
            count: 1
        }
    );
    assert_eq!(
        DialAndCount {
            dial: Dial(50),
            count: 0
        }
        .rotate(Rotation::Left(249)),
        DialAndCount {
            dial: Dial(1),
            count: 2
        }
    );
    assert_eq!(
        DialAndCount {
            dial: Dial(50),
            count: 0
        }
        .rotate(Rotation::Right(249)),
        DialAndCount {
            dial: Dial(99),
            count: 2
        }
    );
    assert_eq!(
        DialAndCount {
            dial: Dial(50),
            count: 0
        }
        .rotate(Rotation::Right(1000)),
        DialAndCount {
            dial: Dial(50),
            count: 10
        }
    );
    assert_eq!(
        DialAndCount {
            dial: Dial(52),
            count: 1
        }
        .rotate(Rotation::Right(48)),
        DialAndCount {
            dial: Dial(0),
            count: 2
        }
    );
    assert_eq!(
        DialAndCount {
            dial: Dial(55),
            count: 2
        }
        .rotate(Rotation::Left(55)),
        DialAndCount {
            dial: Dial(0),
            count: 3
        }
    );
    assert_eq!(
        DialAndCount {
            dial: Dial(55),
            count: 2
        }
        .rotate(Rotation::Left(155)),
        DialAndCount {
            dial: Dial(0),
            count: 4
        }
    );
    assert_eq!(
        DialAndCount {
            dial: Dial(0),
            count: 2
        }
        .rotate(Rotation::Left(5)),
        DialAndCount {
            dial: Dial(95),
            count: 2
        }
    );
    assert_eq!(
        DialAndCount {
            dial: Dial(0),
            count: 5
        }
        .rotate(Rotation::Right(14)),
        DialAndCount {
            dial: Dial(14),
            count: 5
        }
    );
}

impl Default for Dial {
    fn default() -> Self {
        Dial(50)
    }
}

#[test]
fn dial_defaults_to_50() {
    assert_eq!(Dial::default(), Dial(50));
}

pub fn calculate_password(input: &mut dyn std::io::BufRead) -> u32 {
    use std::io::BufRead;
    input
        .lines()
        .fold((Dial::default(), 0), |(dial, count), line| {
            let line = line.expect("failed to read line");
            let rotation = line.parse::<Rotation>().expect("failed to parse rotation");
            let new_dial = dial.rotate(rotation);
            if new_dial == Dial(0) {
                return (new_dial, count + 1);
            } else {
                return (new_dial, count);
            }
        })
        .1
}

#[test]
fn test_calculate_password() {
    assert_eq!(
        calculate_password(&mut std::io::Cursor::new(
            "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n".as_bytes()
        )),
        3
    );
}

#[test]
fn test_default_dial_and_count_is_dial_50_count_0() {
    assert_eq!(
        DialAndCount::default(),
        DialAndCount {
            dial: Dial::default(),
            count: 0
        }
    );
}

pub fn calculate_password_2(input: &mut dyn std::io::BufRead) -> u32 {
    use std::io::BufRead;
    input
        .lines()
        .fold(DialAndCount::default(), |dial_and_count, line| {
            let line = line.expect("failed to read line");
            let rotation = line.parse::<Rotation>().expect("failed to parse rotation");
            return dial_and_count.rotate(rotation);
        })
        .count
}

#[test]
fn test_calculate_password_2() {
    assert_eq!(
        calculate_password_2(&mut std::io::Cursor::new(
            "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n".as_bytes()
        )),
        6
    );
}
