use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{Visitor, VariantAccess};
use serde::de;

use grimoire2::theoretical::Theoretical;


#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TheoreticalWrapper {    
    Known(f64),
    Theory(f64),
    Unknown,
}


impl Default for TheoreticalWrapper {
    fn default() -> Self {
        TheoreticalWrapper::Unknown
    }
}

impl TheoreticalWrapper {
    pub fn to_theoretical(self, default: f64) -> Theoretical<f64> {
        match self {
            Self::Known(x) => Theoretical::Known(x),
            Self::Theory(x) => Theoretical::Theory(x),
            Self::Unknown => Theoretical::Unknown,
        }
    }
}


impl From<Theoretical<f64>> for TheoreticalWrapper {
    fn from(value: Theoretical<f64>) -> Self {
        match value {
            Theoretical::Known(x) => TheoreticalWrapper::Known(x),
            Theoretical::Theory(x) => TheoreticalWrapper::Theory(x),
            Theoretical::Unknown => TheoreticalWrapper::Unknown,
        }
    }
}


impl From<TheoreticalWrapper> for Theoretical<f64> {
    fn from(value: TheoreticalWrapper) -> Self {
        match value {
            TheoreticalWrapper::Known(x) => Theoretical::Known(x),
            TheoreticalWrapper::Theory(x) => Theoretical::Theory(x),
            TheoreticalWrapper::Unknown => Theoretical::Unknown,
        }
    }
}

impl Serialize for TheoreticalWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TheoreticalWrapper::Known(value) => value.serialize(serializer),
            TheoreticalWrapper::Theory(value) => {
                serializer.serialize_newtype_variant("TheoreticalWrapper", 1, "?", value)
            },
            TheoreticalWrapper::Unknown => {
                serializer.serialize_unit_variant("TheoreticalWrapper", 2, "??")
            }
        }
    }
}


impl<'de> Deserialize<'de> for TheoreticalWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TheoreticalWrapperVisitor;

        impl<'de> Visitor<'de> for TheoreticalWrapperVisitor {
            type Value = TheoreticalWrapper;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a float value, ?? for default, !! <value> for known value or !? <value> for unknown value")
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
                Ok(TheoreticalWrapper::Known(value))
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
                where
                    E: de::Error, {
                Ok(TheoreticalWrapper::Known(v as f64))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                    E: de::Error, {
                Ok(TheoreticalWrapper::Known(v as f64))
            }

            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
                where
                    E: de::Error, {
                Ok(TheoreticalWrapper::Known(v as f64))
            }


            fn visit_i128<E>(self,v:i128) -> Result<Self::Value,E>where E:de::Error, {
                Ok(TheoreticalWrapper::Known(v as f64))
            }

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
                where
                    E: de::Error, {
                Ok(TheoreticalWrapper::Known(v as f64))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                where
                    E: de::Error, {
                Ok(TheoreticalWrapper::Known(v as f64))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: de::Error, {
                match v {
                    "??" => Ok(TheoreticalWrapper::Unknown),
                    _ => Err(de::Error::custom(format!("Unknown value: {}", v)))
                }
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
                where
                    A: de::EnumAccess<'de>, {

                let (variant, values): (String, _) = data.variant()?;

                match variant.as_str() {
                    "?" => Ok(
                        values.newtype_variant().map(TheoreticalWrapper::Theory)
                    )?,      
                    "!" => Ok(values.newtype_variant().map(TheoreticalWrapper::Known))?,
                    _ => Err(de::Error::unknown_variant(&variant, &["!", "?"]))
                }
                
            }

        }

        deserializer.deserialize_any(TheoreticalWrapperVisitor)
    }
}


#[cfg(test)]
mod tests {
    use grimoire2::theoretical::Theoretical;
    use serde_yaml::{to_string, from_str};

    use super::TheoreticalWrapper;

    #[test]
    fn test_serialize_known_f64() {
        let value: TheoreticalWrapper = Theoretical::Known(0.5).into();
        let expected = "0.5\n";
        let actual = to_string(&value).unwrap();
        assert_eq!(actual, expected, "Serialization result: '{:?}'", actual);
    }

    #[test]
    fn test_serialize_unknown_f64() {
        let value: TheoreticalWrapper = Theoretical::Theory(0.5).into();
        let expected = "!? 0.5\n";
        let actual = to_string(&value).unwrap();
        assert_eq!(actual, expected, "Serialization result: '{:?}'", actual);
    }

    #[test]
    fn test_serialize_default() {
        let value: TheoreticalWrapper = TheoreticalWrapper::Unknown;
        let expected = "??\n";
        let actual = to_string(&value).unwrap();
        assert_eq!(actual, expected, "Serialization result: '{:?}'", actual);
    }    

    #[test]
    fn test_deserialize_known_f64() {
        let input = "0.5\n";
        let expected: Theoretical<f64> = Theoretical::Known(0.5);
        let actual = from_str::<TheoreticalWrapper>(input).unwrap().to_theoretical(0.);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_unknown_f64() {
        let input = "!? 0.5\n";
        let expected: Theoretical<f64> = Theoretical::Theory(0.5);
        let actual = from_str::<TheoreticalWrapper>(input).unwrap().to_theoretical(0.);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_default() {
        let input = "??\n";
        let expected: Theoretical<f64> = Theoretical::Unknown;
        let actual = from_str::<TheoreticalWrapper>(input).unwrap().to_theoretical(0.);
        assert_eq!(expected, actual);
    }
}