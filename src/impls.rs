use crate::{
    error::BinaryErrorVariant, BinaryError, BinaryReadResult, FromBytesVariable, IntoBytesVariable,
};

impl FromBytesVariable for String {
    const TERMINATION: Option<u8> = Some(0x0);

    fn from_bytes(bytes: &[u8]) -> BinaryReadResult<Self> {
        let mut str = String::new();
        let mut off = 0;

        loop {
            if off > bytes.len() {
                return Err(BinaryError::new(
                    "Slice of bytes too short",
                    BinaryErrorVariant::Variable,
                ));
            }

            match <String as FromBytesVariable>::TERMINATION {
                Some(t) => {
                    if off > bytes.len() {
                        return Err(BinaryError::new(
                            "Slice of bytes too short",
                            BinaryErrorVariant::Variable,
                        ));
                    }

                    if bytes[off] == t {
                        break;
                    }
                }
                None => {
                    if off > bytes.len() {
                        break;
                    }
                }
            }

            str.push(bytes[off].into());
            off += 1;
        }

        return Ok(str);
    }
}

impl IntoBytesVariable for String {
    const TERMINATION: Option<u8> = Some(0x0);

    fn to_bytes(self) -> Vec<u8> {
        match <String as IntoBytesVariable>::TERMINATION {
            Some(t) => {
                let mut v = self.into_bytes();
                v.push(t);
                return v;
            }
            None => self.into_bytes(),
        }
    }

    fn size(&self) -> usize {
        self.len()
    }
}
