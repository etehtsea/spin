wit_bindgen_rust::import!("../../wit/ephemeral/outbound-pg.wit");

/// Exports the generated outbound Pg items.
pub use outbound_pg::*;

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
            DbValue::Boolean(boolean) => Ok(*boolean),
            _ => Err(anyhow::anyhow!(
                "Expected BOOL from the DB but got {:?}",
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

impl Decode for f32 {
    fn decode(value: &DbValue) -> Result<Self, anyhow::Error> {
        match value {
            DbValue::Floating32(n) => Ok(*n),
            _ => Err(anyhow::anyhow!(
                "Expected REAL from the DB but got {:?}",
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
                "Expected DOUBLE PRECISION from the DB but got {:?}",
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
                "Expected BYTEA from the DB but got {:?}",
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
        assert!(bool::decode(&DbValue::Boolean(true)).unwrap());
        assert!(bool::decode(&DbValue::Int32(0)).is_err());
        assert!(Option::<bool>::decode(&DbValue::DbNull).unwrap().is_none());
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

    #[test]
    fn binary() {
        assert!(Vec::<u8>::decode(&DbValue::Binary(vec![0, 0])).is_ok());
        assert!(Vec::<u8>::decode(&DbValue::Boolean(false)).is_err());
        assert!(Option::<Vec<u8>>::decode(&DbValue::DbNull)
            .unwrap()
            .is_none());
    }
}
