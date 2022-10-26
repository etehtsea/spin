wit_bindgen_rust::import!("../../wit/ephemeral/outbound-mysql.wit");

/// Exports the generated outbound MySQL items.
pub use outbound_mysql::*;

pub trait Decode: Sized {
    fn decode(value: &DbValue) -> Result<Self, anyhow::Error>;
}

impl<T> Decode for Option<T>
where
    T: Decode,
{
    fn decode(value: &DbValue) -> Result<Self, anyhow::Error> {
        match value {
            DbValue::DbNull => Ok(None),
            v => Ok(Some(T::decode(v)?)),
        }
    }
}

impl Decode for bool {
    fn decode(value: &DbValue) -> Result<Self, anyhow::Error> {
        match value {
            DbValue::Int8(0) => Ok(false),
            DbValue::Int8(1) => Ok(true),
            _ => Err(anyhow::anyhow!(
                "Expected TINYINT(1) from the DB but got {:?}",
                value
            )),
        }
    }
}

impl Decode for i8 {
    fn decode(value: &DbValue) -> Result<Self, anyhow::Error> {
        match value {
            DbValue::Int8(n) => Ok(*n),
            _ => Err(anyhow::anyhow!(
                "Expected TINYINT from the DB but got {:?}",
                value
            )),
        }
    }
}

impl Decode for i16 {
    fn decode(value: &DbValue) -> Result<Self, anyhow::Error> {
        match value {
            DbValue::Int16(n) => Ok(*n),
            _ => Err(anyhow::anyhow!(
                "Expected SMALLINT from the DB but got {:?}",
                value
            )),
        }
    }
}

impl Decode for i32 {
    fn decode(value: &DbValue) -> Result<Self, anyhow::Error> {
        match value {
            DbValue::Int32(n) => Ok(*n),
            _ => Err(anyhow::anyhow!(
                "Expected INT from the DB but got {:?}",
                value
            )),
        }
    }
}

impl Decode for i64 {
    fn decode(value: &DbValue) -> Result<Self, anyhow::Error> {
        match value {
            DbValue::Int64(n) => Ok(*n),
            _ => Err(anyhow::anyhow!(
                "Expected BIGINT from the DB but got {:?}",
                value
            )),
        }
    }
}

impl Decode for u8 {
    fn decode(value: &DbValue) -> Result<Self, anyhow::Error> {
        match value {
            DbValue::Uint8(n) => Ok(*n),
            _ => Err(anyhow::anyhow!(
                "Expected UNSIGNED TINYINT from the DB but got {:?}",
                value
            )),
        }
    }
}

impl Decode for u16 {
    fn decode(value: &DbValue) -> Result<Self, anyhow::Error> {
        match value {
            DbValue::Uint16(n) => Ok(*n),
            _ => Err(anyhow::anyhow!(
                "Expected UNSIGNED SMALLINT from the DB but got {:?}",
                value
            )),
        }
    }
}

impl Decode for u32 {
    fn decode(value: &DbValue) -> Result<Self, anyhow::Error> {
        match value {
            DbValue::Uint32(n) => Ok(*n),
            _ => Err(anyhow::anyhow!(
                "Expected UNSIGNED INT from the DB but got {:?}",
                value
            )),
        }
    }
}

impl Decode for u64 {
    fn decode(value: &DbValue) -> Result<Self, anyhow::Error> {
        match value {
            DbValue::Uint64(n) => Ok(*n),
            _ => Err(anyhow::anyhow!(
                "Expected UNSIGNED BIGINT from the DB but got {:?}",
                value
            )),
        }
    }
}

impl Decode for f32 {
    fn decode(value: &DbValue) -> Result<Self, anyhow::Error> {
        match value {
            DbValue::Floating32(n) => Ok(*n),
            _ => Err(anyhow::anyhow!(
                "Expected FLOAT from the DB but got {:?}",
                value
            )),
        }
    }
}

impl Decode for f64 {
    fn decode(value: &DbValue) -> Result<Self, anyhow::Error> {
        match value {
            DbValue::Floating64(n) => Ok(*n),
            _ => Err(anyhow::anyhow!(
                "Expected DOUBLE from the DB but got {:?}",
                value
            )),
        }
    }
}

