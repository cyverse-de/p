// @generated
impl serde::Serialize for Header {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.map.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("header.Header", len)?;
        if !self.map.is_empty() {
            struct_ser.serialize_field("map", &self.map)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Header {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "map",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Map,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "map" => Ok(GeneratedField::Map),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Header;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct header.Header")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Header, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut map__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Map => {
                            if map__.is_some() {
                                return Err(serde::de::Error::duplicate_field("map"));
                            }
                            map__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(Header {
                    map: map__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("header.Header", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for header::Value {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("header.Header.Value", len)?;
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for header::Value {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = header::Value;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct header.Header.Value")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<header::Value, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(header::Value {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("header.Header.Value", FIELDS, GeneratedVisitor)
    }
}
