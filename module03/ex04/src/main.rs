use std::str::FromStr;
use std::fmt::{Display, Debug, Formatter};

#[derive(Debug)]
struct Time {
    hours: u32,
    minutes: u32,
}

#[derive(Debug)]
enum TimeParseError {
    MissingColon,
    InvalidLength,
    InvalidNumber,
}

impl Display for TimeParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            TimeParseError::MissingColon => write!(f, "Missing colon"),
            TimeParseError::InvalidLength => write!(f, "Invalid length"),
            TimeParseError::InvalidNumber => write!(f, "Invalid number"),
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:02} hours, {:02} minutes", self.hours, self.minutes)
    }
}

impl FromStr for Time {
    type Err = TimeParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut time = Time {hours: 0, minutes: 0};
        let mut i = 0;
        let mut j = 0;
        let sb = s.as_bytes();
        while sb.len() > i + j && sb[i].is_ascii_digit() {
            time.hours = time.hours * 10 + (sb[i] - 48) as u32;
            i += 1;
        }
        if (sb[i] as char) != ':' {
            return Err(TimeParseError::MissingColon);
        }
        i += 1;
        while sb.len() > i + j  {
            if !sb[i + j].is_ascii_digit() {
                return Err(TimeParseError::InvalidNumber);
            }
            time.minutes = time.minutes * 10 + (sb[i + j] - 48) as u32;
            j += 1;
        }
        if j != 2 {
            return Err(TimeParseError::InvalidLength);
        }
        return Ok(time);

    }
}


fn main() {
    let a: Time = "12:20".parse().unwrap();
    let b: Time = "15:14".parse().unwrap();

    println!("{a}");
    println!("{b}");

    let err1: TimeParseError = "12.20".parse::<Time>().unwrap_err();
    let err2: TimeParseError = "12:2".parse::<Time>().unwrap_err();
    let err3: TimeParseError = "12:2a".parse::<Time>().unwrap_err();
    println!("error: {err1}");
    println!("error: {err2}");
    println!("error: {err3}");
}
