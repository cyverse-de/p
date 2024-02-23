// @generated
impl serde::Serialize for AddAddonRequest {
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
        if self.addon.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.AddAddonRequest", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.addon.as_ref() {
            struct_ser.serialize_field("addon", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddAddonRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "addon",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Addon,
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
                            "addon" => Ok(GeneratedField::Addon),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddAddonRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.AddAddonRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddAddonRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut addon__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Addon => {
                            if addon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addon"));
                            }
                            addon__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddAddonRequest {
                    header: header__,
                    addon: addon__,
                })
            }
        }
        deserializer.deserialize_struct("qms.AddAddonRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddPlanQuotaDefaultRequest {
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
        if !self.plan_name.is_empty() {
            len += 1;
        }
        if self.quota_default.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.AddPlanQuotaDefaultRequest", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.plan_name.is_empty() {
            struct_ser.serialize_field("plan_name", &self.plan_name)?;
        }
        if let Some(v) = self.quota_default.as_ref() {
            struct_ser.serialize_field("quota_default", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddPlanQuotaDefaultRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "plan_name",
            "quota_default",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            PlanName,
            QuotaDefault,
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
                            "plan_name" => Ok(GeneratedField::PlanName),
                            "quota_default" => Ok(GeneratedField::QuotaDefault),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddPlanQuotaDefaultRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.AddPlanQuotaDefaultRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddPlanQuotaDefaultRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut plan_name__ = None;
                let mut quota_default__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::PlanName => {
                            if plan_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plan_name"));
                            }
                            plan_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuotaDefault => {
                            if quota_default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota_default"));
                            }
                            quota_default__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddPlanQuotaDefaultRequest {
                    header: header__,
                    plan_name: plan_name__.unwrap_or_default(),
                    quota_default: quota_default__,
                })
            }
        }
        deserializer.deserialize_struct("qms.AddPlanQuotaDefaultRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddPlanRequest {
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
        if self.plan.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.AddPlanRequest", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.plan.as_ref() {
            struct_ser.serialize_field("plan", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddPlanRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "plan",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Plan,
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
                            "plan" => Ok(GeneratedField::Plan),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddPlanRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.AddPlanRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddPlanRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut plan__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Plan => {
                            if plan__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plan"));
                            }
                            plan__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddPlanRequest {
                    header: header__,
                    plan: plan__,
                })
            }
        }
        deserializer.deserialize_struct("qms.AddPlanRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddQuotaRequest {
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
        if self.quota.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.AddQuotaRequest", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.quota.as_ref() {
            struct_ser.serialize_field("quota", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddQuotaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "quota",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Quota,
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
                            "quota" => Ok(GeneratedField::Quota),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddQuotaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.AddQuotaRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddQuotaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut quota__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Quota => {
                            if quota__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota"));
                            }
                            quota__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddQuotaRequest {
                    header: header__,
                    quota: quota__,
                })
            }
        }
        deserializer.deserialize_struct("qms.AddQuotaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddUpdateRequest {
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
        if self.update.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.AddUpdateRequest", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.update.as_ref() {
            struct_ser.serialize_field("update", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddUpdateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "update",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Update,
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
                            "update" => Ok(GeneratedField::Update),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddUpdateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.AddUpdateRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddUpdateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut update__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Update => {
                            if update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update"));
                            }
                            update__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddUpdateRequest {
                    header: header__,
                    update: update__,
                })
            }
        }
        deserializer.deserialize_struct("qms.AddUpdateRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddUpdateResponse {
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
        if self.update.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.AddUpdateResponse", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if let Some(v) = self.update.as_ref() {
            struct_ser.serialize_field("update", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddUpdateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "update",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            Update,
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
                            "update" => Ok(GeneratedField::Update),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddUpdateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.AddUpdateResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddUpdateResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut update__ = None;
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
                        GeneratedField::Update => {
                            if update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update"));
                            }
                            update__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddUpdateResponse {
                    header: header__,
                    error: error__,
                    update: update__,
                })
            }
        }
        deserializer.deserialize_struct("qms.AddUpdateResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddUsage {
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
        if !self.username.is_empty() {
            len += 1;
        }
        if !self.resource_name.is_empty() {
            len += 1;
        }
        if !self.update_type.is_empty() {
            len += 1;
        }
        if self.usage_value != 0. {
            len += 1;
        }
        if !self.resource_unit.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.AddUsage", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if !self.resource_name.is_empty() {
            struct_ser.serialize_field("resource_name", &self.resource_name)?;
        }
        if !self.update_type.is_empty() {
            struct_ser.serialize_field("update_type", &self.update_type)?;
        }
        if self.usage_value != 0. {
            struct_ser.serialize_field("usage_value", &self.usage_value)?;
        }
        if !self.resource_unit.is_empty() {
            struct_ser.serialize_field("resource_unit", &self.resource_unit)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddUsage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "username",
            "resource_name",
            "update_type",
            "usage_value",
            "resource_unit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Username,
            ResourceName,
            UpdateType,
            UsageValue,
            ResourceUnit,
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
                            "username" => Ok(GeneratedField::Username),
                            "resource_name" => Ok(GeneratedField::ResourceName),
                            "update_type" => Ok(GeneratedField::UpdateType),
                            "usage_value" => Ok(GeneratedField::UsageValue),
                            "resource_unit" => Ok(GeneratedField::ResourceUnit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddUsage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.AddUsage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddUsage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut username__ = None;
                let mut resource_name__ = None;
                let mut update_type__ = None;
                let mut usage_value__ = None;
                let mut resource_unit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceName => {
                            if resource_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_name"));
                            }
                            resource_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdateType => {
                            if update_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update_type"));
                            }
                            update_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UsageValue => {
                            if usage_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usage_value"));
                            }
                            usage_value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResourceUnit => {
                            if resource_unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_unit"));
                            }
                            resource_unit__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AddUsage {
                    header: header__,
                    username: username__.unwrap_or_default(),
                    resource_name: resource_name__.unwrap_or_default(),
                    update_type: update_type__.unwrap_or_default(),
                    usage_value: usage_value__.unwrap_or_default(),
                    resource_unit: resource_unit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.AddUsage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddUserRequest {
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
        if !self.username.is_empty() {
            len += 1;
        }
        if !self.plan_name.is_empty() {
            len += 1;
        }
        if self.paid {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.AddUserRequest", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if !self.plan_name.is_empty() {
            struct_ser.serialize_field("planName", &self.plan_name)?;
        }
        if self.paid {
            struct_ser.serialize_field("paid", &self.paid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddUserRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "username",
            "plan_name",
            "planName",
            "paid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Username,
            PlanName,
            Paid,
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
                            "username" => Ok(GeneratedField::Username),
                            "planName" | "plan_name" => Ok(GeneratedField::PlanName),
                            "paid" => Ok(GeneratedField::Paid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddUserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.AddUserRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddUserRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut username__ = None;
                let mut plan_name__ = None;
                let mut paid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PlanName => {
                            if plan_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("planName"));
                            }
                            plan_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Paid => {
                            if paid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paid"));
                            }
                            paid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AddUserRequest {
                    header: header__,
                    username: username__.unwrap_or_default(),
                    plan_name: plan_name__.unwrap_or_default(),
                    paid: paid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.AddUserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddUserResponse {
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
        if !self.uuid.is_empty() {
            len += 1;
        }
        if !self.username.is_empty() {
            len += 1;
        }
        if !self.plan_name.is_empty() {
            len += 1;
        }
        if !self.plan_uuid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.AddUserResponse", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if !self.plan_name.is_empty() {
            struct_ser.serialize_field("planName", &self.plan_name)?;
        }
        if !self.plan_uuid.is_empty() {
            struct_ser.serialize_field("planUuid", &self.plan_uuid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddUserResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "uuid",
            "username",
            "plan_name",
            "planName",
            "plan_uuid",
            "planUuid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            Uuid,
            Username,
            PlanName,
            PlanUuid,
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
                            "uuid" => Ok(GeneratedField::Uuid),
                            "username" => Ok(GeneratedField::Username),
                            "planName" | "plan_name" => Ok(GeneratedField::PlanName),
                            "planUuid" | "plan_uuid" => Ok(GeneratedField::PlanUuid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddUserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.AddUserResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddUserResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut uuid__ = None;
                let mut username__ = None;
                let mut plan_name__ = None;
                let mut plan_uuid__ = None;
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
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PlanName => {
                            if plan_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("planName"));
                            }
                            plan_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PlanUuid => {
                            if plan_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("planUuid"));
                            }
                            plan_uuid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AddUserResponse {
                    header: header__,
                    error: error__,
                    uuid: uuid__.unwrap_or_default(),
                    username: username__.unwrap_or_default(),
                    plan_name: plan_name__.unwrap_or_default(),
                    plan_uuid: plan_uuid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.AddUserResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Addon {
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
        if !self.description.is_empty() {
            len += 1;
        }
        if self.resource_type.is_some() {
            len += 1;
        }
        if self.default_amount != 0. {
            len += 1;
        }
        if self.default_paid {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.Addon", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.resource_type.as_ref() {
            struct_ser.serialize_field("resource_type", v)?;
        }
        if self.default_amount != 0. {
            struct_ser.serialize_field("default_amount", &self.default_amount)?;
        }
        if self.default_paid {
            struct_ser.serialize_field("default_paid", &self.default_paid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Addon {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uuid",
            "name",
            "description",
            "resource_type",
            "default_amount",
            "default_paid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            Name,
            Description,
            ResourceType,
            DefaultAmount,
            DefaultPaid,
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
                            "description" => Ok(GeneratedField::Description),
                            "resource_type" => Ok(GeneratedField::ResourceType),
                            "default_amount" => Ok(GeneratedField::DefaultAmount),
                            "default_paid" => Ok(GeneratedField::DefaultPaid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Addon;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.Addon")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Addon, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut resource_type__ = None;
                let mut default_amount__ = None;
                let mut default_paid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_type"));
                            }
                            resource_type__ = map_.next_value()?;
                        }
                        GeneratedField::DefaultAmount => {
                            if default_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("default_amount"));
                            }
                            default_amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DefaultPaid => {
                            if default_paid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("default_paid"));
                            }
                            default_paid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Addon {
                    uuid: uuid__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    resource_type: resource_type__,
                    default_amount: default_amount__.unwrap_or_default(),
                    default_paid: default_paid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.Addon", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddonListResponse {
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
        if !self.addons.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.AddonListResponse", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.addons.is_empty() {
            struct_ser.serialize_field("addons", &self.addons)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddonListResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "addons",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            Addons,
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
                            "addons" => Ok(GeneratedField::Addons),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddonListResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.AddonListResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddonListResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut addons__ = None;
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
                        GeneratedField::Addons => {
                            if addons__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addons"));
                            }
                            addons__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AddonListResponse {
                    header: header__,
                    error: error__,
                    addons: addons__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.AddonListResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddonLookupRequest {
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
        if self.addon.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.AddonLookupRequest", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.addon.as_ref() {
            match v {
                addon_lookup_request::Addon::Uuid(v) => {
                    struct_ser.serialize_field("uuid", v)?;
                }
                addon_lookup_request::Addon::Name(v) => {
                    struct_ser.serialize_field("name", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddonLookupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "uuid",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Uuid,
            Name,
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
                            "uuid" => Ok(GeneratedField::Uuid),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddonLookupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.AddonLookupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddonLookupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut addon__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Uuid => {
                            if addon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            addon__ = map_.next_value::<::std::option::Option<_>>()?.map(addon_lookup_request::Addon::Uuid);
                        }
                        GeneratedField::Name => {
                            if addon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            addon__ = map_.next_value::<::std::option::Option<_>>()?.map(addon_lookup_request::Addon::Name);
                        }
                    }
                }
                Ok(AddonLookupRequest {
                    header: header__,
                    addon: addon__,
                })
            }
        }
        deserializer.deserialize_struct("qms.AddonLookupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddonResponse {
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
        if self.addon.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.AddonResponse", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if let Some(v) = self.addon.as_ref() {
            struct_ser.serialize_field("addon", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddonResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "addon",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            Addon,
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
                            "addon" => Ok(GeneratedField::Addon),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddonResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.AddonResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddonResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut addon__ = None;
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
                        GeneratedField::Addon => {
                            if addon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addon"));
                            }
                            addon__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddonResponse {
                    header: header__,
                    error: error__,
                    addon: addon__,
                })
            }
        }
        deserializer.deserialize_struct("qms.AddonResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AllUserOveragesRequest {
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
        if !self.username.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.AllUserOveragesRequest", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AllUserOveragesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "username",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Username,
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
                            "username" => Ok(GeneratedField::Username),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AllUserOveragesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.AllUserOveragesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AllUserOveragesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut username__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AllUserOveragesRequest {
                    header: header__,
                    username: username__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.AllUserOveragesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChangeSubscriptionRequest {
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
        if !self.username.is_empty() {
            len += 1;
        }
        if self.periods != 0 {
            len += 1;
        }
        if !self.end_date.is_empty() {
            len += 1;
        }
        if self.plan.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.ChangeSubscriptionRequest", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if self.periods != 0 {
            struct_ser.serialize_field("periods", &self.periods)?;
        }
        if !self.end_date.is_empty() {
            struct_ser.serialize_field("endDate", &self.end_date)?;
        }
        if let Some(v) = self.plan.as_ref() {
            match v {
                change_subscription_request::Plan::Uuid(v) => {
                    struct_ser.serialize_field("uuid", v)?;
                }
                change_subscription_request::Plan::Name(v) => {
                    struct_ser.serialize_field("name", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChangeSubscriptionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "username",
            "periods",
            "end_date",
            "endDate",
            "uuid",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Username,
            Periods,
            EndDate,
            Uuid,
            Name,
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
                            "username" => Ok(GeneratedField::Username),
                            "periods" => Ok(GeneratedField::Periods),
                            "endDate" | "end_date" => Ok(GeneratedField::EndDate),
                            "uuid" => Ok(GeneratedField::Uuid),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChangeSubscriptionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.ChangeSubscriptionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChangeSubscriptionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut username__ = None;
                let mut periods__ = None;
                let mut end_date__ = None;
                let mut plan__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Periods => {
                            if periods__.is_some() {
                                return Err(serde::de::Error::duplicate_field("periods"));
                            }
                            periods__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EndDate => {
                            if end_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endDate"));
                            }
                            end_date__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Uuid => {
                            if plan__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            plan__ = map_.next_value::<::std::option::Option<_>>()?.map(change_subscription_request::Plan::Uuid);
                        }
                        GeneratedField::Name => {
                            if plan__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            plan__ = map_.next_value::<::std::option::Option<_>>()?.map(change_subscription_request::Plan::Name);
                        }
                    }
                }
                Ok(ChangeSubscriptionRequest {
                    header: header__,
                    username: username__.unwrap_or_default(),
                    periods: periods__.unwrap_or_default(),
                    end_date: end_date__.unwrap_or_default(),
                    plan: plan__,
                })
            }
        }
        deserializer.deserialize_struct("qms.ChangeSubscriptionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUsages {
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
        if !self.username.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.GetUsages", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUsages {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "username",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Username,
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
                            "username" => Ok(GeneratedField::Username),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUsages;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.GetUsages")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUsages, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut username__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetUsages {
                    header: header__,
                    username: username__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.GetUsages", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IsOverage {
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
        if self.is_overage {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.IsOverage", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if self.is_overage {
            struct_ser.serialize_field("is_overage", &self.is_overage)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IsOverage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "is_overage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            IsOverage,
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
                            "is_overage" => Ok(GeneratedField::IsOverage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IsOverage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.IsOverage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IsOverage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut is_overage__ = None;
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
                        GeneratedField::IsOverage => {
                            if is_overage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("is_overage"));
                            }
                            is_overage__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(IsOverage {
                    header: header__,
                    error: error__,
                    is_overage: is_overage__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.IsOverage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IsOverageRequest {
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
        if !self.username.is_empty() {
            len += 1;
        }
        if !self.resource_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.IsOverageRequest", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if !self.resource_name.is_empty() {
            struct_ser.serialize_field("resource_name", &self.resource_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IsOverageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "username",
            "resource_name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Username,
            ResourceName,
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
                            "username" => Ok(GeneratedField::Username),
                            "resource_name" => Ok(GeneratedField::ResourceName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IsOverageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.IsOverageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IsOverageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut username__ = None;
                let mut resource_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceName => {
                            if resource_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_name"));
                            }
                            resource_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(IsOverageRequest {
                    header: header__,
                    username: username__.unwrap_or_default(),
                    resource_name: resource_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.IsOverageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NoParamsRequest {
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
        let mut struct_ser = serializer.serialize_struct("qms.NoParamsRequest", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NoParamsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NoParamsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.NoParamsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NoParamsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                    }
                }
                Ok(NoParamsRequest {
                    header: header__,
                })
            }
        }
        deserializer.deserialize_struct("qms.NoParamsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Overage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_name.is_empty() {
            len += 1;
        }
        if self.quota != 0. {
            len += 1;
        }
        if self.usage != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.Overage", len)?;
        if !self.resource_name.is_empty() {
            struct_ser.serialize_field("resource_name", &self.resource_name)?;
        }
        if self.quota != 0. {
            struct_ser.serialize_field("quota", &self.quota)?;
        }
        if self.usage != 0. {
            struct_ser.serialize_field("usage", &self.usage)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Overage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_name",
            "quota",
            "usage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceName,
            Quota,
            Usage,
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
                            "resource_name" => Ok(GeneratedField::ResourceName),
                            "quota" => Ok(GeneratedField::Quota),
                            "usage" => Ok(GeneratedField::Usage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Overage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.Overage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Overage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_name__ = None;
                let mut quota__ = None;
                let mut usage__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceName => {
                            if resource_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_name"));
                            }
                            resource_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quota => {
                            if quota__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota"));
                            }
                            quota__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Usage => {
                            if usage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usage"));
                            }
                            usage__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Overage {
                    resource_name: resource_name__.unwrap_or_default(),
                    quota: quota__.unwrap_or_default(),
                    usage: usage__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.Overage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OverageList {
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
        if !self.overages.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.OverageList", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.overages.is_empty() {
            struct_ser.serialize_field("overages", &self.overages)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OverageList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "overages",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            Overages,
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
                            "overages" => Ok(GeneratedField::Overages),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OverageList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.OverageList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OverageList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut overages__ = None;
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
                        GeneratedField::Overages => {
                            if overages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overages"));
                            }
                            overages__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OverageList {
                    header: header__,
                    error: error__,
                    overages: overages__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.OverageList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OverageResponse {
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
        if self.overage.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.OverageResponse", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if let Some(v) = self.overage.as_ref() {
            struct_ser.serialize_field("overage", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OverageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "overage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            Overage,
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
                            "overage" => Ok(GeneratedField::Overage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OverageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.OverageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OverageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut overage__ = None;
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
                        GeneratedField::Overage => {
                            if overage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overage"));
                            }
                            overage__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OverageResponse {
                    header: header__,
                    error: error__,
                    overage: overage__,
                })
            }
        }
        deserializer.deserialize_struct("qms.OverageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Plan {
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
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.plan_quota_defaults.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.Plan", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.plan_quota_defaults.is_empty() {
            struct_ser.serialize_field("plan_quota_defaults", &self.plan_quota_defaults)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Plan {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uuid",
            "name",
            "description",
            "plan_quota_defaults",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            Name,
            Description,
            PlanQuotaDefaults,
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
                            "description" => Ok(GeneratedField::Description),
                            "plan_quota_defaults" => Ok(GeneratedField::PlanQuotaDefaults),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Plan;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.Plan")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Plan, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut plan_quota_defaults__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PlanQuotaDefaults => {
                            if plan_quota_defaults__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plan_quota_defaults"));
                            }
                            plan_quota_defaults__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Plan {
                    uuid: uuid__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    plan_quota_defaults: plan_quota_defaults__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.Plan", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PlanList {
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
        if !self.plans.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.PlanList", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.plans.is_empty() {
            struct_ser.serialize_field("plans", &self.plans)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PlanList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "plans",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            Plans,
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
                            "plans" => Ok(GeneratedField::Plans),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PlanList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.PlanList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PlanList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut plans__ = None;
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
                        GeneratedField::Plans => {
                            if plans__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plans"));
                            }
                            plans__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PlanList {
                    header: header__,
                    error: error__,
                    plans: plans__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.PlanList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PlanRequest {
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
        if !self.plan_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.PlanRequest", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.plan_id.is_empty() {
            struct_ser.serialize_field("plan_id", &self.plan_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PlanRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "plan_id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            PlanId,
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
                            "plan_id" => Ok(GeneratedField::PlanId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PlanRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.PlanRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PlanRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut plan_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::PlanId => {
                            if plan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plan_id"));
                            }
                            plan_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PlanRequest {
                    header: header__,
                    plan_id: plan_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.PlanRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PlanResponse {
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
        if self.plan.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.PlanResponse", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if let Some(v) = self.plan.as_ref() {
            struct_ser.serialize_field("plan", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PlanResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "plan",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            Plan,
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
                            "plan" => Ok(GeneratedField::Plan),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PlanResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.PlanResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PlanResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut plan__ = None;
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
                        GeneratedField::Plan => {
                            if plan__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plan"));
                            }
                            plan__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PlanResponse {
                    header: header__,
                    error: error__,
                    plan: plan__,
                })
            }
        }
        deserializer.deserialize_struct("qms.PlanResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QmsUser {
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
        if !self.username.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.QMSUser", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QmsUser {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uuid",
            "username",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            Username,
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
                            "username" => Ok(GeneratedField::Username),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QmsUser;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.QMSUser")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QmsUser, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut username__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QmsUser {
                    uuid: uuid__.unwrap_or_default(),
                    username: username__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.QMSUser", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QmsUserList {
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
        if !self.users.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.QMSUserList", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.users.is_empty() {
            struct_ser.serialize_field("users", &self.users)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QmsUserList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "users",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            Users,
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
                            "users" => Ok(GeneratedField::Users),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QmsUserList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.QMSUserList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QmsUserList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut users__ = None;
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
                        GeneratedField::Users => {
                            if users__.is_some() {
                                return Err(serde::de::Error::duplicate_field("users"));
                            }
                            users__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QmsUserList {
                    header: header__,
                    error: error__,
                    users: users__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.QMSUserList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QmsUserResponse {
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
        if self.user.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.QMSUserResponse", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QmsUserResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "user",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
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
                            "header" => Ok(GeneratedField::Header),
                            "error" => Ok(GeneratedField::Error),
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
            type Value = QmsUserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.QMSUserResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QmsUserResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut user__ = None;
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
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QmsUserResponse {
                    header: header__,
                    error: error__,
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("qms.QMSUserResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Quota {
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
        if self.quota != 0. {
            len += 1;
        }
        if self.resource_type.is_some() {
            len += 1;
        }
        if !self.created_by.is_empty() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if !self.last_modified_by.is_empty() {
            len += 1;
        }
        if self.last_modified_at.is_some() {
            len += 1;
        }
        if !self.subscription_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.Quota", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if self.quota != 0. {
            struct_ser.serialize_field("quota", &self.quota)?;
        }
        if let Some(v) = self.resource_type.as_ref() {
            struct_ser.serialize_field("resource_type", v)?;
        }
        if !self.created_by.is_empty() {
            struct_ser.serialize_field("created_by", &self.created_by)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("created_at", v)?;
        }
        if !self.last_modified_by.is_empty() {
            struct_ser.serialize_field("last_modified_by", &self.last_modified_by)?;
        }
        if let Some(v) = self.last_modified_at.as_ref() {
            struct_ser.serialize_field("last_modified_at", v)?;
        }
        if !self.subscription_id.is_empty() {
            struct_ser.serialize_field("subscription_id", &self.subscription_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Quota {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uuid",
            "quota",
            "resource_type",
            "CreatedBy",
            "created_by",
            "CreatedAt",
            "created_at",
            "LastModifiedBy",
            "last_modified_by",
            "LastModifiedAt",
            "last_modified_at",
            "subscription_id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            Quota,
            ResourceType,
            CreatedBy,
            CreatedAt,
            LastModifiedBy,
            LastModifiedAt,
            SubscriptionId,
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
                            "quota" => Ok(GeneratedField::Quota),
                            "resource_type" => Ok(GeneratedField::ResourceType),
                            "created_by" | "CreatedBy" => Ok(GeneratedField::CreatedBy),
                            "created_at" | "CreatedAt" => Ok(GeneratedField::CreatedAt),
                            "last_modified_by" | "LastModifiedBy" => Ok(GeneratedField::LastModifiedBy),
                            "last_modified_at" | "LastModifiedAt" => Ok(GeneratedField::LastModifiedAt),
                            "subscription_id" => Ok(GeneratedField::SubscriptionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Quota;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.Quota")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Quota, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut quota__ = None;
                let mut resource_type__ = None;
                let mut created_by__ = None;
                let mut created_at__ = None;
                let mut last_modified_by__ = None;
                let mut last_modified_at__ = None;
                let mut subscription_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quota => {
                            if quota__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota"));
                            }
                            quota__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_type"));
                            }
                            resource_type__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedBy => {
                            if created_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("created_by"));
                            }
                            created_by__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("created_at"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::LastModifiedBy => {
                            if last_modified_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("last_modified_by"));
                            }
                            last_modified_by__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastModifiedAt => {
                            if last_modified_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("last_modified_at"));
                            }
                            last_modified_at__ = map_.next_value()?;
                        }
                        GeneratedField::SubscriptionId => {
                            if subscription_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscription_id"));
                            }
                            subscription_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Quota {
                    uuid: uuid__.unwrap_or_default(),
                    quota: quota__.unwrap_or_default(),
                    resource_type: resource_type__,
                    created_by: created_by__.unwrap_or_default(),
                    created_at: created_at__,
                    last_modified_by: last_modified_by__.unwrap_or_default(),
                    last_modified_at: last_modified_at__,
                    subscription_id: subscription_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.Quota", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaDefault {
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
        if self.quota_value != 0. {
            len += 1;
        }
        if self.resource_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.QuotaDefault", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if self.quota_value != 0. {
            struct_ser.serialize_field("quota_value", &self.quota_value)?;
        }
        if let Some(v) = self.resource_type.as_ref() {
            struct_ser.serialize_field("resource_type", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuotaDefault {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uuid",
            "quota_value",
            "resource_type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            QuotaValue,
            ResourceType,
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
                            "quota_value" => Ok(GeneratedField::QuotaValue),
                            "resource_type" => Ok(GeneratedField::ResourceType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaDefault;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.QuotaDefault")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuotaDefault, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut quota_value__ = None;
                let mut resource_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuotaValue => {
                            if quota_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota_value"));
                            }
                            quota_value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_type"));
                            }
                            resource_type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QuotaDefault {
                    uuid: uuid__.unwrap_or_default(),
                    quota_value: quota_value__.unwrap_or_default(),
                    resource_type: resource_type__,
                })
            }
        }
        deserializer.deserialize_struct("qms.QuotaDefault", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaDefaultList {
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
        if !self.quota_defaults.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.QuotaDefaultList", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.quota_defaults.is_empty() {
            struct_ser.serialize_field("quota_defaults", &self.quota_defaults)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuotaDefaultList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "quota_defaults",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            QuotaDefaults,
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
                            "quota_defaults" => Ok(GeneratedField::QuotaDefaults),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaDefaultList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.QuotaDefaultList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuotaDefaultList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut quota_defaults__ = None;
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
                        GeneratedField::QuotaDefaults => {
                            if quota_defaults__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota_defaults"));
                            }
                            quota_defaults__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QuotaDefaultList {
                    header: header__,
                    error: error__,
                    quota_defaults: quota_defaults__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.QuotaDefaultList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaDefaultResponse {
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
        if self.quota_default.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.QuotaDefaultResponse", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if let Some(v) = self.quota_default.as_ref() {
            struct_ser.serialize_field("quota_default", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuotaDefaultResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "quota_default",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            QuotaDefault,
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
                            "quota_default" => Ok(GeneratedField::QuotaDefault),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaDefaultResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.QuotaDefaultResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuotaDefaultResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut quota_default__ = None;
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
                        GeneratedField::QuotaDefault => {
                            if quota_default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota_default"));
                            }
                            quota_default__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QuotaDefaultResponse {
                    header: header__,
                    error: error__,
                    quota_default: quota_default__,
                })
            }
        }
        deserializer.deserialize_struct("qms.QuotaDefaultResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaList {
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
        if !self.quotas.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.QuotaList", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.quotas.is_empty() {
            struct_ser.serialize_field("quotas", &self.quotas)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuotaList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "quotas",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            Quotas,
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
                            "quotas" => Ok(GeneratedField::Quotas),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.QuotaList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuotaList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut quotas__ = None;
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
                        GeneratedField::Quotas => {
                            if quotas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotas"));
                            }
                            quotas__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QuotaList {
                    header: header__,
                    error: error__,
                    quotas: quotas__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.QuotaList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaResponse {
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
        if self.quota.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.QuotaResponse", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if let Some(v) = self.quota.as_ref() {
            struct_ser.serialize_field("quota", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuotaResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "quota",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            Quota,
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
                            "quota" => Ok(GeneratedField::Quota),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.QuotaResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuotaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut quota__ = None;
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
                        GeneratedField::Quota => {
                            if quota__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota"));
                            }
                            quota__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QuotaResponse {
                    header: header__,
                    error: error__,
                    quota: quota__,
                })
            }
        }
        deserializer.deserialize_struct("qms.QuotaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestByUserId {
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
        if !self.user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.RequestByUserID", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("user_id", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestByUserId {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "user_id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            UserId,
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
                            "user_id" => Ok(GeneratedField::UserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestByUserId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.RequestByUserID")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RequestByUserId, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_id"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RequestByUserId {
                    header: header__,
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.RequestByUserID", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestByUsername {
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
        if !self.username.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.RequestByUsername", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestByUsername {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "username",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Username,
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
                            "username" => Ok(GeneratedField::Username),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestByUsername;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.RequestByUsername")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RequestByUsername, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut username__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RequestByUsername {
                    header: header__,
                    username: username__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.RequestByUsername", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceType {
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
        if !self.unit.is_empty() {
            len += 1;
        }
        if !self.consumable.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.ResourceType", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
        }
        if !self.consumable.is_empty() {
            struct_ser.serialize_field("consumable", &self.consumable)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uuid",
            "name",
            "unit",
            "consumable",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            Name,
            Unit,
            Consumable,
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
                            "unit" => Ok(GeneratedField::Unit),
                            "consumable" => Ok(GeneratedField::Consumable),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.ResourceType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut name__ = None;
                let mut unit__ = None;
                let mut consumable__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Consumable => {
                            if consumable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consumable"));
                            }
                            consumable__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResourceType {
                    uuid: uuid__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                    consumable: consumable__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.ResourceType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceTypeList {
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
        if !self.resource_types.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.ResourceTypeList", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.resource_types.is_empty() {
            struct_ser.serialize_field("resource_types", &self.resource_types)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceTypeList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "resource_types",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            ResourceTypes,
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
                            "resource_types" => Ok(GeneratedField::ResourceTypes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceTypeList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.ResourceTypeList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceTypeList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut resource_types__ = None;
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
                        GeneratedField::ResourceTypes => {
                            if resource_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_types"));
                            }
                            resource_types__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResourceTypeList {
                    header: header__,
                    error: error__,
                    resource_types: resource_types__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.ResourceTypeList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceTypeResponse {
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
        if self.resource_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.ResourceTypeResponse", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if let Some(v) = self.resource_type.as_ref() {
            struct_ser.serialize_field("resource_type", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceTypeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "resource_type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            ResourceType,
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
                            "resource_type" => Ok(GeneratedField::ResourceType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceTypeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.ResourceTypeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceTypeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut resource_type__ = None;
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
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_type"));
                            }
                            resource_type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ResourceTypeResponse {
                    header: header__,
                    error: error__,
                    resource_type: resource_type__,
                })
            }
        }
        deserializer.deserialize_struct("qms.ResourceTypeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Subscription {
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
        if self.effective_start_date.is_some() {
            len += 1;
        }
        if self.effective_end_date.is_some() {
            len += 1;
        }
        if self.user.is_some() {
            len += 1;
        }
        if self.plan.is_some() {
            len += 1;
        }
        if !self.quotas.is_empty() {
            len += 1;
        }
        if !self.usages.is_empty() {
            len += 1;
        }
        if self.paid {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.Subscription", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if let Some(v) = self.effective_start_date.as_ref() {
            struct_ser.serialize_field("effective_start_date", v)?;
        }
        if let Some(v) = self.effective_end_date.as_ref() {
            struct_ser.serialize_field("effective_end_date", v)?;
        }
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        if let Some(v) = self.plan.as_ref() {
            struct_ser.serialize_field("plan", v)?;
        }
        if !self.quotas.is_empty() {
            struct_ser.serialize_field("quotas", &self.quotas)?;
        }
        if !self.usages.is_empty() {
            struct_ser.serialize_field("usages", &self.usages)?;
        }
        if self.paid {
            struct_ser.serialize_field("paid", &self.paid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Subscription {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uuid",
            "effective_start_date",
            "effective_end_date",
            "user",
            "plan",
            "quotas",
            "usages",
            "paid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            EffectiveStartDate,
            EffectiveEndDate,
            User,
            Plan,
            Quotas,
            Usages,
            Paid,
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
                            "effective_start_date" => Ok(GeneratedField::EffectiveStartDate),
                            "effective_end_date" => Ok(GeneratedField::EffectiveEndDate),
                            "user" => Ok(GeneratedField::User),
                            "plan" => Ok(GeneratedField::Plan),
                            "quotas" => Ok(GeneratedField::Quotas),
                            "usages" => Ok(GeneratedField::Usages),
                            "paid" => Ok(GeneratedField::Paid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Subscription;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.Subscription")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Subscription, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut effective_start_date__ = None;
                let mut effective_end_date__ = None;
                let mut user__ = None;
                let mut plan__ = None;
                let mut quotas__ = None;
                let mut usages__ = None;
                let mut paid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EffectiveStartDate => {
                            if effective_start_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("effective_start_date"));
                            }
                            effective_start_date__ = map_.next_value()?;
                        }
                        GeneratedField::EffectiveEndDate => {
                            if effective_end_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("effective_end_date"));
                            }
                            effective_end_date__ = map_.next_value()?;
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                        GeneratedField::Plan => {
                            if plan__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plan"));
                            }
                            plan__ = map_.next_value()?;
                        }
                        GeneratedField::Quotas => {
                            if quotas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotas"));
                            }
                            quotas__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Usages => {
                            if usages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usages"));
                            }
                            usages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Paid => {
                            if paid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paid"));
                            }
                            paid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Subscription {
                    uuid: uuid__.unwrap_or_default(),
                    effective_start_date: effective_start_date__,
                    effective_end_date: effective_end_date__,
                    user: user__,
                    plan: plan__,
                    quotas: quotas__.unwrap_or_default(),
                    usages: usages__.unwrap_or_default(),
                    paid: paid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.Subscription", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubscriptionAddon {
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
        if self.addon.is_some() {
            len += 1;
        }
        if self.subscription.is_some() {
            len += 1;
        }
        if self.amount != 0. {
            len += 1;
        }
        if self.paid {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.SubscriptionAddon", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if let Some(v) = self.addon.as_ref() {
            struct_ser.serialize_field("addon", v)?;
        }
        if let Some(v) = self.subscription.as_ref() {
            struct_ser.serialize_field("subscription", v)?;
        }
        if self.amount != 0. {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if self.paid {
            struct_ser.serialize_field("paid", &self.paid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionAddon {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uuid",
            "addon",
            "subscription",
            "amount",
            "paid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            Addon,
            Subscription,
            Amount,
            Paid,
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
                            "addon" => Ok(GeneratedField::Addon),
                            "subscription" => Ok(GeneratedField::Subscription),
                            "amount" => Ok(GeneratedField::Amount),
                            "paid" => Ok(GeneratedField::Paid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubscriptionAddon;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.SubscriptionAddon")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubscriptionAddon, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut addon__ = None;
                let mut subscription__ = None;
                let mut amount__ = None;
                let mut paid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Addon => {
                            if addon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addon"));
                            }
                            addon__ = map_.next_value()?;
                        }
                        GeneratedField::Subscription => {
                            if subscription__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscription"));
                            }
                            subscription__ = map_.next_value()?;
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Paid => {
                            if paid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paid"));
                            }
                            paid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SubscriptionAddon {
                    uuid: uuid__.unwrap_or_default(),
                    addon: addon__,
                    subscription: subscription__,
                    amount: amount__.unwrap_or_default(),
                    paid: paid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.SubscriptionAddon", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubscriptionAddonListResponse {
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
        if !self.subscription_addons.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.SubscriptionAddonListResponse", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.subscription_addons.is_empty() {
            struct_ser.serialize_field("subscription_addons", &self.subscription_addons)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionAddonListResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "subscription_addons",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            SubscriptionAddons,
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
                            "subscription_addons" => Ok(GeneratedField::SubscriptionAddons),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubscriptionAddonListResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.SubscriptionAddonListResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubscriptionAddonListResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut subscription_addons__ = None;
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
                        GeneratedField::SubscriptionAddons => {
                            if subscription_addons__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscription_addons"));
                            }
                            subscription_addons__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SubscriptionAddonListResponse {
                    header: header__,
                    error: error__,
                    subscription_addons: subscription_addons__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.SubscriptionAddonListResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubscriptionAddonResponse {
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
        if self.subscription_addon.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.SubscriptionAddonResponse", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if let Some(v) = self.subscription_addon.as_ref() {
            struct_ser.serialize_field("subscription_addon", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionAddonResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "subscription_addon",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            SubscriptionAddon,
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
                            "subscription_addon" => Ok(GeneratedField::SubscriptionAddon),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubscriptionAddonResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.SubscriptionAddonResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubscriptionAddonResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut subscription_addon__ = None;
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
                        GeneratedField::SubscriptionAddon => {
                            if subscription_addon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscription_addon"));
                            }
                            subscription_addon__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SubscriptionAddonResponse {
                    header: header__,
                    error: error__,
                    subscription_addon: subscription_addon__,
                })
            }
        }
        deserializer.deserialize_struct("qms.SubscriptionAddonResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubscriptionList {
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
        if !self.subscriptions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.SubscriptionList", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.subscriptions.is_empty() {
            struct_ser.serialize_field("subscriptions", &self.subscriptions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "subscriptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            Subscriptions,
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
                            "subscriptions" => Ok(GeneratedField::Subscriptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubscriptionList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.SubscriptionList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubscriptionList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut subscriptions__ = None;
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
                        GeneratedField::Subscriptions => {
                            if subscriptions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscriptions"));
                            }
                            subscriptions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SubscriptionList {
                    header: header__,
                    error: error__,
                    subscriptions: subscriptions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.SubscriptionList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubscriptionResponse {
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
        if self.subscription.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.SubscriptionResponse", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if let Some(v) = self.subscription.as_ref() {
            struct_ser.serialize_field("subscription", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "subscription",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            Subscription,
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
                            "subscription" => Ok(GeneratedField::Subscription),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubscriptionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.SubscriptionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubscriptionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut subscription__ = None;
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
                        GeneratedField::Subscription => {
                            if subscription__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscription"));
                            }
                            subscription__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SubscriptionResponse {
                    header: header__,
                    error: error__,
                    subscription: subscription__,
                })
            }
        }
        deserializer.deserialize_struct("qms.SubscriptionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Update {
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
        if !self.value_type.is_empty() {
            len += 1;
        }
        if self.value != 0. {
            len += 1;
        }
        if self.effective_date.is_some() {
            len += 1;
        }
        if self.operation.is_some() {
            len += 1;
        }
        if self.resource_type.is_some() {
            len += 1;
        }
        if self.user.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.Update", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.value_type.is_empty() {
            struct_ser.serialize_field("value_type", &self.value_type)?;
        }
        if self.value != 0. {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if let Some(v) = self.effective_date.as_ref() {
            struct_ser.serialize_field("effective_date", v)?;
        }
        if let Some(v) = self.operation.as_ref() {
            struct_ser.serialize_field("operation", v)?;
        }
        if let Some(v) = self.resource_type.as_ref() {
            struct_ser.serialize_field("resource_type", v)?;
        }
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Update {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uuid",
            "value_type",
            "value",
            "effective_date",
            "operation",
            "resource_type",
            "user",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            ValueType,
            Value,
            EffectiveDate,
            Operation,
            ResourceType,
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
                            "uuid" => Ok(GeneratedField::Uuid),
                            "value_type" => Ok(GeneratedField::ValueType),
                            "value" => Ok(GeneratedField::Value),
                            "effective_date" => Ok(GeneratedField::EffectiveDate),
                            "operation" => Ok(GeneratedField::Operation),
                            "resource_type" => Ok(GeneratedField::ResourceType),
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
            type Value = Update;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.Update")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Update, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut value_type__ = None;
                let mut value__ = None;
                let mut effective_date__ = None;
                let mut operation__ = None;
                let mut resource_type__ = None;
                let mut user__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValueType => {
                            if value_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value_type"));
                            }
                            value_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EffectiveDate => {
                            if effective_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("effective_date"));
                            }
                            effective_date__ = map_.next_value()?;
                        }
                        GeneratedField::Operation => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation__ = map_.next_value()?;
                        }
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_type"));
                            }
                            resource_type__ = map_.next_value()?;
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Update {
                    uuid: uuid__.unwrap_or_default(),
                    value_type: value_type__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    effective_date: effective_date__,
                    operation: operation__,
                    resource_type: resource_type__,
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("qms.Update", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAddonRequest {
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
        if self.addon.is_some() {
            len += 1;
        }
        if self.update_name {
            len += 1;
        }
        if self.update_description {
            len += 1;
        }
        if self.update_resource_type {
            len += 1;
        }
        if self.update_default_amount {
            len += 1;
        }
        if self.update_default_paid {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.UpdateAddonRequest", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.addon.as_ref() {
            struct_ser.serialize_field("addon", v)?;
        }
        if self.update_name {
            struct_ser.serialize_field("updateName", &self.update_name)?;
        }
        if self.update_description {
            struct_ser.serialize_field("updateDescription", &self.update_description)?;
        }
        if self.update_resource_type {
            struct_ser.serialize_field("updateResourceType", &self.update_resource_type)?;
        }
        if self.update_default_amount {
            struct_ser.serialize_field("updateDefaultAmount", &self.update_default_amount)?;
        }
        if self.update_default_paid {
            struct_ser.serialize_field("updateDefaultPaid", &self.update_default_paid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAddonRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "addon",
            "update_name",
            "updateName",
            "update_description",
            "updateDescription",
            "update_resource_type",
            "updateResourceType",
            "update_default_amount",
            "updateDefaultAmount",
            "update_default_paid",
            "updateDefaultPaid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Addon,
            UpdateName,
            UpdateDescription,
            UpdateResourceType,
            UpdateDefaultAmount,
            UpdateDefaultPaid,
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
                            "addon" => Ok(GeneratedField::Addon),
                            "updateName" | "update_name" => Ok(GeneratedField::UpdateName),
                            "updateDescription" | "update_description" => Ok(GeneratedField::UpdateDescription),
                            "updateResourceType" | "update_resource_type" => Ok(GeneratedField::UpdateResourceType),
                            "updateDefaultAmount" | "update_default_amount" => Ok(GeneratedField::UpdateDefaultAmount),
                            "updateDefaultPaid" | "update_default_paid" => Ok(GeneratedField::UpdateDefaultPaid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAddonRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.UpdateAddonRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAddonRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut addon__ = None;
                let mut update_name__ = None;
                let mut update_description__ = None;
                let mut update_resource_type__ = None;
                let mut update_default_amount__ = None;
                let mut update_default_paid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Addon => {
                            if addon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addon"));
                            }
                            addon__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateName => {
                            if update_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateName"));
                            }
                            update_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdateDescription => {
                            if update_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateDescription"));
                            }
                            update_description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdateResourceType => {
                            if update_resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateResourceType"));
                            }
                            update_resource_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdateDefaultAmount => {
                            if update_default_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateDefaultAmount"));
                            }
                            update_default_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdateDefaultPaid => {
                            if update_default_paid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateDefaultPaid"));
                            }
                            update_default_paid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateAddonRequest {
                    header: header__,
                    addon: addon__,
                    update_name: update_name__.unwrap_or_default(),
                    update_description: update_description__.unwrap_or_default(),
                    update_resource_type: update_resource_type__.unwrap_or_default(),
                    update_default_amount: update_default_amount__.unwrap_or_default(),
                    update_default_paid: update_default_paid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.UpdateAddonRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateListRequest {
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
        if self.user.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.UpdateListRequest", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateListRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "user",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
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
                            "header" => Ok(GeneratedField::Header),
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
            type Value = UpdateListRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.UpdateListRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateListRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut user__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateListRequest {
                    header: header__,
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("qms.UpdateListRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateListResponse {
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
        if !self.updates.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.UpdateListResponse", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.updates.is_empty() {
            struct_ser.serialize_field("updates", &self.updates)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateListResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "updates",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            Updates,
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
                            "updates" => Ok(GeneratedField::Updates),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateListResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.UpdateListResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateListResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut updates__ = None;
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
                        GeneratedField::Updates => {
                            if updates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updates"));
                            }
                            updates__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateListResponse {
                    header: header__,
                    error: error__,
                    updates: updates__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.UpdateListResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateOperation {
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
        let mut struct_ser = serializer.serialize_struct("qms.UpdateOperation", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateOperation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uuid",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateOperation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.UpdateOperation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateOperation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateOperation {
                    uuid: uuid__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.UpdateOperation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateSubscriptionAddonRequest {
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
        if self.subscription_addon.is_some() {
            len += 1;
        }
        if self.update_addon_id {
            len += 1;
        }
        if self.update_subscription_id {
            len += 1;
        }
        if self.update_amount {
            len += 1;
        }
        if self.update_paid {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.UpdateSubscriptionAddonRequest", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.subscription_addon.as_ref() {
            struct_ser.serialize_field("subscription_addon", v)?;
        }
        if self.update_addon_id {
            struct_ser.serialize_field("update_addon_id", &self.update_addon_id)?;
        }
        if self.update_subscription_id {
            struct_ser.serialize_field("update_subscription_id", &self.update_subscription_id)?;
        }
        if self.update_amount {
            struct_ser.serialize_field("update_amount", &self.update_amount)?;
        }
        if self.update_paid {
            struct_ser.serialize_field("update_paid", &self.update_paid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateSubscriptionAddonRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "subscription_addon",
            "update_addon_id",
            "update_subscription_id",
            "update_amount",
            "update_paid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            SubscriptionAddon,
            UpdateAddonId,
            UpdateSubscriptionId,
            UpdateAmount,
            UpdatePaid,
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
                            "subscription_addon" => Ok(GeneratedField::SubscriptionAddon),
                            "update_addon_id" => Ok(GeneratedField::UpdateAddonId),
                            "update_subscription_id" => Ok(GeneratedField::UpdateSubscriptionId),
                            "update_amount" => Ok(GeneratedField::UpdateAmount),
                            "update_paid" => Ok(GeneratedField::UpdatePaid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateSubscriptionAddonRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.UpdateSubscriptionAddonRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateSubscriptionAddonRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut subscription_addon__ = None;
                let mut update_addon_id__ = None;
                let mut update_subscription_id__ = None;
                let mut update_amount__ = None;
                let mut update_paid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::SubscriptionAddon => {
                            if subscription_addon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscription_addon"));
                            }
                            subscription_addon__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateAddonId => {
                            if update_addon_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update_addon_id"));
                            }
                            update_addon_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdateSubscriptionId => {
                            if update_subscription_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update_subscription_id"));
                            }
                            update_subscription_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdateAmount => {
                            if update_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update_amount"));
                            }
                            update_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdatePaid => {
                            if update_paid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update_paid"));
                            }
                            update_paid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateSubscriptionAddonRequest {
                    header: header__,
                    subscription_addon: subscription_addon__,
                    update_addon_id: update_addon_id__.unwrap_or_default(),
                    update_subscription_id: update_subscription_id__.unwrap_or_default(),
                    update_amount: update_amount__.unwrap_or_default(),
                    update_paid: update_paid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.UpdateSubscriptionAddonRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Usage {
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
        if self.usage != 0. {
            len += 1;
        }
        if !self.subscription_id.is_empty() {
            len += 1;
        }
        if self.resource_type.is_some() {
            len += 1;
        }
        if !self.created_by.is_empty() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if !self.last_modified_by.is_empty() {
            len += 1;
        }
        if self.last_modified_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.Usage", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if self.usage != 0. {
            struct_ser.serialize_field("usage", &self.usage)?;
        }
        if !self.subscription_id.is_empty() {
            struct_ser.serialize_field("subscription_id", &self.subscription_id)?;
        }
        if let Some(v) = self.resource_type.as_ref() {
            struct_ser.serialize_field("resource_type", v)?;
        }
        if !self.created_by.is_empty() {
            struct_ser.serialize_field("created_by", &self.created_by)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("created_at", v)?;
        }
        if !self.last_modified_by.is_empty() {
            struct_ser.serialize_field("last_modified_by", &self.last_modified_by)?;
        }
        if let Some(v) = self.last_modified_at.as_ref() {
            struct_ser.serialize_field("last_modified_at", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Usage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uuid",
            "usage",
            "subscription_id",
            "resource_type",
            "CreatedBy",
            "created_by",
            "CreatedAt",
            "created_at",
            "LastModifiedBy",
            "last_modified_by",
            "LastModifiedAt",
            "last_modified_at",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            Usage,
            SubscriptionId,
            ResourceType,
            CreatedBy,
            CreatedAt,
            LastModifiedBy,
            LastModifiedAt,
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
                            "usage" => Ok(GeneratedField::Usage),
                            "subscription_id" => Ok(GeneratedField::SubscriptionId),
                            "resource_type" => Ok(GeneratedField::ResourceType),
                            "created_by" | "CreatedBy" => Ok(GeneratedField::CreatedBy),
                            "created_at" | "CreatedAt" => Ok(GeneratedField::CreatedAt),
                            "last_modified_by" | "LastModifiedBy" => Ok(GeneratedField::LastModifiedBy),
                            "last_modified_at" | "LastModifiedAt" => Ok(GeneratedField::LastModifiedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Usage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.Usage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Usage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut usage__ = None;
                let mut subscription_id__ = None;
                let mut resource_type__ = None;
                let mut created_by__ = None;
                let mut created_at__ = None;
                let mut last_modified_by__ = None;
                let mut last_modified_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Usage => {
                            if usage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usage"));
                            }
                            usage__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SubscriptionId => {
                            if subscription_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscription_id"));
                            }
                            subscription_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_type"));
                            }
                            resource_type__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedBy => {
                            if created_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("created_by"));
                            }
                            created_by__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("created_at"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::LastModifiedBy => {
                            if last_modified_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("last_modified_by"));
                            }
                            last_modified_by__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastModifiedAt => {
                            if last_modified_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("last_modified_at"));
                            }
                            last_modified_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Usage {
                    uuid: uuid__.unwrap_or_default(),
                    usage: usage__.unwrap_or_default(),
                    subscription_id: subscription_id__.unwrap_or_default(),
                    resource_type: resource_type__,
                    created_by: created_by__.unwrap_or_default(),
                    created_at: created_at__,
                    last_modified_by: last_modified_by__.unwrap_or_default(),
                    last_modified_at: last_modified_at__,
                })
            }
        }
        deserializer.deserialize_struct("qms.Usage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UsageList {
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
        if !self.usages.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.UsageList", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.usages.is_empty() {
            struct_ser.serialize_field("usages", &self.usages)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UsageList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "usages",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            Usages,
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
                            "usages" => Ok(GeneratedField::Usages),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UsageList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.UsageList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UsageList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut usages__ = None;
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
                        GeneratedField::Usages => {
                            if usages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usages"));
                            }
                            usages__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UsageList {
                    header: header__,
                    error: error__,
                    usages: usages__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.UsageList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UsageResponse {
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
        if self.usage.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.UsageResponse", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if let Some(v) = self.usage.as_ref() {
            struct_ser.serialize_field("usage", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UsageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "usage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            Usage,
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
                            "usage" => Ok(GeneratedField::Usage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UsageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.UsageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UsageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut usage__ = None;
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
                        GeneratedField::Usage => {
                            if usage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usage"));
                            }
                            usage__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UsageResponse {
                    header: header__,
                    error: error__,
                    usage: usage__,
                })
            }
        }
        deserializer.deserialize_struct("qms.UsageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserResourceOveragesRequest {
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
        if !self.username.is_empty() {
            len += 1;
        }
        if !self.resource_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("qms.UserResourceOveragesRequest", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if !self.resource_name.is_empty() {
            struct_ser.serialize_field("resource_name", &self.resource_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserResourceOveragesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "username",
            "resource_name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Username,
            ResourceName,
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
                            "username" => Ok(GeneratedField::Username),
                            "resource_name" => Ok(GeneratedField::ResourceName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserResourceOveragesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct qms.UserResourceOveragesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserResourceOveragesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut username__ = None;
                let mut resource_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceName => {
                            if resource_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_name"));
                            }
                            resource_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UserResourceOveragesRequest {
                    header: header__,
                    username: username__.unwrap_or_default(),
                    resource_name: resource_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("qms.UserResourceOveragesRequest", FIELDS, GeneratedVisitor)
    }
}
