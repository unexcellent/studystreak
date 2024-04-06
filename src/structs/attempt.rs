use regex::Regex;

macro_rules! regex_pattern {
    ($pat:expr) => {
        Regex::new($pat).expect("Invalid regex pattern")
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum Attempt {
    Correct,
    Incorrect,
    WithHelp,
    Skipped,
    PartiallyCorrect(u32, u32),
}
impl Attempt {
    /// Construct the attempt enum from a string
    pub fn parse(raw_attempt: &str) -> Result<Attempt, UnsupportedAttemptStringError> {
        if regex_pattern!(r"\d+/\d+").is_match(raw_attempt) {
            return Ok(Attempt::parse_partially_correct(raw_attempt))
        }

        match raw_attempt {
            "v" => Ok(Attempt::Correct),
            "x" => Ok(Attempt::Incorrect),
            "h" => Ok(Attempt::WithHelp),
            "-" => Ok(Attempt::Skipped),
            _ => Err(UnsupportedAttemptStringError),
        }
    }

    fn parse_partially_correct(raw_attempt: &str) -> Attempt {
        let numbers: Vec<&str> = raw_attempt
            .split('/')
            .collect();

        Attempt::PartiallyCorrect(
            numbers.first().unwrap().parse().unwrap(), 
            numbers.get(1).unwrap().parse().unwrap()
        )
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct UnsupportedAttemptStringError;


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_correct_from_str() {
        assert_eq!(Attempt::parse("v"), Ok(Attempt::Correct));
    }

    #[test]
    fn test_incorrect_from_str() {
        assert_eq!(Attempt::parse("x"), Ok(Attempt::Incorrect));
    }

    #[test]
    fn test_withhelp_from_str() {
        assert_eq!(Attempt::parse("h"), Ok(Attempt::WithHelp));
    }

    #[test]
    fn test_skipped_from_str() {
        assert_eq!(Attempt::parse("-"), Ok(Attempt::Skipped));
    }

    #[test]
    fn test_partially_correct_from_str() {
        assert_eq!(Attempt::parse("4/12"), Ok(Attempt::PartiallyCorrect(4, 12)));
    }

    #[test]
    fn test_invalid_str() {
        assert_eq!(Attempt::parse("some_garbage"), Err(UnsupportedAttemptStringError));
    }

}