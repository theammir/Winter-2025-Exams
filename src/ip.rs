use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct IPv4([u8; 4]);

#[derive(Debug, PartialEq, Eq)]
pub enum ConversionError {
    IncorrectOctetCount,
    InvalidOctet,
}

impl FromStr for IPv4 {
    type Err = ConversionError;

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
