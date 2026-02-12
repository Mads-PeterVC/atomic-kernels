#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AtomicNumber(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtomicNumberError {
    OutOfRange { z: u8, max: u8 },
    ConversionFailed,
}

impl AtomicNumber {
    pub const MAX: u8 = 110;

    pub fn new<T>(z: T) -> Result<Self, AtomicNumberError>
    where
        T: TryInto<u8>,
    {
        let z_u8: u8 = z
            .try_into()
            .map_err(|_| AtomicNumberError::ConversionFailed)?;

        if z_u8 > 0 && z_u8 <= Self::MAX {
            Ok(Self(z_u8))
        } else {
            Err(AtomicNumberError::OutOfRange {
                z: z_u8,
                max: Self::MAX,
            })
        }
    }

    pub const fn get(self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {

    use crate::geometry::{AtomicNumber, AtomicNumberError};

    #[test]
    fn test_new() {
        let z = AtomicNumber::new(100).unwrap();
        assert_eq!(z.get(), 100);
    }

    #[test]
    fn test_out_of_range_error() {
        let z = AtomicNumber::new(111);
        assert_eq!(z, Err(AtomicNumberError::OutOfRange { z: 111, max: 110 }));
    }

    #[test]
    fn test_conversion_error() {
        let z = AtomicNumber::new(2000);
        assert_eq!(z, Err(AtomicNumberError::ConversionFailed));
    }
}