impl Decode for Vec<u8> {
    fn decode(value: &DbValue) -> Result<Self, anyhow::Error> {
        match value {
            DbValue::Binary(n) => Ok(n.to_owned()),
            _ => Err(anyhow::anyhow!(
                "Expected BINARY, VARBINARY from the DB but got {:?}",
                value
            )),
        }
    }
}

impl Decode for String {
    fn decode(value: &DbValue) -> Result<Self, anyhow::Error> {
        match value {
            DbValue::Str(s) => Ok(s.to_owned()),
            _ => Err(anyhow::anyhow!(
                "Expected CHAR, VARCHAR, TEXT from the DB but got {:?}",
                value
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolean() {
        assert!(bool::decode(&DbValue::Int8(1)).unwrap());
        assert!(bool::decode(&DbValue::Int8(3)).is_err());
        assert!(bool::decode(&DbValue::Int32(0)).is_err());
        assert!(Option::<bool>::decode(&DbValue::DbNull).unwrap().is_none());
    }

    #[test]
    fn int8() {
        assert_eq!(i8::decode(&DbValue::Int8(0)).unwrap(), 0);
        assert!(i8::decode(&DbValue::Int32(0)).is_err());
        assert!(Option::<i8>::decode(&DbValue::DbNull).unwrap().is_none());
    }

    #[test]
    fn int16() {
        assert_eq!(i16::decode(&DbValue::Int16(0)).unwrap(), 0);
        assert!(i16::decode(&DbValue::Int32(0)).is_err());
        assert!(Option::<i16>::decode(&DbValue::DbNull).unwrap().is_none());
    }

    #[test]
    fn int32() {
        assert_eq!(i32::decode(&DbValue::Int32(0)).unwrap(), 0);
        assert!(i32::decode(&DbValue::Boolean(false)).is_err());
        assert!(Option::<i32>::decode(&DbValue::DbNull).unwrap().is_none());
    }

    #[test]
    fn int64() {
        assert_eq!(i64::decode(&DbValue::Int64(0)).unwrap(), 0);
        assert!(i64::decode(&DbValue::Boolean(false)).is_err());
        assert!(Option::<i64>::decode(&DbValue::DbNull).unwrap().is_none());
    }

    #[test]
    fn uint8() {
        assert_eq!(u8::decode(&DbValue::Uint8(0)).unwrap(), 0);
        assert!(u8::decode(&DbValue::Uint32(0)).is_err());
        assert!(Option::<u16>::decode(&DbValue::DbNull).unwrap().is_none());
    }

    #[test]
    fn uint16() {
        assert_eq!(u16::decode(&DbValue::Uint16(0)).unwrap(), 0);
        assert!(u16::decode(&DbValue::Uint32(0)).is_err());
        assert!(Option::<u16>::decode(&DbValue::DbNull).unwrap().is_none());
    }

    #[test]
    fn uint32() {
        assert_eq!(u32::decode(&DbValue::Uint32(0)).unwrap(), 0);
        assert!(u32::decode(&DbValue::Boolean(false)).is_err());
        assert!(Option::<u32>::decode(&DbValue::DbNull).unwrap().is_none());
    }

    #[test]
    fn uint64() {
        assert_eq!(u64::decode(&DbValue::Uint64(0)).unwrap(), 0);
        assert!(u64::decode(&DbValue::Boolean(false)).is_err());
        assert!(Option::<u64>::decode(&DbValue::DbNull).unwrap().is_none());
    }

    #[test]
    fn floating32() {
        assert!(f32::decode(&DbValue::Floating32(0.0)).is_ok());
        assert!(f32::decode(&DbValue::Boolean(false)).is_err());
        assert!(Option::<f32>::decode(&DbValue::DbNull).unwrap().is_none());
    }

    #[test]
    fn floating64() {
        assert!(f64::decode(&DbValue::Floating64(0.0)).is_ok());
        assert!(f64::decode(&DbValue::Boolean(false)).is_err());
        assert!(Option::<f64>::decode(&DbValue::DbNull).unwrap().is_none());
    }

    #[test]
    fn str() {
        assert_eq!(
            String::decode(&DbValue::Str(String::from("foo"))).unwrap(),
            String::from("foo")
        );

        assert!(String::decode(&DbValue::Int32(0)).is_err());
        assert!(Option::<String>::decode(&DbValue::DbNull)
            .unwrap()
            .is_none());
    }
}
