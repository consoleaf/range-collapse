use lazy_static::lazy_static;
use std::{fmt::Display, num::ParseIntError, str::FromStr};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct Range(i32, i32);
pub struct RangeVec(Vec<Range>);

#[derive(Ord, PartialOrd, PartialEq, Eq)]
pub enum RangePart {
    Start,
    End,
}

use RangePart::*;

pub fn calc(items: RangeVec) -> RangeVec {
    let items = items.0;
    let mut track = vec![];

    for item in items {
        track.push((item.0, Start));
        track.push((item.1, End));
    }

    track.sort();

    let mut result = vec![];
    let mut current = None;
    let mut cnt = 0;

    for (num, position) in track {
        match (cnt, &position) {
            (0, Start) => {
                current = Some(num);
            }
            (1, End) => {
                result.push(Range(current.unwrap(), num));
                current = None;
            }
            _ => (),
        }
        match position {
            Start => cnt += 1,
            End => cnt -= 1,
        }
    }

    return RangeVec(result);
}

#[cfg(test)]
mod tests {
    use super::RangeParseError;

    #[test]
    fn test() -> Result<(), RangeParseError> {
        let cases = vec![
            ("1-5,3-7", "1-7"),
            ("-4, 20-30, 2-7", "-7, 20-30"),
            ("-4, 5-10", "-4, 5-10"),
            ("1-20, 15-25, 22-40", "1-40"),
            ("-100, 50-255", "-255"),
        ];

        for (input, expected) in cases {
            let res = super::calc(input.parse()?);
            assert_eq!(res.to_string(), expected);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct RangeParseError(String);
impl Display for RangeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl RangeParseError {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }
}
impl From<ParseIntError> for RangeParseError {
    fn from(value: ParseIntError) -> Self {
        Self(value.to_string())
    }
}

lazy_static! {
    static ref RANGE_REGEX: Regex = Regex::new(r"(-?\d*)-(-?\d+)").unwrap();
}

impl FromStr for Range {
    type Err = RangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, [start, end]) = RANGE_REGEX
            .captures(s)
            .ok_or(RangeParseError::new("Bad range format"))?
            .extract();
        return Ok(Self(
            (if start == "" { "0" } else { start })
                .parse()
                .map_err(RangeParseError::from)?,
            end.parse().map_err(RangeParseError::from)?,
        ));
    }
}

impl FromStr for RangeVec {
    type Err = RangeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Result<Vec<_>, _> = s.split(",").map(Range::from_str).collect();
        items.and_then(|x| Ok(RangeVec(x)))
    }
}

impl ToString for RangeVec {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|a| {
                format!(
                    "{}-{}",
                    if a.0 == 0 {
                        "".to_string()
                    } else {
                        a.0.to_string()
                    },
                    a.1
                )
            })
            .join(", ")
    }
}
