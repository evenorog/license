use crate::{Exception, License};
use core::fmt;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for dyn License {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.id())
    }
}

impl Serialize for dyn Exception {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.id())
    }
}

struct LicenseVisitor;

impl<'de> Visitor<'de> for LicenseVisitor {
    type Value = &'static dyn License;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an SPDX license id")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        value.parse().map_err(Error::custom)
    }
}

impl<'de> Deserialize<'de> for &dyn License {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(LicenseVisitor)
    }
}

struct ExceptionVisitor;

impl<'de> Visitor<'de> for ExceptionVisitor {
    type Value = &'static dyn Exception;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an SPDX exception id")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        value.parse().map_err(Error::custom)
    }
}

impl<'de> Deserialize<'de> for &dyn Exception {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ExceptionVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::License;

    #[test]
    fn serde() {
        let mit: &dyn License = "MIT".parse().unwrap();
        let s = serde_json::to_string(&mit).unwrap();
        assert_eq!(s, "\"MIT\"");
        let _: &dyn License = serde_json::from_str(&s).unwrap();
    }
}
