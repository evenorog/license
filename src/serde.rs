use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{Visitor, Error};
use crate::{License, Exception};
use core::fmt;

impl Serialize for &dyn License {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.id())
    }
}

impl Serialize for &dyn Exception {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.id())
    }
}

struct LicenseVisitor;

impl<'de> Visitor<'de> for LicenseVisitor {
    type Value = &'static dyn License;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an SPDX license id")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: Error {
        value.parse().map_err(Error::custom)
    }
}

impl<'de> Deserialize<'de> for &dyn License {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(LicenseVisitor)
    }
}

struct ExceptionVisitor;

impl<'de> Visitor<'de> for ExceptionVisitor {
    type Value = &'static dyn Exception;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an SPDX exception id")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: Error {
        value.parse().map_err(Error::custom)
    }
}

impl<'de> Deserialize<'de> for &dyn Exception {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(ExceptionVisitor)
    }
}
