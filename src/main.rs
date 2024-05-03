use std::collections::HashSet;
use regex::Regex;
use once_cell::sync::Lazy;
use pipeline::{pipe, pipe_fun};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    if let Err(e) = process_lines(reader) {
        eprintln!("Error processing lines: {}", e);
    }
}

fn process_lines<R>(reader : R) -> io::Result<()>
where
    R: BufRead,
{
    for line in reader.lines() {
        let line = line?;
        match process_line(&line) {
            Ok(processed_line) => println!("{}", processed_line),
            Err(err) => eprintln!("ERROR {:?}", err),
        };
    }

    Ok(())
}

static COMPONENT_ID_REGEX : Lazy<Regex> = Lazy::new(||
    Regex::new(r"([A-Z]+)(\d+)").expect("Invalid regex pattern")
);

fn extract_ids<'a>(line : &'a str) -> impl Iterator<Item = (&'a str, u32)> + 'a {
    line
        .split(',')
        .filter_map(move |s| {
            COMPONENT_ID_REGEX
                .captures(s)
                .and_then(|captures| {
                    let component_type =
                        captures.get(1)?.as_str();
                    let n =
                        captures
                        .get(2)?
                        .as_str()
                        .parse::<u32>()
                        .ok()?;
                    Some((component_type,n))
                })
        })
}

fn find_number_ranges(xs : impl IntoIterator<Item = u32>) -> Vec<Vec<u32>>
{
    xs
        .into_iter()
        .fold(vec![], |mut acc, n| {
            if let Some(last_pairs) = acc.last_mut() {
                if let Some(&last_val) = last_pairs.last() {
                    if last_val == n - 1 {
                        last_pairs.push(n);
                        return acc;
                    }
                }
            }
            acc.push(vec![n]);
            acc
        })
}

fn condense_ranges(xss : impl IntoIterator<Item = Vec<u32>>) -> String
{
    xss
        .into_iter()
        .filter_map(|xs|
            if xs.len() > 2 {
                let first = xs.first().unwrap().to_string();
                let last = xs.last().unwrap().to_string();
                Some(format!("{}-{}", first, last))
            } else if xs.len() == 0 {
                None
            } else {
                Some(
                    xs
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","))
            }
        )
        .collect::<Vec<_>>()
        .join(",")
}

#[derive(Debug, PartialEq)]
enum LineError {
    MixedComponentTypes(HashSet<String>),
    NoComponentType
}

fn process_line(line: &str) -> Result<String, LineError> {
    let ids = extract_ids(line);
    let (component_types,component_numbers) : (Vec<_>,Vec<_>) = ids.into_iter().unzip();

    // validate that the textual part of all the component ids are the same
    let distint_component_types : HashSet<&str> =
        component_types.into_iter().collect();
    let num_component_types = distint_component_types.len();
    if num_component_types == 0 {
        return Err(LineError::NoComponentType)
    } else if num_component_types > 1 {
        return Err(
            LineError::MixedComponentTypes(
                distint_component_types
                .into_iter()
                .map(String::from)
                .collect::<HashSet<_>>()))
    }
    let component_type = *distint_component_types.iter().next().unwrap();

    let condensed_ranges : String = pipe!(
        component_numbers.into_iter()
        => find_number_ranges
        => condense_ranges
    );

    Ok(format!("{}:{}", component_type, condensed_ranges))
}

#[cfg(test)]
mod find_number_ranges_tests {
    use super::*;

    #[test]
    fn test_empty_vector() {
        let numbers: Vec<u32> = vec![];
        let result = find_number_ranges(numbers);
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_element() {
        let numbers = vec![1];
        let result = find_number_ranges(numbers);
        assert_eq!(result, vec![vec![1]]);
    }

    #[test]
    fn test_consecutive_numbers() {
        let numbers = vec![1, 2, 3, 4];
        let result = find_number_ranges(numbers);
        assert_eq!(result, vec![vec![1, 2, 3, 4]]);
    }

    #[test]
    fn test_non_consecutive_numbers() {
        let numbers = vec![1, 3, 5, 7];
        let result = find_number_ranges(numbers);
        assert_eq!(result, vec![vec![1], vec![3], vec![5], vec![7]]);
    }

    #[test]
    fn test_mixed_numbers() {
        let numbers = vec![1, 2, 4, 5, 6, 8];
        let result = find_number_ranges(numbers);
        assert_eq!(result, vec![vec![1, 2], vec![4, 5, 6], vec![8]]);
    }
}

#[cfg(test)]
mod condense_ranges_tests {
    use super::*;

    #[test]
    fn test_empty_vector() {
        let input: Vec<Vec<u32>> = vec![vec![]];
        let expected = "";
        assert_eq!(condense_ranges(input), expected);
    }

    #[test]
    fn test_single_element_vector() {
        let input = vec![vec![1]];
        let expected = "1";
        assert_eq!(condense_ranges(input), expected);
    }

    #[test]
    fn test_multiple_element_vector() {
        let input = vec![vec![1, 2, 3, 4]];
        let expected = "1-4";
        assert_eq!(condense_ranges(input), expected);
    }

    #[test]
    fn test_mixed_vectors() {
        let input = vec![vec![], vec![1], vec![3, 4, 5, 6], vec![8, 9]];
        let expected = "1,3-6,8,9";
        assert_eq!(condense_ranges(input), expected);
    }

    #[test]
    fn test_multiple_vectors() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6, 7], vec![8, 9]];
        let expected = "1-3,4-7,8,9";
        assert_eq!(condense_ranges(input), expected);
    }
}

#[cfg(test)]
mod process_line_tests {
    use super::*;

    #[test]
    fn success_cases() {
        let test_cases = vec![
            ("R1, R3, R4, R7", "R:1,3,4,7"),
            ("   R1,R3  ,  R4,R7   ", "R:1,3,4,7"),
            ("R1,R2,R3,R7,R8,R22,R23,R24,R25", "R:1-3,7,8,22-25"),
            ("IC1,IC3,IC4,IC7", "IC:1,3,4,7"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(process_line(input), Ok(String::from(expected)));
        }
    }

    #[test]
    fn error_cases_no_component_type() {
        let test_cases : Vec<(&str, Result<String, LineError>)> = vec![
            ("foobar", Err(LineError::NoComponentType)),
            ("7", Err(LineError::NoComponentType)),
        ];

        for (input, expected) in test_cases {
            assert_eq!(process_line(input), expected);
        }
    }

    #[test]
    fn error_cases_mixed_component_types() {
        let test_cases : Vec<(&str, Result<String, LineError>)> = vec![
            ("R1, R2, D5",
             Err(LineError::MixedComponentTypes(HashSet::from([String::from("D"), String::from("R")])))),
        ];

        for (input, expected) in test_cases {
            assert_eq!(process_line(input), expected);
        }
    }
}
