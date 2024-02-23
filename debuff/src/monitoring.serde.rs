// @generated
impl serde::Serialize for DnsCheckResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.header.is_some() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        if !self.lookups.is_empty() {
            len += 1;
        }
        if !self.node.is_empty() {
            len += 1;
        }
        if !self.date_sent.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("monitoring.DNSCheckResult", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.lookups.is_empty() {
            struct_ser.serialize_field("lookups", &self.lookups)?;
        }
        if !self.node.is_empty() {
            struct_ser.serialize_field("node", &self.node)?;
        }
        if !self.date_sent.is_empty() {
            struct_ser.serialize_field("date_sent", &self.date_sent)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DnsCheckResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "lookups",
            "node",
            "date_sent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            Lookups,
            Node,
            DateSent,
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
                            "header" => Ok(GeneratedField::Header),
                            "error" => Ok(GeneratedField::Error),
                            "lookups" => Ok(GeneratedField::Lookups),
                            "node" => Ok(GeneratedField::Node),
                            "date_sent" => Ok(GeneratedField::DateSent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DnsCheckResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct monitoring.DNSCheckResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DnsCheckResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut lookups__ = None;
                let mut node__ = None;
                let mut date_sent__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
                        }
                        GeneratedField::Lookups => {
                            if lookups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lookups"));
                            }
                            lookups__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DateSent => {
                            if date_sent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("date_sent"));
                            }
                            date_sent__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DnsCheckResult {
                    header: header__,
                    error: error__,
                    lookups: lookups__.unwrap_or_default(),
                    node: node__.unwrap_or_default(),
                    date_sent: date_sent__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("monitoring.DNSCheckResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DnsLookup {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.host.is_empty() {
            len += 1;
        }
        if !self.addresses.is_empty() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.error.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("monitoring.DNSLookup", len)?;
        if !self.host.is_empty() {
            struct_ser.serialize_field("host", &self.host)?;
        }
        if !self.addresses.is_empty() {
            struct_ser.serialize_field("addresses", &self.addresses)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.error.is_empty() {
            struct_ser.serialize_field("error", &self.error)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DnsLookup {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "host",
            "addresses",
            "type",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Host,
            Addresses,
            Type,
            Error,
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
                            "host" => Ok(GeneratedField::Host),
                            "addresses" => Ok(GeneratedField::Addresses),
                            "type" => Ok(GeneratedField::Type),
                            "error" => Ok(GeneratedField::Error),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DnsLookup;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct monitoring.DNSLookup")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DnsLookup, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut host__ = None;
                let mut addresses__ = None;
                let mut r#type__ = None;
                let mut error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Host => {
                            if host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            host__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Addresses => {
                            if addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addresses"));
                            }
                            addresses__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DnsLookup {
                    host: host__.unwrap_or_default(),
                    addresses: addresses__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    error: error__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("monitoring.DNSLookup", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Heartbeat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.header.is_some() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        if !self.node.is_empty() {
            len += 1;
        }
        if !self.date_sent.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("monitoring.Heartbeat", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.node.is_empty() {
            struct_ser.serialize_field("node", &self.node)?;
        }
        if !self.date_sent.is_empty() {
            struct_ser.serialize_field("date_sent", &self.date_sent)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Heartbeat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "node",
            "date_sent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            Node,
            DateSent,
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
                            "header" => Ok(GeneratedField::Header),
                            "error" => Ok(GeneratedField::Error),
                            "node" => Ok(GeneratedField::Node),
                            "date_sent" => Ok(GeneratedField::DateSent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Heartbeat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct monitoring.Heartbeat")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Heartbeat, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut node__ = None;
                let mut date_sent__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
                        }
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DateSent => {
                            if date_sent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("date_sent"));
                            }
                            date_sent__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Heartbeat {
                    header: header__,
                    error: error__,
                    node: node__.unwrap_or_default(),
                    date_sent: date_sent__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("monitoring.Heartbeat", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LookupType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::UnsetLookup => "UNSET_LOOKUP",
            Self::InternalLookup => "INTERNAL_LOOKUP",
            Self::ExternalLookup => "EXTERNAL_LOOKUP",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for LookupType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNSET_LOOKUP",
            "INTERNAL_LOOKUP",
            "EXTERNAL_LOOKUP",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LookupType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNSET_LOOKUP" => Ok(LookupType::UnsetLookup),
                    "INTERNAL_LOOKUP" => Ok(LookupType::InternalLookup),
                    "EXTERNAL_LOOKUP" => Ok(LookupType::ExternalLookup),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
