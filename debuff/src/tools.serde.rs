// @generated
impl serde::Serialize for Tool {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.uuid.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.version.is_empty() {
            len += 1;
        }
        if !self.attribution.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.time_limit_seconds != 0 {
            len += 1;
        }
        if self.restricted {
            len += 1;
        }
        if self.interactive {
            len += 1;
        }
        if self.gpu_enabled {
            len += 1;
        }
        if self.integration_data.is_some() {
            len += 1;
        }
        if self.container_image.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tools.Tool", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.attribution.is_empty() {
            struct_ser.serialize_field("attribution", &self.attribution)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.time_limit_seconds != 0 {
            struct_ser.serialize_field("time_limit_seconds", &self.time_limit_seconds)?;
        }
        if self.restricted {
            struct_ser.serialize_field("restricted", &self.restricted)?;
        }
        if self.interactive {
            struct_ser.serialize_field("interactive", &self.interactive)?;
        }
        if self.gpu_enabled {
            struct_ser.serialize_field("gpu_enabled", &self.gpu_enabled)?;
        }
        if let Some(v) = self.integration_data.as_ref() {
            struct_ser.serialize_field("integration_data", v)?;
        }
        if let Some(v) = self.container_image.as_ref() {
            struct_ser.serialize_field("container_image", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Tool {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uuid",
            "name",
            "version",
            "attribution",
            "description",
            "time_limit_seconds",
            "restricted",
            "interactive",
            "gpu_enabled",
            "integration_data",
            "container_image",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            Name,
            Version,
            Attribution,
            Description,
            TimeLimitSeconds,
            Restricted,
            Interactive,
            GpuEnabled,
            IntegrationData,
            ContainerImage,
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
                            "uuid" => Ok(GeneratedField::Uuid),
                            "name" => Ok(GeneratedField::Name),
                            "version" => Ok(GeneratedField::Version),
                            "attribution" => Ok(GeneratedField::Attribution),
                            "description" => Ok(GeneratedField::Description),
                            "time_limit_seconds" => Ok(GeneratedField::TimeLimitSeconds),
                            "restricted" => Ok(GeneratedField::Restricted),
                            "interactive" => Ok(GeneratedField::Interactive),
                            "gpu_enabled" => Ok(GeneratedField::GpuEnabled),
                            "integration_data" => Ok(GeneratedField::IntegrationData),
                            "container_image" => Ok(GeneratedField::ContainerImage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Tool;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tools.Tool")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Tool, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut name__ = None;
                let mut version__ = None;
                let mut attribution__ = None;
                let mut description__ = None;
                let mut time_limit_seconds__ = None;
                let mut restricted__ = None;
                let mut interactive__ = None;
                let mut gpu_enabled__ = None;
                let mut integration_data__ = None;
                let mut container_image__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map.next_value()?);
                        }
                        GeneratedField::Attribution => {
                            if attribution__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attribution"));
                            }
                            attribution__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::TimeLimitSeconds => {
                            if time_limit_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time_limit_seconds"));
                            }
                            time_limit_seconds__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Restricted => {
                            if restricted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restricted"));
                            }
                            restricted__ = Some(map.next_value()?);
                        }
                        GeneratedField::Interactive => {
                            if interactive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interactive"));
                            }
                            interactive__ = Some(map.next_value()?);
                        }
                        GeneratedField::GpuEnabled => {
                            if gpu_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gpu_enabled"));
                            }
                            gpu_enabled__ = Some(map.next_value()?);
                        }
                        GeneratedField::IntegrationData => {
                            if integration_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integration_data"));
                            }
                            integration_data__ = map.next_value()?;
                        }
                        GeneratedField::ContainerImage => {
                            if container_image__.is_some() {
                                return Err(serde::de::Error::duplicate_field("container_image"));
                            }
                            container_image__ = map.next_value()?;
                        }
                    }
                }
                Ok(Tool {
                    uuid: uuid__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    attribution: attribution__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    time_limit_seconds: time_limit_seconds__.unwrap_or_default(),
                    restricted: restricted__.unwrap_or_default(),
                    interactive: interactive__.unwrap_or_default(),
                    gpu_enabled: gpu_enabled__.unwrap_or_default(),
                    integration_data: integration_data__,
                    container_image: container_image__,
                })
            }
        }
        deserializer.deserialize_struct("tools.Tool", FIELDS, GeneratedVisitor)
    }
}
