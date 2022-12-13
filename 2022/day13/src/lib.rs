#[cfg(all(feature = "nom-parser", feature = "json-parser"))]
compile_error!(
    "feature \"nom-parser\" and feature \"json-parser\" cannot be enabled at the same time"
);

#[cfg(not(any(feature = "nom-parser", feature = "json-parser")))]
compile_error!("at least one parser has to be enabled");

use std::{cmp::Ordering, fmt::Display, str::FromStr};

#[derive(Clone, Debug)]
pub enum Value {
    Integer(u8),
    List(Vec<Value>),
}

#[cfg(feature = "nom-parser")]
mod nom_parser {
    use nom::{
        branch::alt,
        character::streaming::char,
        combinator::{cut, map},
        error::make_error,
        multi::separated_list0,
        sequence::{preceded, terminated},
        IResult,
    };

    use super::Value;

    fn parse_number(input: &str) -> IResult<&str, u8> {
        if input.len() < 1 {
            Err(nom::Err::Error(make_error(
                input,
                nom::error::ErrorKind::Eof,
            )))
        } else {
            let number_end = input.as_bytes().iter().position(|c| !c.is_ascii_digit());

            if let Some(end) = number_end {
                let (num_str, rem) = input.split_at(end);

                if num_str.is_empty() {
                    Err(nom::Err::Error(make_error(
                        input,
                        nom::error::ErrorKind::Digit,
                    )))
                } else {
                    Ok((rem, num_str.parse().unwrap()))
                }
            } else {
                Err(nom::Err::Error(make_error(
                    input,
                    nom::error::ErrorKind::Digit,
                )))
            }
        }
    }

    fn parse_list(input: &str) -> IResult<&str, Vec<Value>> {
        preceded(
            char('['),
            cut(terminated(
                separated_list0(char(','), parse_value),
                char(']'),
            )),
        )(input)
    }

    fn parse_value(input: &str) -> IResult<&str, Value> {
        alt((
            map(parse_number, Value::Integer),
            map(parse_list, Value::List),
        ))(input)
    }

    pub fn parse_signal(input: &str) -> Value {
        parse_value(input).unwrap().1
    }
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[cfg(feature = "nom-parser")]
        {
            Ok(nom_parser::parse_signal(s))
        }

        #[cfg(feature = "json-parser")]
        {
            let v = serde_json::from_str::<serde_json::Value>(s).unwrap();

            Ok(Self::from_json(v))
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{n}")?,
            Value::List(list) => {
                write!(f, "[")?;
                for item in &list[..list.len().saturating_sub(1)] {
                    write!(f, "{item},")?;
                }
                if let Some(item) = list.last() {
                    write!(f, "{item}")?;
                }
                write!(f, "]")?;
            }
        };

        Ok(())
    }
}

impl Value {
    #[cfg(feature = "json-parser")]
    fn from_json(json: serde_json::Value) -> Self {
        match json {
            serde_json::Value::Number(num) => Self::Integer(num.as_u64().unwrap() as u8),
            serde_json::Value::Array(arr) => {
                Self::List(arr.into_iter().map(Value::from_json).collect())
            }
            _ => panic!("invalid json object"),
        }
    }

    /// This is different from [`eq`]: It campares whether the representation is actually the same,
    /// not just the same value.
    fn is_same(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => {
                l0.len() == r0.len() && std::iter::zip(l0, r0).all(|(a, b)| a.is_same(b))
            }
            _ => false,
        }
    }
}

impl PartialOrd<Self> for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a.cmp(b),
            (Value::List(a), Value::List(b)) => {
                for i in 0..a.len().max(b.len()) {
                    let left = a.get(i);
                    let right = b.get(i);

                    if let Some((a, b)) = left.zip(right) {
                        let ab = a.cmp(b);
                        if ab != Ordering::Equal {
                            return ab;
                        }
                    } else if right.is_none() {
                        return Ordering::Greater;
                    } else if left.is_none() {
                        return Ordering::Less;
                    } else {
                        unreachable!();
                    }
                }

                Ordering::Equal
            }
            (Value::Integer(_), Value::List(_)) => Self::List(vec![self.clone()]).cmp(other),
            (Value::List(_), Value::Integer(_)) => self.cmp(&Self::List(vec![other.clone()])),
        }
    }
}

impl Eq for Value {}
impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

pub fn parse_input(input: &str) -> Vec<Value> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Value>().unwrap())
        .collect()
}

pub fn part1(signals: &[Value]) -> usize {
    signals
        .chunks_exact(2)
        .enumerate()
        .filter(|(_i, chunk)| chunk[0].cmp(&chunk[1]) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part2(signals: &[Value]) -> usize {
    let mut signals = Vec::from(signals);

    let divider1: Value = "[[2]]".parse().unwrap();
    let divider2: Value = "[[6]]".parse().unwrap();
    signals.push(divider1.clone());
    signals.push(divider2.clone());

    signals.sort();

    let d1 = signals.iter().position(|s| s.is_same(&divider1)).unwrap() + 1;
    let d2 = signals.iter().position(|s| s.is_same(&divider2)).unwrap() + 1;

    d1 * d2
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_example_input_part1() {
        let signals = parse_input(EXAMPLE_INPUT);
        assert_eq!(part1(&signals), 13);
    }

    #[test]
    fn test_example_input_part2() {
        let signals = parse_input(EXAMPLE_INPUT);
        assert_eq!(part2(&signals), 140);
    }
}
