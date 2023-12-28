use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::{Match, Regex};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_date_regex_can_parse_year_month_day() {
        // Arrange
        let haystack = "1234-56-78";

        // Act
        let captures = REGEX.captures(haystack).unwrap();

        // Assert
        let overall = captures.get(0).unwrap().as_str();
        let year = captures.get(1).unwrap().as_str();
        let month = captures.get(2).unwrap().as_str();
        let day = captures.get(3).unwrap().as_str();

        assert_eq!(overall, "1234-56-78");
        assert_eq!(year, "1234");
        assert_eq!(month, "56");
        assert_eq!(day, "78");
    }
}

/// Stores the year/month/day
pub struct Date {
    year: u8,
    month: u8,
    day: u8,
}

/// Regular Expression that parses out the YYYY-MM-DD from a string.
static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d{4})-(\d{2})-(\d{2})(_DC)?").unwrap());

/// Attempts to parse the u8 value out of the Option<Match>, returns 0u8 on failure.
fn match_to_u8(potential_match: Option<Match>) -> u8 {
    // A match occured
    if let Some(unwrapped_match) = potential_match {
        // Attempt to parse out the u8 from the str
        let unwrapped_str = unwrapped_match.as_str();

        // Parse str to u8 or default value.
        unwrapped_str.parse::<u8>().unwrap_or(0u8)
    } else {
        // Zero means nothing was parsed.
        0
    }
}

impl Date {
    pub fn from_str(str: &str) -> Option<Date> {
        let captures = REGEX.captures(str)?;

        let date = Date {
            year: match_to_u8(captures.get(1)),
            month: match_to_u8(captures.get(2)),
            day: match_to_u8(captures.get(3)),
        };

        if date.is_valid() {
            Some(date)
        } else {
            None
        }
    }

    pub fn is_valid(self: &Date) -> bool {

        if self.year == 0 { 
            return false; 
        }

        if self.day == 0 {
            return false;
        }

        let days_in_month = match self.month {
            // January - 31 days
            1 => 31,
            // February - 28 days in a common year and 29 days in leap years
            2 => if self.year % 4 == 0 { 29 } else { 28 },
            // March - 31 days
            3 => 31,
            // April - 30 days
            4 => 30,
            // May - 31 days
            5 => 31,
            // June - 30 days
            6 => 30,
            // July - 31 days
            7 => 31,
            // August - 31 days
            8 => 31,
            // September - 30 days
            9 => 30,
            // October - 31 days
            10 => 31,
            // November - 30 days
            11 => 30,
            // December - 31 days
            12 => 31,
            // Month out of range
            _ => { return false; },
        };

        self.day <= days_in_month
    }
}
