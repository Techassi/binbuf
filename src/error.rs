use std::fmt;

#[derive(Debug)]
pub enum BinaryErrorVariant {
    U16,
    U32,
    U64,
    U128,
    Variable,
}

impl fmt::Display for BinaryErrorVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryErrorVariant::U16 => write!(f, "u16/i16"),
            BinaryErrorVariant::U32 => write!(f, "u32/i32"),
            BinaryErrorVariant::U64 => write!(f, "u64/i64"),
            BinaryErrorVariant::U128 => write!(f, "u128/i128"),
            BinaryErrorVariant::Variable => write!(f, "variable"),
        }
    }
}

pub struct BinaryError {
    variant: BinaryErrorVariant,
    message: String,
}

impl BinaryError {
    pub fn new<M: Into<String>>(message: M, variant: BinaryErrorVariant) -> Self {
        return Self {
            message: message.into(),
            variant,
        };
    }
}

impl fmt::Display for BinaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.message, self.variant)
    }
}

impl fmt::Debug for BinaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BinaryError")
            .field("message", &self.message)
            .field("variant", &self.variant)
            .finish()
    }
}
