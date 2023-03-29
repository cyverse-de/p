// @generated
impl serde::Serialize for App {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.wiki_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("apps.App", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.wiki_url.is_empty() {
            struct_ser.serialize_field("wiki_url", &self.wiki_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for App {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "description",
            "wiki_url",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Description,
            WikiUrl,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "wiki_url" => Ok(GeneratedField::WikiUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = App;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct apps.App")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<App, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut wiki_url__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::WikiUrl => {
                            if wiki_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wiki_url"));
                            }
                            wiki_url__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(App {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    wiki_url: wiki_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("apps.App", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AppVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.app_id.is_empty() {
            len += 1;
        }
        if !self.version.is_empty() {
            len += 1;
        }
        if self.version_order != 0 {
            len += 1;
        }
        if self.deleted {
            len += 1;
        }
        if self.disabled {
            len += 1;
        }
        if self.integration.is_some() {
            len += 1;
        }
        if self.integration_date.is_some() {
            len += 1;
        }
        if self.edited_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("apps.AppVersion", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.app_id.is_empty() {
            struct_ser.serialize_field("app_id", &self.app_id)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if self.version_order != 0 {
            struct_ser.serialize_field("version_order", ToString::to_string(&self.version_order).as_str())?;
        }
        if self.deleted {
            struct_ser.serialize_field("deleted", &self.deleted)?;
        }
        if self.disabled {
            struct_ser.serialize_field("disabled", &self.disabled)?;
        }
        if let Some(v) = self.integration.as_ref() {
            struct_ser.serialize_field("integration", v)?;
        }
        if let Some(v) = self.integration_date.as_ref() {
            struct_ser.serialize_field("integration_date", v)?;
        }
        if let Some(v) = self.edited_date.as_ref() {
            struct_ser.serialize_field("edited_date", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AppVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "app_id",
            "version",
            "version_order",
            "deleted",
            "disabled",
            "integration",
            "integration_date",
            "edited_date",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            AppId,
            Version,
            VersionOrder,
            Deleted,
            Disabled,
            Integration,
            IntegrationDate,
            EditedDate,
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
                            "id" => Ok(GeneratedField::Id),
                            "app_id" => Ok(GeneratedField::AppId),
                            "version" => Ok(GeneratedField::Version),
                            "version_order" => Ok(GeneratedField::VersionOrder),
                            "deleted" => Ok(GeneratedField::Deleted),
                            "disabled" => Ok(GeneratedField::Disabled),
                            "integration" => Ok(GeneratedField::Integration),
                            "integration_date" => Ok(GeneratedField::IntegrationDate),
                            "edited_date" => Ok(GeneratedField::EditedDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AppVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct apps.AppVersion")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AppVersion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut app_id__ = None;
                let mut version__ = None;
                let mut version_order__ = None;
                let mut deleted__ = None;
                let mut disabled__ = None;
                let mut integration__ = None;
                let mut integration_date__ = None;
                let mut edited_date__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::AppId => {
                            if app_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app_id"));
                            }
                            app_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map.next_value()?);
                        }
                        GeneratedField::VersionOrder => {
                            if version_order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version_order"));
                            }
                            version_order__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Deleted => {
                            if deleted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleted"));
                            }
                            deleted__ = Some(map.next_value()?);
                        }
                        GeneratedField::Disabled => {
                            if disabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disabled"));
                            }
                            disabled__ = Some(map.next_value()?);
                        }
                        GeneratedField::Integration => {
                            if integration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integration"));
                            }
                            integration__ = map.next_value()?;
                        }
                        GeneratedField::IntegrationDate => {
                            if integration_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integration_date"));
                            }
                            integration_date__ = map.next_value()?;
                        }
                        GeneratedField::EditedDate => {
                            if edited_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("edited_date"));
                            }
                            edited_date__ = map.next_value()?;
                        }
                    }
                }
                Ok(AppVersion {
                    id: id__.unwrap_or_default(),
                    app_id: app_id__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    version_order: version_order__.unwrap_or_default(),
                    deleted: deleted__.unwrap_or_default(),
                    disabled: disabled__.unwrap_or_default(),
                    integration: integration__,
                    integration_date: integration_date__,
                    edited_date: edited_date__,
                })
            }
        }
        deserializer.deserialize_struct("apps.AppVersion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IntegrationData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.integrator_name.is_empty() {
            len += 1;
        }
        if !self.integrator_email.is_empty() {
            len += 1;
        }
        if self.user.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("apps.IntegrationData", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.integrator_name.is_empty() {
            struct_ser.serialize_field("integrator_name", &self.integrator_name)?;
        }
        if !self.integrator_email.is_empty() {
            struct_ser.serialize_field("integrator_email", &self.integrator_email)?;
        }
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IntegrationData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "integrator_name",
            "integrator_email",
            "user",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            IntegratorName,
            IntegratorEmail,
            User,
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
                            "id" => Ok(GeneratedField::Id),
                            "integrator_name" => Ok(GeneratedField::IntegratorName),
                            "integrator_email" => Ok(GeneratedField::IntegratorEmail),
                            "user" => Ok(GeneratedField::User),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IntegrationData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct apps.IntegrationData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IntegrationData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut integrator_name__ = None;
                let mut integrator_email__ = None;
                let mut user__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::IntegratorName => {
                            if integrator_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integrator_name"));
                            }
                            integrator_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::IntegratorEmail => {
                            if integrator_email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integrator_email"));
                            }
                            integrator_email__ = Some(map.next_value()?);
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map.next_value()?;
                        }
                    }
                }
                Ok(IntegrationData {
                    id: id__.unwrap_or_default(),
                    integrator_name: integrator_name__.unwrap_or_default(),
                    integrator_email: integrator_email__.unwrap_or_default(),
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("apps.IntegrationData", FIELDS, GeneratedVisitor)
    }
}
