use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SumMode {
    Auto,
    NumericOnly,
    ConcatOnly,
}

impl FromStr for SumMode {
    type Err = String;

    fn from_str(input: &str) -> Result<SumMode, Self::Err> {
        match input.to_lowercase().as_str() {
            "auto" => Ok(SumMode::Auto),
            "numeric" => Ok(SumMode::NumericOnly),
            "concat" => Ok(SumMode::ConcatOnly),
            _ => Err(format!("Invalid Input: {}", input)),
        }
    }
}