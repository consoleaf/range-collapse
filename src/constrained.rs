use std::{fmt::Display, num::ParseIntError, str::FromStr};

use itertools::Itertools;

#[derive(Debug)]
pub struct Range(u8, u8);
pub struct RangeVec(Vec<Range>);

pub fn calc(items: RangeVec) -> RangeVec {
    let items = items.0;
    let mut track = std::iter::repeat(0).take(256).collect_vec();

    for item in items {
        track[item.0 as usize] += 1;
        track[item.1 as usize] -= 1;
    }

    let mut result = vec![];
    let mut current_range = None;
    let mut cnt = 0;

    track.iter().enumerate().for_each(|(idx, val)| {
        if cnt == 0 && *val > 0 {
            current_range = Some(idx.try_into().unwrap());
        }
        if cnt > 0 && cnt + val == 0 {
            result.push(Range(current_range.unwrap(), idx.try_into().unwrap()));
        }
        cnt += val;
    });

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

impl FromStr for RangeVec {
    type Err = RangeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Result<Vec<_>, _> = s
            .split(",")
            .map(|item| {
                let v = item.trim().split("-").collect_vec();
                if v.len() != 2 {
                    return Err(RangeParseError("Wrong format".to_string()));
                }
                Ok(Range(
                    if v[0] == "" {
                        0
                    } else {
                        v[0].parse()
                            .map_err(|x: ParseIntError| RangeParseError(x.to_string()))?
                    },
                    v[1].parse()
                        .map_err(|x: ParseIntError| RangeParseError(x.to_string()))?,
                ))
                // .map(|x| u8::from_str(x.trim()).map_err(|x| RangeParseError(x.to_string())))
                // .collect::<Result<Vec<_>, _>>()
            })
            .collect();

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
