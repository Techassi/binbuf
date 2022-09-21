use std::fmt;

#[derive(Clone, Copy)]
pub enum Endianness {
    Big,
    Little,
}

impl<T: Into<String>> From<T> for Endianness {
    fn from(input: T) -> Self {
        let input: String = input.into();
        return match input.to_lowercase().as_str() {
            "big" => Self::Big,
            "little" => Self::Little,
            _ => panic!("Invalid endianness"),
        };
    }
}

impl fmt::Display for Endianness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Big => write!(f, "Big"),
            Self::Little => write!(f, "Little"),
        }
    }
}

impl fmt::Debug for Endianness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Big => write!(f, "Big"),
            Self::Little => write!(f, "Little"),
        }
    }
}
