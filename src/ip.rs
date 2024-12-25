//! Parse IPv4 from string into its octets.
//!
//! # Specification
//! Given a string with a (probably invalid) IPv4 address, return a list of 4 integers
//! representing its octets.

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct IPv4(pub [u8; 4]);

#[derive(Debug, PartialEq, Eq)]
pub enum ConversionError {
    /// Splitting the string by `.` showed incorrect number of substrings.
    IncorrectOctetCount,
    /// An octet couldn't be parsed into `u8`.
    InvalidOctet,
}

impl FromStr for IPv4 {
    type Err = ConversionError;

    /// Although very concise, the implementation is not perfect due to having two `collect` calls,
    /// and thus two heap allocations.
    ///
    /// # Example
    /// ```rust
    /// use winter_2025_exams::ip::*;
    ///
    /// assert_eq!("127.0.0.1".parse::<IPv4>().unwrap(), IPv4([127, 0, 0, 1]));
    /// assert_eq!("100500.22.30.l".parse::<IPv4>().unwrap_err(), ConversionError::InvalidOctet);
    /// ```
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let octets: Vec<&str> = value.split('.').collect();
        if octets.len() != 4 {
            return Err(ConversionError::IncorrectOctetCount);
        }

        Ok(IPv4(
            octets
                .iter()
                .map(|o| o.parse::<u8>())
                .collect::<Result<Vec<u8>, _>>()
                .map_err(|_| ConversionError::InvalidOctet)?
                .try_into()
                .unwrap(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{ConversionError, FromStr, IPv4};

    #[test]
    fn test_ip_ok_cases() {
        let ok_cases = [
            ("127.0.0.1", IPv4([127, 0, 0, 1])),
            ("0.0.0.0", IPv4([0, 0, 0, 0])),
            ("255.255.255.0", IPv4([255, 255, 255, 0])),
            ("10.0.0.10", IPv4([10, 0, 0, 10])),
        ];

        for (input, expected) in ok_cases {
            assert_eq!(IPv4::from_str(input).expect("got err instead"), expected);
        }
    }

    #[test]
    fn test_ip_err_cases() {
        let err_cases = [
            (".0.0.", ConversionError::InvalidOctet),
            ("127001", ConversionError::IncorrectOctetCount),
            ("127.0.0", ConversionError::IncorrectOctetCount),
            ("", ConversionError::IncorrectOctetCount),
            ("256.256.1177.5", ConversionError::InvalidOctet),
        ];

        for (input, expected) in err_cases {
            assert_eq!(IPv4::from_str(input).expect_err("got ok instead"), expected)
        }
    }
}
