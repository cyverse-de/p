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
        let mut struct_ser = serializer.serialize_struct("debuff.AddAddonRequest", len)?;
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
                formatter.write_str("struct debuff.AddAddonRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AddAddonRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut addon__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Addon => {
                            if addon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addon"));
                            }
                            addon__ = map.next_value()?;
                        }
                    }
                }
                Ok(AddAddonRequest {
                    header: header__,
                    addon: addon__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.AddAddonRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.AddPlanQuotaDefaultRequest", len)?;
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
                formatter.write_str("struct debuff.AddPlanQuotaDefaultRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AddPlanQuotaDefaultRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut plan_name__ = None;
                let mut quota_default__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::PlanName => {
                            if plan_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plan_name"));
                            }
                            plan_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::QuotaDefault => {
                            if quota_default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota_default"));
                            }
                            quota_default__ = map.next_value()?;
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
        deserializer.deserialize_struct("debuff.AddPlanQuotaDefaultRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.AddPlanRequest", len)?;
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
                formatter.write_str("struct debuff.AddPlanRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AddPlanRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut plan__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Plan => {
                            if plan__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plan"));
                            }
                            plan__ = map.next_value()?;
                        }
                    }
                }
                Ok(AddPlanRequest {
                    header: header__,
                    plan: plan__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.AddPlanRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.AddQuotaRequest", len)?;
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
                formatter.write_str("struct debuff.AddQuotaRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AddQuotaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut quota__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Quota => {
                            if quota__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota"));
                            }
                            quota__ = map.next_value()?;
                        }
                    }
                }
                Ok(AddQuotaRequest {
                    header: header__,
                    quota: quota__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.AddQuotaRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.AddUpdateRequest", len)?;
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
                formatter.write_str("struct debuff.AddUpdateRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AddUpdateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut update__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Update => {
                            if update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update"));
                            }
                            update__ = map.next_value()?;
                        }
                    }
                }
                Ok(AddUpdateRequest {
                    header: header__,
                    update: update__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.AddUpdateRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.AddUpdateResponse", len)?;
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
                formatter.write_str("struct debuff.AddUpdateResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AddUpdateResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut update__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::Update => {
                            if update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update"));
                            }
                            update__ = map.next_value()?;
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
        deserializer.deserialize_struct("debuff.AddUpdateResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.AddUsage", len)?;
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
                formatter.write_str("struct debuff.AddUsage")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AddUsage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut username__ = None;
                let mut resource_name__ = None;
                let mut update_type__ = None;
                let mut usage_value__ = None;
                let mut resource_unit__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResourceName => {
                            if resource_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_name"));
                            }
                            resource_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpdateType => {
                            if update_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update_type"));
                            }
                            update_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::UsageValue => {
                            if usage_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usage_value"));
                            }
                            usage_value__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResourceUnit => {
                            if resource_unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_unit"));
                            }
                            resource_unit__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.AddUsage", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.AddUserRequest", len)?;
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
                formatter.write_str("struct debuff.AddUserRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AddUserRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut username__ = None;
                let mut plan_name__ = None;
                let mut paid__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map.next_value()?);
                        }
                        GeneratedField::PlanName => {
                            if plan_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("planName"));
                            }
                            plan_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Paid => {
                            if paid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paid"));
                            }
                            paid__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.AddUserRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.AddUserResponse", len)?;
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
                formatter.write_str("struct debuff.AddUserResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AddUserResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut uuid__ = None;
                let mut username__ = None;
                let mut plan_name__ = None;
                let mut plan_uuid__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map.next_value()?);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map.next_value()?);
                        }
                        GeneratedField::PlanName => {
                            if plan_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("planName"));
                            }
                            plan_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::PlanUuid => {
                            if plan_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("planUuid"));
                            }
                            plan_uuid__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.AddUserResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.Addon", len)?;
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
                formatter.write_str("struct debuff.Addon")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Addon, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut resource_type__ = None;
                let mut default_amount__ = None;
                let mut default_paid__ = None;
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
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_type"));
                            }
                            resource_type__ = map.next_value()?;
                        }
                        GeneratedField::DefaultAmount => {
                            if default_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("default_amount"));
                            }
                            default_amount__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DefaultPaid => {
                            if default_paid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("default_paid"));
                            }
                            default_paid__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.Addon", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.AddonListResponse", len)?;
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
                formatter.write_str("struct debuff.AddonListResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AddonListResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut addons__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::Addons => {
                            if addons__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addons"));
                            }
                            addons__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.AddonListResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.AddonLookupRequest", len)?;
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
                formatter.write_str("struct debuff.AddonLookupRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AddonLookupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut addon__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Uuid => {
                            if addon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            addon__ = map.next_value::<::std::option::Option<_>>()?.map(addon_lookup_request::Addon::Uuid);
                        }
                        GeneratedField::Name => {
                            if addon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            addon__ = map.next_value::<::std::option::Option<_>>()?.map(addon_lookup_request::Addon::Name);
                        }
                    }
                }
                Ok(AddonLookupRequest {
                    header: header__,
                    addon: addon__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.AddonLookupRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.AddonResponse", len)?;
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
                formatter.write_str("struct debuff.AddonResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AddonResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut addon__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::Addon => {
                            if addon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addon"));
                            }
                            addon__ = map.next_value()?;
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
        deserializer.deserialize_struct("debuff.AddonResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.AllUserOveragesRequest", len)?;
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
                formatter.write_str("struct debuff.AllUserOveragesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AllUserOveragesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut username__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AllUserOveragesRequest {
                    header: header__,
                    username: username__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.AllUserOveragesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnalysisRecord {
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
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.can_share {
            len += 1;
        }
        if !self.username.is_empty() {
            len += 1;
        }
        if !self.app_id.is_empty() {
            len += 1;
        }
        if self.batch_status.is_some() {
            len += 1;
        }
        if !self.system_id.is_empty() {
            len += 1;
        }
        if self.app_disabled {
            len += 1;
        }
        if self.batch {
            len += 1;
        }
        if !self.enddate.is_empty() {
            len += 1;
        }
        if !self.startdate.is_empty() {
            len += 1;
        }
        if !self.app_description.is_empty() {
            len += 1;
        }
        if !self.interactive_urls.is_empty() {
            len += 1;
        }
        if !self.wiki_url.is_empty() {
            len += 1;
        }
        if self.notify {
            len += 1;
        }
        if !self.result_folder_id.is_empty() {
            len += 1;
        }
        if !self.app_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.AnalysisRecord", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.can_share {
            struct_ser.serialize_field("can_share", &self.can_share)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if !self.app_id.is_empty() {
            struct_ser.serialize_field("app_id", &self.app_id)?;
        }
        if let Some(v) = self.batch_status.as_ref() {
            struct_ser.serialize_field("batch_status", v)?;
        }
        if !self.system_id.is_empty() {
            struct_ser.serialize_field("system_id", &self.system_id)?;
        }
        if self.app_disabled {
            struct_ser.serialize_field("app_disabled", &self.app_disabled)?;
        }
        if self.batch {
            struct_ser.serialize_field("batch", &self.batch)?;
        }
        if !self.enddate.is_empty() {
            struct_ser.serialize_field("enddate", &self.enddate)?;
        }
        if !self.startdate.is_empty() {
            struct_ser.serialize_field("startdate", &self.startdate)?;
        }
        if !self.app_description.is_empty() {
            struct_ser.serialize_field("app_description", &self.app_description)?;
        }
        if !self.interactive_urls.is_empty() {
            struct_ser.serialize_field("interactive_urls", &self.interactive_urls)?;
        }
        if !self.wiki_url.is_empty() {
            struct_ser.serialize_field("wiki_url", &self.wiki_url)?;
        }
        if self.notify {
            struct_ser.serialize_field("notify", &self.notify)?;
        }
        if !self.result_folder_id.is_empty() {
            struct_ser.serialize_field("resultfolderid", &self.result_folder_id)?;
        }
        if !self.app_name.is_empty() {
            struct_ser.serialize_field("app_name", &self.app_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnalysisRecord {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "id",
            "description",
            "name",
            "can_share",
            "username",
            "app_id",
            "batch_status",
            "system_id",
            "app_disabled",
            "batch",
            "enddate",
            "startdate",
            "app_description",
            "interactive_urls",
            "wiki_url",
            "notify",
            "result_folder_id",
            "resultfolderid",
            "app_name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Id,
            Description,
            Name,
            CanShare,
            Username,
            AppId,
            BatchStatus,
            SystemId,
            AppDisabled,
            Batch,
            Enddate,
            Startdate,
            AppDescription,
            InteractiveUrls,
            WikiUrl,
            Notify,
            ResultFolderId,
            AppName,
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
                            "id" => Ok(GeneratedField::Id),
                            "description" => Ok(GeneratedField::Description),
                            "name" => Ok(GeneratedField::Name),
                            "can_share" => Ok(GeneratedField::CanShare),
                            "username" => Ok(GeneratedField::Username),
                            "app_id" => Ok(GeneratedField::AppId),
                            "batch_status" => Ok(GeneratedField::BatchStatus),
                            "system_id" => Ok(GeneratedField::SystemId),
                            "app_disabled" => Ok(GeneratedField::AppDisabled),
                            "batch" => Ok(GeneratedField::Batch),
                            "enddate" => Ok(GeneratedField::Enddate),
                            "startdate" => Ok(GeneratedField::Startdate),
                            "app_description" => Ok(GeneratedField::AppDescription),
                            "interactive_urls" => Ok(GeneratedField::InteractiveUrls),
                            "wiki_url" => Ok(GeneratedField::WikiUrl),
                            "notify" => Ok(GeneratedField::Notify),
                            "resultfolderid" | "result_folder_id" => Ok(GeneratedField::ResultFolderId),
                            "app_name" => Ok(GeneratedField::AppName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnalysisRecord;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.AnalysisRecord")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AnalysisRecord, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut id__ = None;
                let mut description__ = None;
                let mut name__ = None;
                let mut can_share__ = None;
                let mut username__ = None;
                let mut app_id__ = None;
                let mut batch_status__ = None;
                let mut system_id__ = None;
                let mut app_disabled__ = None;
                let mut batch__ = None;
                let mut enddate__ = None;
                let mut startdate__ = None;
                let mut app_description__ = None;
                let mut interactive_urls__ = None;
                let mut wiki_url__ = None;
                let mut notify__ = None;
                let mut result_folder_id__ = None;
                let mut app_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::CanShare => {
                            if can_share__.is_some() {
                                return Err(serde::de::Error::duplicate_field("can_share"));
                            }
                            can_share__ = Some(map.next_value()?);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map.next_value()?);
                        }
                        GeneratedField::AppId => {
                            if app_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app_id"));
                            }
                            app_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::BatchStatus => {
                            if batch_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batch_status"));
                            }
                            batch_status__ = map.next_value()?;
                        }
                        GeneratedField::SystemId => {
                            if system_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("system_id"));
                            }
                            system_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::AppDisabled => {
                            if app_disabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app_disabled"));
                            }
                            app_disabled__ = Some(map.next_value()?);
                        }
                        GeneratedField::Batch => {
                            if batch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batch"));
                            }
                            batch__ = Some(map.next_value()?);
                        }
                        GeneratedField::Enddate => {
                            if enddate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enddate"));
                            }
                            enddate__ = Some(map.next_value()?);
                        }
                        GeneratedField::Startdate => {
                            if startdate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startdate"));
                            }
                            startdate__ = Some(map.next_value()?);
                        }
                        GeneratedField::AppDescription => {
                            if app_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app_description"));
                            }
                            app_description__ = Some(map.next_value()?);
                        }
                        GeneratedField::InteractiveUrls => {
                            if interactive_urls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interactive_urls"));
                            }
                            interactive_urls__ = Some(map.next_value()?);
                        }
                        GeneratedField::WikiUrl => {
                            if wiki_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wiki_url"));
                            }
                            wiki_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::Notify => {
                            if notify__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notify"));
                            }
                            notify__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResultFolderId => {
                            if result_folder_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resultfolderid"));
                            }
                            result_folder_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::AppName => {
                            if app_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app_name"));
                            }
                            app_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AnalysisRecord {
                    header: header__,
                    id: id__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    can_share: can_share__.unwrap_or_default(),
                    username: username__.unwrap_or_default(),
                    app_id: app_id__.unwrap_or_default(),
                    batch_status: batch_status__,
                    system_id: system_id__.unwrap_or_default(),
                    app_disabled: app_disabled__.unwrap_or_default(),
                    batch: batch__.unwrap_or_default(),
                    enddate: enddate__.unwrap_or_default(),
                    startdate: startdate__.unwrap_or_default(),
                    app_description: app_description__.unwrap_or_default(),
                    interactive_urls: interactive_urls__.unwrap_or_default(),
                    wiki_url: wiki_url__.unwrap_or_default(),
                    notify: notify__.unwrap_or_default(),
                    result_folder_id: result_folder_id__.unwrap_or_default(),
                    app_name: app_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.AnalysisRecord", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for analysis_record::BatchStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.total != 0 {
            len += 1;
        }
        if self.completed != 0 {
            len += 1;
        }
        if self.running != 0 {
            len += 1;
        }
        if self.submitted != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.AnalysisRecord.BatchStatus", len)?;
        if self.total != 0 {
            struct_ser.serialize_field("total", ToString::to_string(&self.total).as_str())?;
        }
        if self.completed != 0 {
            struct_ser.serialize_field("completed", ToString::to_string(&self.completed).as_str())?;
        }
        if self.running != 0 {
            struct_ser.serialize_field("running", ToString::to_string(&self.running).as_str())?;
        }
        if self.submitted != 0 {
            struct_ser.serialize_field("submitted", ToString::to_string(&self.submitted).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for analysis_record::BatchStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "total",
            "completed",
            "running",
            "submitted",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Total,
            Completed,
            Running,
            Submitted,
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
                            "total" => Ok(GeneratedField::Total),
                            "completed" => Ok(GeneratedField::Completed),
                            "running" => Ok(GeneratedField::Running),
                            "submitted" => Ok(GeneratedField::Submitted),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = analysis_record::BatchStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.AnalysisRecord.BatchStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<analysis_record::BatchStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut total__ = None;
                let mut completed__ = None;
                let mut running__ = None;
                let mut submitted__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Completed => {
                            if completed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("completed"));
                            }
                            completed__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Running => {
                            if running__.is_some() {
                                return Err(serde::de::Error::duplicate_field("running"));
                            }
                            running__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Submitted => {
                            if submitted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("submitted"));
                            }
                            submitted__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(analysis_record::BatchStatus {
                    total: total__.unwrap_or_default(),
                    completed: completed__.unwrap_or_default(),
                    running: running__.unwrap_or_default(),
                    submitted: submitted__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.AnalysisRecord.BatchStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnalysisRecordList {
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
        if !self.analyses.is_empty() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.AnalysisRecordList", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.analyses.is_empty() {
            struct_ser.serialize_field("analyses", &self.analyses)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnalysisRecordList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "analyses",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Analyses,
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
                            "header" => Ok(GeneratedField::Header),
                            "analyses" => Ok(GeneratedField::Analyses),
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
            type Value = AnalysisRecordList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.AnalysisRecordList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AnalysisRecordList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut analyses__ = None;
                let mut error__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Analyses => {
                            if analyses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("analyses"));
                            }
                            analyses__ = Some(map.next_value()?);
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                    }
                }
                Ok(AnalysisRecordList {
                    header: header__,
                    analyses: analyses__.unwrap_or_default(),
                    error: error__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.AnalysisRecordList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnalysisRecordLookupRequest {
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
        if !self.requesting_user.is_empty() {
            len += 1;
        }
        if self.lookup_ids.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.AnalysisRecordLookupRequest", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.requesting_user.is_empty() {
            struct_ser.serialize_field("requestingUser", &self.requesting_user)?;
        }
        if let Some(v) = self.lookup_ids.as_ref() {
            match v {
                analysis_record_lookup_request::LookupIds::AnalysisId(v) => {
                    struct_ser.serialize_field("analysisId", v)?;
                }
                analysis_record_lookup_request::LookupIds::ExternalId(v) => {
                    struct_ser.serialize_field("externalId", v)?;
                }
                analysis_record_lookup_request::LookupIds::UserId(v) => {
                    struct_ser.serialize_field("userId", v)?;
                }
                analysis_record_lookup_request::LookupIds::Username(v) => {
                    struct_ser.serialize_field("username", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnalysisRecordLookupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "requesting_user",
            "requestingUser",
            "analysis_id",
            "analysisId",
            "external_id",
            "externalId",
            "user_id",
            "userId",
            "username",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            RequestingUser,
            AnalysisId,
            ExternalId,
            UserId,
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
                            "requestingUser" | "requesting_user" => Ok(GeneratedField::RequestingUser),
                            "analysisId" | "analysis_id" => Ok(GeneratedField::AnalysisId),
                            "externalId" | "external_id" => Ok(GeneratedField::ExternalId),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
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
            type Value = AnalysisRecordLookupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.AnalysisRecordLookupRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AnalysisRecordLookupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut requesting_user__ = None;
                let mut lookup_ids__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::RequestingUser => {
                            if requesting_user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestingUser"));
                            }
                            requesting_user__ = Some(map.next_value()?);
                        }
                        GeneratedField::AnalysisId => {
                            if lookup_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("analysisId"));
                            }
                            lookup_ids__ = map.next_value::<::std::option::Option<_>>()?.map(analysis_record_lookup_request::LookupIds::AnalysisId);
                        }
                        GeneratedField::ExternalId => {
                            if lookup_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalId"));
                            }
                            lookup_ids__ = map.next_value::<::std::option::Option<_>>()?.map(analysis_record_lookup_request::LookupIds::ExternalId);
                        }
                        GeneratedField::UserId => {
                            if lookup_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            lookup_ids__ = map.next_value::<::std::option::Option<_>>()?.map(analysis_record_lookup_request::LookupIds::UserId);
                        }
                        GeneratedField::Username => {
                            if lookup_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            lookup_ids__ = map.next_value::<::std::option::Option<_>>()?.map(analysis_record_lookup_request::LookupIds::Username);
                        }
                    }
                }
                Ok(AnalysisRecordLookupRequest {
                    header: header__,
                    requesting_user: requesting_user__.unwrap_or_default(),
                    lookup_ids: lookup_ids__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.AnalysisRecordLookupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnalysisRecordResponse {
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
        if !self.analyses.is_empty() {
            len += 1;
        }
        if !self.timestamp.is_empty() {
            len += 1;
        }
        if self.total != 0 {
            len += 1;
        }
        if !self.status_count.is_empty() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.AnalysisRecordResponse", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.analyses.is_empty() {
            struct_ser.serialize_field("analyses", &self.analyses)?;
        }
        if !self.timestamp.is_empty() {
            struct_ser.serialize_field("timestamp", &self.timestamp)?;
        }
        if self.total != 0 {
            struct_ser.serialize_field("total", ToString::to_string(&self.total).as_str())?;
        }
        if !self.status_count.is_empty() {
            struct_ser.serialize_field("status-count", &self.status_count)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnalysisRecordResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "analyses",
            "timestamp",
            "total",
            "status_count",
            "status-count",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Analyses,
            Timestamp,
            Total,
            StatusCount,
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
                            "header" => Ok(GeneratedField::Header),
                            "analyses" => Ok(GeneratedField::Analyses),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "total" => Ok(GeneratedField::Total),
                            "status-count" | "status_count" => Ok(GeneratedField::StatusCount),
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
            type Value = AnalysisRecordResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.AnalysisRecordResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AnalysisRecordResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut analyses__ = None;
                let mut timestamp__ = None;
                let mut total__ = None;
                let mut status_count__ = None;
                let mut error__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Analyses => {
                            if analyses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("analyses"));
                            }
                            analyses__ = Some(map.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = Some(map.next_value()?);
                        }
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StatusCount => {
                            if status_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status-count"));
                            }
                            status_count__ = Some(map.next_value()?);
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                    }
                }
                Ok(AnalysisRecordResponse {
                    header: header__,
                    analyses: analyses__.unwrap_or_default(),
                    timestamp: timestamp__.unwrap_or_default(),
                    total: total__.unwrap_or_default(),
                    status_count: status_count__.unwrap_or_default(),
                    error: error__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.AnalysisRecordResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for analysis_record_response::StatusCountRecord {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.count != 0 {
            len += 1;
        }
        if !self.status.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.AnalysisRecordResponse.StatusCountRecord", len)?;
        if self.count != 0 {
            struct_ser.serialize_field("count", ToString::to_string(&self.count).as_str())?;
        }
        if !self.status.is_empty() {
            struct_ser.serialize_field("status", &self.status)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for analysis_record_response::StatusCountRecord {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "count",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Count,
            Status,
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
                            "count" => Ok(GeneratedField::Count),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = analysis_record_response::StatusCountRecord;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.AnalysisRecordResponse.StatusCountRecord")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<analysis_record_response::StatusCountRecord, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut count__ = None;
                let mut status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Count => {
                            if count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(analysis_record_response::StatusCountRecord {
                    count: count__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.AnalysisRecordResponse.StatusCountRecord", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnalysisStatus {
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
        if self.job.is_some() {
            len += 1;
        }
        if self.version != 0 {
            len += 1;
        }
        if !self.state.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if !self.sent_on.is_empty() {
            len += 1;
        }
        if !self.sender.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.AnalysisStatus", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if let Some(v) = self.job.as_ref() {
            struct_ser.serialize_field("job", v)?;
        }
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.state.is_empty() {
            struct_ser.serialize_field("state", &self.state)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if !self.sent_on.is_empty() {
            struct_ser.serialize_field("sent_on", &self.sent_on)?;
        }
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnalysisStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error",
            "job",
            "version",
            "state",
            "message",
            "sent_on",
            "sender",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Error,
            Job,
            Version,
            State,
            Message,
            SentOn,
            Sender,
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
                            "job" => Ok(GeneratedField::Job),
                            "version" => Ok(GeneratedField::Version),
                            "state" => Ok(GeneratedField::State),
                            "message" => Ok(GeneratedField::Message),
                            "sent_on" => Ok(GeneratedField::SentOn),
                            "sender" => Ok(GeneratedField::Sender),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnalysisStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.AnalysisStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AnalysisStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut job__ = None;
                let mut version__ = None;
                let mut state__ = None;
                let mut message__ = None;
                let mut sent_on__ = None;
                let mut sender__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::Job => {
                            if job__.is_some() {
                                return Err(serde::de::Error::duplicate_field("job"));
                            }
                            job__ = map.next_value()?;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map.next_value()?);
                        }
                        GeneratedField::SentOn => {
                            if sent_on__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sent_on"));
                            }
                            sent_on__ = Some(map.next_value()?);
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AnalysisStatus {
                    header: header__,
                    error: error__,
                    job: job__,
                    version: version__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    sent_on: sent_on__.unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.AnalysisStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnalysisSubmission {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.app_description.is_empty() {
            len += 1;
        }
        if !self.app_id.is_empty() {
            len += 1;
        }
        if !self.app_name.is_empty() {
            len += 1;
        }
        if self.archive_logs {
            len += 1;
        }
        if !self.batch_id.is_empty() {
            len += 1;
        }
        if !self.condor_id.is_empty() {
            len += 1;
        }
        if !self.condor_log_path.is_empty() {
            len += 1;
        }
        if self.create_output_subdir {
            len += 1;
        }
        if self.date_submitted.is_some() {
            len += 1;
        }
        if self.date_started.is_some() {
            len += 1;
        }
        if self.date_completed.is_some() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.email.is_empty() {
            len += 1;
        }
        if self.extra.is_some() {
            len += 1;
        }
        if !self.execution_target.is_empty() {
            len += 1;
        }
        if self.exit_code != 0 {
            len += 1;
        }
        if self.failure_count != 0 {
            len += 1;
        }
        if self.failure_threshold != 0 {
            len += 1;
        }
        if !self.file_metadata.is_empty() {
            len += 1;
        }
        if !self.filter_files.is_empty() {
            len += 1;
        }
        if !self.group.is_empty() {
            len += 1;
        }
        if !self.input_path_list_file.is_empty() {
            len += 1;
        }
        if !self.input_tickets_file.is_empty() {
            len += 1;
        }
        if !self.invocation_id.is_empty() {
            len += 1;
        }
        if !self.irods_base.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.nfs_base.is_empty() {
            len += 1;
        }
        if self.notify {
            len += 1;
        }
        if !self.now_date.is_empty() {
            len += 1;
        }
        if !self.output_dir.is_empty() {
            len += 1;
        }
        if !self.output_dir_ticket.is_empty() {
            len += 1;
        }
        if !self.output_ticket_file.is_empty() {
            len += 1;
        }
        if !self.request_type.is_empty() {
            len += 1;
        }
        if self.run_on_nfs {
            len += 1;
        }
        if self.skip_parent_metadata {
            len += 1;
        }
        if !self.steps.is_empty() {
            len += 1;
        }
        if !self.submission_date.is_empty() {
            len += 1;
        }
        if !self.submitter.is_empty() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if !self.user_groups.is_empty() {
            len += 1;
        }
        if !self.user_home.is_empty() {
            len += 1;
        }
        if !self.wiki_url.is_empty() {
            len += 1;
        }
        if !self.config_file.is_empty() {
            len += 1;
        }
        if self.header.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.AnalysisSubmission", len)?;
        if !self.app_description.is_empty() {
            struct_ser.serialize_field("app_description", &self.app_description)?;
        }
        if !self.app_id.is_empty() {
            struct_ser.serialize_field("app_id", &self.app_id)?;
        }
        if !self.app_name.is_empty() {
            struct_ser.serialize_field("app_name", &self.app_name)?;
        }
        if self.archive_logs {
            struct_ser.serialize_field("archive_logs", &self.archive_logs)?;
        }
        if !self.batch_id.is_empty() {
            struct_ser.serialize_field("batch_id", &self.batch_id)?;
        }
        if !self.condor_id.is_empty() {
            struct_ser.serialize_field("condor_id", &self.condor_id)?;
        }
        if !self.condor_log_path.is_empty() {
            struct_ser.serialize_field("condor_log_path", &self.condor_log_path)?;
        }
        if self.create_output_subdir {
            struct_ser.serialize_field("create_output_subdir", &self.create_output_subdir)?;
        }
        if let Some(v) = self.date_submitted.as_ref() {
            struct_ser.serialize_field("date_submitted", v)?;
        }
        if let Some(v) = self.date_started.as_ref() {
            struct_ser.serialize_field("date_started", v)?;
        }
        if let Some(v) = self.date_completed.as_ref() {
            struct_ser.serialize_field("date_completed", v)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if let Some(v) = self.extra.as_ref() {
            struct_ser.serialize_field("extra", v)?;
        }
        if !self.execution_target.is_empty() {
            struct_ser.serialize_field("execution_target", &self.execution_target)?;
        }
        if self.exit_code != 0 {
            struct_ser.serialize_field("exit_code", &self.exit_code)?;
        }
        if self.failure_count != 0 {
            struct_ser.serialize_field("failure_count", ToString::to_string(&self.failure_count).as_str())?;
        }
        if self.failure_threshold != 0 {
            struct_ser.serialize_field("failure_threshold", ToString::to_string(&self.failure_threshold).as_str())?;
        }
        if !self.file_metadata.is_empty() {
            struct_ser.serialize_field("file-metadata", &self.file_metadata)?;
        }
        if !self.filter_files.is_empty() {
            struct_ser.serialize_field("filter_files", &self.filter_files)?;
        }
        if !self.group.is_empty() {
            struct_ser.serialize_field("group", &self.group)?;
        }
        if !self.input_path_list_file.is_empty() {
            struct_ser.serialize_field("input_path_list", &self.input_path_list_file)?;
        }
        if !self.input_tickets_file.is_empty() {
            struct_ser.serialize_field("input_ticket_list", &self.input_tickets_file)?;
        }
        if !self.invocation_id.is_empty() {
            struct_ser.serialize_field("invocation_id", &self.invocation_id)?;
        }
        if !self.irods_base.is_empty() {
            struct_ser.serialize_field("irods_base", &self.irods_base)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.nfs_base.is_empty() {
            struct_ser.serialize_field("nfs_base", &self.nfs_base)?;
        }
        if self.notify {
            struct_ser.serialize_field("notify", &self.notify)?;
        }
        if !self.now_date.is_empty() {
            struct_ser.serialize_field("now_date", &self.now_date)?;
        }
        if !self.output_dir.is_empty() {
            struct_ser.serialize_field("output_dir", &self.output_dir)?;
        }
        if !self.output_dir_ticket.is_empty() {
            struct_ser.serialize_field("output_dir_ticket", &self.output_dir_ticket)?;
        }
        if !self.output_ticket_file.is_empty() {
            struct_ser.serialize_field("output_ticket_file", &self.output_ticket_file)?;
        }
        if !self.request_type.is_empty() {
            struct_ser.serialize_field("request_type", &self.request_type)?;
        }
        if self.run_on_nfs {
            struct_ser.serialize_field("run-on-nfs", &self.run_on_nfs)?;
        }
        if self.skip_parent_metadata {
            struct_ser.serialize_field("skip-parent-meta", &self.skip_parent_metadata)?;
        }
        if !self.steps.is_empty() {
            struct_ser.serialize_field("steps", &self.steps)?;
        }
        if !self.submission_date.is_empty() {
            struct_ser.serialize_field("submission_date", &self.submission_date)?;
        }
        if !self.submitter.is_empty() {
            struct_ser.serialize_field("username", &self.submitter)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("user_id", &self.user_id)?;
        }
        if !self.user_groups.is_empty() {
            struct_ser.serialize_field("user_groups", &self.user_groups)?;
        }
        if !self.user_home.is_empty() {
            struct_ser.serialize_field("user_home", &self.user_home)?;
        }
        if !self.wiki_url.is_empty() {
            struct_ser.serialize_field("wiki_url", &self.wiki_url)?;
        }
        if !self.config_file.is_empty() {
            struct_ser.serialize_field("config_file", &self.config_file)?;
        }
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnalysisSubmission {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "app_description",
            "app_id",
            "app_name",
            "archive_logs",
            "batch_id",
            "condor_id",
            "condor_log_path",
            "create_output_subdir",
            "date_submitted",
            "date_started",
            "date_completed",
            "description",
            "email",
            "extra",
            "execution_target",
            "exit_code",
            "failure_count",
            "failure_threshold",
            "file_metadata",
            "file-metadata",
            "filter_files",
            "group",
            "input_path_list_file",
            "input_path_list",
            "input_tickets_file",
            "input_ticket_list",
            "invocation_id",
            "irods_base",
            "name",
            "nfs_base",
            "notify",
            "now_date",
            "output_dir",
            "output_dir_ticket",
            "output_ticket_file",
            "request_type",
            "run_on_nfs",
            "run-on-nfs",
            "skip_parent_metadata",
            "skip-parent-meta",
            "steps",
            "submission_date",
            "submitter",
            "username",
            "type",
            "user_id",
            "user_groups",
            "user_home",
            "wiki_url",
            "config_file",
            "header",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AppDescription,
            AppId,
            AppName,
            ArchiveLogs,
            BatchId,
            CondorId,
            CondorLogPath,
            CreateOutputSubdir,
            DateSubmitted,
            DateStarted,
            DateCompleted,
            Description,
            Email,
            Extra,
            ExecutionTarget,
            ExitCode,
            FailureCount,
            FailureThreshold,
            FileMetadata,
            FilterFiles,
            Group,
            InputPathListFile,
            InputTicketsFile,
            InvocationId,
            IrodsBase,
            Name,
            NfsBase,
            Notify,
            NowDate,
            OutputDir,
            OutputDirTicket,
            OutputTicketFile,
            RequestType,
            RunOnNfs,
            SkipParentMetadata,
            Steps,
            SubmissionDate,
            Submitter,
            Type,
            UserId,
            UserGroups,
            UserHome,
            WikiUrl,
            ConfigFile,
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
                            "app_description" => Ok(GeneratedField::AppDescription),
                            "app_id" => Ok(GeneratedField::AppId),
                            "app_name" => Ok(GeneratedField::AppName),
                            "archive_logs" => Ok(GeneratedField::ArchiveLogs),
                            "batch_id" => Ok(GeneratedField::BatchId),
                            "condor_id" => Ok(GeneratedField::CondorId),
                            "condor_log_path" => Ok(GeneratedField::CondorLogPath),
                            "create_output_subdir" => Ok(GeneratedField::CreateOutputSubdir),
                            "date_submitted" => Ok(GeneratedField::DateSubmitted),
                            "date_started" => Ok(GeneratedField::DateStarted),
                            "date_completed" => Ok(GeneratedField::DateCompleted),
                            "description" => Ok(GeneratedField::Description),
                            "email" => Ok(GeneratedField::Email),
                            "extra" => Ok(GeneratedField::Extra),
                            "execution_target" => Ok(GeneratedField::ExecutionTarget),
                            "exit_code" => Ok(GeneratedField::ExitCode),
                            "failure_count" => Ok(GeneratedField::FailureCount),
                            "failure_threshold" => Ok(GeneratedField::FailureThreshold),
                            "file-metadata" | "file_metadata" => Ok(GeneratedField::FileMetadata),
                            "filter_files" => Ok(GeneratedField::FilterFiles),
                            "group" => Ok(GeneratedField::Group),
                            "input_path_list" | "input_path_list_file" => Ok(GeneratedField::InputPathListFile),
                            "input_ticket_list" | "input_tickets_file" => Ok(GeneratedField::InputTicketsFile),
                            "invocation_id" => Ok(GeneratedField::InvocationId),
                            "irods_base" => Ok(GeneratedField::IrodsBase),
                            "name" => Ok(GeneratedField::Name),
                            "nfs_base" => Ok(GeneratedField::NfsBase),
                            "notify" => Ok(GeneratedField::Notify),
                            "now_date" => Ok(GeneratedField::NowDate),
                            "output_dir" => Ok(GeneratedField::OutputDir),
                            "output_dir_ticket" => Ok(GeneratedField::OutputDirTicket),
                            "output_ticket_file" => Ok(GeneratedField::OutputTicketFile),
                            "request_type" => Ok(GeneratedField::RequestType),
                            "run-on-nfs" | "run_on_nfs" => Ok(GeneratedField::RunOnNfs),
                            "skip-parent-meta" | "skip_parent_metadata" => Ok(GeneratedField::SkipParentMetadata),
                            "steps" => Ok(GeneratedField::Steps),
                            "submission_date" => Ok(GeneratedField::SubmissionDate),
                            "username" | "submitter" => Ok(GeneratedField::Submitter),
                            "type" => Ok(GeneratedField::Type),
                            "user_id" => Ok(GeneratedField::UserId),
                            "user_groups" => Ok(GeneratedField::UserGroups),
                            "user_home" => Ok(GeneratedField::UserHome),
                            "wiki_url" => Ok(GeneratedField::WikiUrl),
                            "config_file" => Ok(GeneratedField::ConfigFile),
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
            type Value = AnalysisSubmission;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.AnalysisSubmission")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AnalysisSubmission, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut app_description__ = None;
                let mut app_id__ = None;
                let mut app_name__ = None;
                let mut archive_logs__ = None;
                let mut batch_id__ = None;
                let mut condor_id__ = None;
                let mut condor_log_path__ = None;
                let mut create_output_subdir__ = None;
                let mut date_submitted__ = None;
                let mut date_started__ = None;
                let mut date_completed__ = None;
                let mut description__ = None;
                let mut email__ = None;
                let mut extra__ = None;
                let mut execution_target__ = None;
                let mut exit_code__ = None;
                let mut failure_count__ = None;
                let mut failure_threshold__ = None;
                let mut file_metadata__ = None;
                let mut filter_files__ = None;
                let mut group__ = None;
                let mut input_path_list_file__ = None;
                let mut input_tickets_file__ = None;
                let mut invocation_id__ = None;
                let mut irods_base__ = None;
                let mut name__ = None;
                let mut nfs_base__ = None;
                let mut notify__ = None;
                let mut now_date__ = None;
                let mut output_dir__ = None;
                let mut output_dir_ticket__ = None;
                let mut output_ticket_file__ = None;
                let mut request_type__ = None;
                let mut run_on_nfs__ = None;
                let mut skip_parent_metadata__ = None;
                let mut steps__ = None;
                let mut submission_date__ = None;
                let mut submitter__ = None;
                let mut r#type__ = None;
                let mut user_id__ = None;
                let mut user_groups__ = None;
                let mut user_home__ = None;
                let mut wiki_url__ = None;
                let mut config_file__ = None;
                let mut header__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AppDescription => {
                            if app_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app_description"));
                            }
                            app_description__ = Some(map.next_value()?);
                        }
                        GeneratedField::AppId => {
                            if app_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app_id"));
                            }
                            app_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::AppName => {
                            if app_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app_name"));
                            }
                            app_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::ArchiveLogs => {
                            if archive_logs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archive_logs"));
                            }
                            archive_logs__ = Some(map.next_value()?);
                        }
                        GeneratedField::BatchId => {
                            if batch_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batch_id"));
                            }
                            batch_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::CondorId => {
                            if condor_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("condor_id"));
                            }
                            condor_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::CondorLogPath => {
                            if condor_log_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("condor_log_path"));
                            }
                            condor_log_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::CreateOutputSubdir => {
                            if create_output_subdir__.is_some() {
                                return Err(serde::de::Error::duplicate_field("create_output_subdir"));
                            }
                            create_output_subdir__ = Some(map.next_value()?);
                        }
                        GeneratedField::DateSubmitted => {
                            if date_submitted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("date_submitted"));
                            }
                            date_submitted__ = map.next_value()?;
                        }
                        GeneratedField::DateStarted => {
                            if date_started__.is_some() {
                                return Err(serde::de::Error::duplicate_field("date_started"));
                            }
                            date_started__ = map.next_value()?;
                        }
                        GeneratedField::DateCompleted => {
                            if date_completed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("date_completed"));
                            }
                            date_completed__ = map.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map.next_value()?);
                        }
                        GeneratedField::Extra => {
                            if extra__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extra"));
                            }
                            extra__ = map.next_value()?;
                        }
                        GeneratedField::ExecutionTarget => {
                            if execution_target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("execution_target"));
                            }
                            execution_target__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExitCode => {
                            if exit_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exit_code"));
                            }
                            exit_code__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FailureCount => {
                            if failure_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failure_count"));
                            }
                            failure_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FailureThreshold => {
                            if failure_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failure_threshold"));
                            }
                            failure_threshold__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FileMetadata => {
                            if file_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("file-metadata"));
                            }
                            file_metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::FilterFiles => {
                            if filter_files__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter_files"));
                            }
                            filter_files__ = Some(map.next_value()?);
                        }
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = Some(map.next_value()?);
                        }
                        GeneratedField::InputPathListFile => {
                            if input_path_list_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input_path_list"));
                            }
                            input_path_list_file__ = Some(map.next_value()?);
                        }
                        GeneratedField::InputTicketsFile => {
                            if input_tickets_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input_ticket_list"));
                            }
                            input_tickets_file__ = Some(map.next_value()?);
                        }
                        GeneratedField::InvocationId => {
                            if invocation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("invocation_id"));
                            }
                            invocation_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::IrodsBase => {
                            if irods_base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("irods_base"));
                            }
                            irods_base__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::NfsBase => {
                            if nfs_base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nfs_base"));
                            }
                            nfs_base__ = Some(map.next_value()?);
                        }
                        GeneratedField::Notify => {
                            if notify__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notify"));
                            }
                            notify__ = Some(map.next_value()?);
                        }
                        GeneratedField::NowDate => {
                            if now_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("now_date"));
                            }
                            now_date__ = Some(map.next_value()?);
                        }
                        GeneratedField::OutputDir => {
                            if output_dir__.is_some() {
                                return Err(serde::de::Error::duplicate_field("output_dir"));
                            }
                            output_dir__ = Some(map.next_value()?);
                        }
                        GeneratedField::OutputDirTicket => {
                            if output_dir_ticket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("output_dir_ticket"));
                            }
                            output_dir_ticket__ = Some(map.next_value()?);
                        }
                        GeneratedField::OutputTicketFile => {
                            if output_ticket_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("output_ticket_file"));
                            }
                            output_ticket_file__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestType => {
                            if request_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request_type"));
                            }
                            request_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::RunOnNfs => {
                            if run_on_nfs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("run-on-nfs"));
                            }
                            run_on_nfs__ = Some(map.next_value()?);
                        }
                        GeneratedField::SkipParentMetadata => {
                            if skip_parent_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skip-parent-meta"));
                            }
                            skip_parent_metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::Steps => {
                            if steps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("steps"));
                            }
                            steps__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubmissionDate => {
                            if submission_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("submission_date"));
                            }
                            submission_date__ = Some(map.next_value()?);
                        }
                        GeneratedField::Submitter => {
                            if submitter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            submitter__ = Some(map.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_id"));
                            }
                            user_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::UserGroups => {
                            if user_groups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_groups"));
                            }
                            user_groups__ = Some(map.next_value()?);
                        }
                        GeneratedField::UserHome => {
                            if user_home__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_home"));
                            }
                            user_home__ = Some(map.next_value()?);
                        }
                        GeneratedField::WikiUrl => {
                            if wiki_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wiki_url"));
                            }
                            wiki_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::ConfigFile => {
                            if config_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config_file"));
                            }
                            config_file__ = Some(map.next_value()?);
                        }
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                    }
                }
                Ok(AnalysisSubmission {
                    app_description: app_description__.unwrap_or_default(),
                    app_id: app_id__.unwrap_or_default(),
                    app_name: app_name__.unwrap_or_default(),
                    archive_logs: archive_logs__.unwrap_or_default(),
                    batch_id: batch_id__.unwrap_or_default(),
                    condor_id: condor_id__.unwrap_or_default(),
                    condor_log_path: condor_log_path__.unwrap_or_default(),
                    create_output_subdir: create_output_subdir__.unwrap_or_default(),
                    date_submitted: date_submitted__,
                    date_started: date_started__,
                    date_completed: date_completed__,
                    description: description__.unwrap_or_default(),
                    email: email__.unwrap_or_default(),
                    extra: extra__,
                    execution_target: execution_target__.unwrap_or_default(),
                    exit_code: exit_code__.unwrap_or_default(),
                    failure_count: failure_count__.unwrap_or_default(),
                    failure_threshold: failure_threshold__.unwrap_or_default(),
                    file_metadata: file_metadata__.unwrap_or_default(),
                    filter_files: filter_files__.unwrap_or_default(),
                    group: group__.unwrap_or_default(),
                    input_path_list_file: input_path_list_file__.unwrap_or_default(),
                    input_tickets_file: input_tickets_file__.unwrap_or_default(),
                    invocation_id: invocation_id__.unwrap_or_default(),
                    irods_base: irods_base__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    nfs_base: nfs_base__.unwrap_or_default(),
                    notify: notify__.unwrap_or_default(),
                    now_date: now_date__.unwrap_or_default(),
                    output_dir: output_dir__.unwrap_or_default(),
                    output_dir_ticket: output_dir_ticket__.unwrap_or_default(),
                    output_ticket_file: output_ticket_file__.unwrap_or_default(),
                    request_type: request_type__.unwrap_or_default(),
                    run_on_nfs: run_on_nfs__.unwrap_or_default(),
                    skip_parent_metadata: skip_parent_metadata__.unwrap_or_default(),
                    steps: steps__.unwrap_or_default(),
                    submission_date: submission_date__.unwrap_or_default(),
                    submitter: submitter__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    user_groups: user_groups__.unwrap_or_default(),
                    user_home: user_home__.unwrap_or_default(),
                    wiki_url: wiki_url__.unwrap_or_default(),
                    config_file: config_file__.unwrap_or_default(),
                    header: header__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.AnalysisSubmission", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssociateByUuiDs {
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
        if !self.parent_uuid.is_empty() {
            len += 1;
        }
        if !self.child_uuid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.AssociateByUUIDs", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.parent_uuid.is_empty() {
            struct_ser.serialize_field("parent_uuid", &self.parent_uuid)?;
        }
        if !self.child_uuid.is_empty() {
            struct_ser.serialize_field("child_uuid", &self.child_uuid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssociateByUuiDs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "parent_uuid",
            "child_uuid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            ParentUuid,
            ChildUuid,
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
                            "parent_uuid" => Ok(GeneratedField::ParentUuid),
                            "child_uuid" => Ok(GeneratedField::ChildUuid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssociateByUuiDs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.AssociateByUUIDs")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AssociateByUuiDs, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut parent_uuid__ = None;
                let mut child_uuid__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::ParentUuid => {
                            if parent_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent_uuid"));
                            }
                            parent_uuid__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChildUuid => {
                            if child_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("child_uuid"));
                            }
                            child_uuid__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AssociateByUuiDs {
                    header: header__,
                    parent_uuid: parent_uuid__.unwrap_or_default(),
                    child_uuid: child_uuid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.AssociateByUUIDs", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ByUuid {
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
        if !self.uuid.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.ByUUID", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ByUuid {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "uuid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Uuid,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ByUuid;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.ByUUID")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ByUuid, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut uuid__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ByUuid {
                    header: header__,
                    uuid: uuid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.ByUUID", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ByUuidAndUserId {
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
        if !self.uuid.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.ByUUIDAndUserID", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("user_id", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ByUuidAndUserId {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "uuid",
            "user_id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Uuid,
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
                            "uuid" => Ok(GeneratedField::Uuid),
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
            type Value = ByUuidAndUserId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.ByUUIDAndUserID")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ByUuidAndUserId, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut uuid__ = None;
                let mut user_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_id"));
                            }
                            user_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ByUuidAndUserId {
                    header: header__,
                    uuid: uuid__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.ByUUIDAndUserID", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ByUuidAndUsername {
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
        if !self.uuid.is_empty() {
            len += 1;
        }
        if !self.username.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.ByUUIDAndUsername", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ByUuidAndUsername {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "uuid",
            "username",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
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
                            "header" => Ok(GeneratedField::Header),
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
            type Value = ByUuidAndUsername;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.ByUUIDAndUsername")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ByUuidAndUsername, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut uuid__ = None;
                let mut username__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map.next_value()?);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ByUuidAndUsername {
                    header: header__,
                    uuid: uuid__.unwrap_or_default(),
                    username: username__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.ByUUIDAndUsername", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ByUserId {
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
        let mut struct_ser = serializer.serialize_struct("debuff.ByUserID", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("user_id", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ByUserId {
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
            type Value = ByUserId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.ByUserID")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ByUserId, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut user_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_id"));
                            }
                            user_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ByUserId {
                    header: header__,
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.ByUserID", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ByUsername {
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
        let mut struct_ser = serializer.serialize_struct("debuff.ByUsername", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ByUsername {
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
            type Value = ByUsername;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.ByUsername")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ByUsername, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut username__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ByUsername {
                    header: header__,
                    username: username__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.ByUsername", FIELDS, GeneratedVisitor)
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
        if self.plan.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.ChangeSubscriptionRequest", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
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
            "uuid",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Username,
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
                formatter.write_str("struct debuff.ChangeSubscriptionRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ChangeSubscriptionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut username__ = None;
                let mut plan__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map.next_value()?);
                        }
                        GeneratedField::Uuid => {
                            if plan__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            plan__ = map.next_value::<::std::option::Option<_>>()?.map(change_subscription_request::Plan::Uuid);
                        }
                        GeneratedField::Name => {
                            if plan__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            plan__ = map.next_value::<::std::option::Option<_>>()?.map(change_subscription_request::Plan::Name);
                        }
                    }
                }
                Ok(ChangeSubscriptionRequest {
                    header: header__,
                    username: username__.unwrap_or_default(),
                    plan: plan__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.ChangeSubscriptionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Container {
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
        if !self.volumes.is_empty() {
            len += 1;
        }
        if !self.devices.is_empty() {
            len += 1;
        }
        if !self.volumes_from.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.network_mode.is_empty() {
            len += 1;
        }
        if self.cpu_shares != 0 {
            len += 1;
        }
        if self.interactive_apps.is_some() {
            len += 1;
        }
        if self.memory_limit != 0 {
            len += 1;
        }
        if self.min_memory_limit != 0 {
            len += 1;
        }
        if self.max_cpu_cores != 0. {
            len += 1;
        }
        if self.min_cpu_cores != 0. {
            len += 1;
        }
        if self.min_disk_space != 0 {
            len += 1;
        }
        if self.pids_limit != 0 {
            len += 1;
        }
        if self.image.is_some() {
            len += 1;
        }
        if !self.entry_point.is_empty() {
            len += 1;
        }
        if !self.working_dir.is_empty() {
            len += 1;
        }
        if !self.ports.is_empty() {
            len += 1;
        }
        if self.skip_tmp_mount {
            len += 1;
        }
        if self.uid != 0 {
            len += 1;
        }
        if self.header.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.Container", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.volumes.is_empty() {
            struct_ser.serialize_field("container_volumes", &self.volumes)?;
        }
        if !self.devices.is_empty() {
            struct_ser.serialize_field("container_devices", &self.devices)?;
        }
        if !self.volumes_from.is_empty() {
            struct_ser.serialize_field("container_volumes_from", &self.volumes_from)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.network_mode.is_empty() {
            struct_ser.serialize_field("network_mode", &self.network_mode)?;
        }
        if self.cpu_shares != 0 {
            struct_ser.serialize_field("cpu_shares", ToString::to_string(&self.cpu_shares).as_str())?;
        }
        if let Some(v) = self.interactive_apps.as_ref() {
            struct_ser.serialize_field("interactive_apps", v)?;
        }
        if self.memory_limit != 0 {
            struct_ser.serialize_field("memory_limit", ToString::to_string(&self.memory_limit).as_str())?;
        }
        if self.min_memory_limit != 0 {
            struct_ser.serialize_field("min_memory_limit", ToString::to_string(&self.min_memory_limit).as_str())?;
        }
        if self.max_cpu_cores != 0. {
            struct_ser.serialize_field("max_cpu_cores", &self.max_cpu_cores)?;
        }
        if self.min_cpu_cores != 0. {
            struct_ser.serialize_field("min_cpu_cores", &self.min_cpu_cores)?;
        }
        if self.min_disk_space != 0 {
            struct_ser.serialize_field("min_disk_space", ToString::to_string(&self.min_disk_space).as_str())?;
        }
        if self.pids_limit != 0 {
            struct_ser.serialize_field("pids_limit", ToString::to_string(&self.pids_limit).as_str())?;
        }
        if let Some(v) = self.image.as_ref() {
            struct_ser.serialize_field("image", v)?;
        }
        if !self.entry_point.is_empty() {
            struct_ser.serialize_field("entrypoint", &self.entry_point)?;
        }
        if !self.working_dir.is_empty() {
            struct_ser.serialize_field("working_directory", &self.working_dir)?;
        }
        if !self.ports.is_empty() {
            struct_ser.serialize_field("ports", &self.ports)?;
        }
        if self.skip_tmp_mount {
            struct_ser.serialize_field("skip_tmp_mount", &self.skip_tmp_mount)?;
        }
        if self.uid != 0 {
            struct_ser.serialize_field("uid", &self.uid)?;
        }
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Container {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "volumes",
            "container_volumes",
            "devices",
            "container_devices",
            "volumes_from",
            "container_volumes_from",
            "name",
            "network_mode",
            "cpu_shares",
            "interactive_apps",
            "memory_limit",
            "min_memory_limit",
            "max_cpu_cores",
            "min_cpu_cores",
            "min_disk_space",
            "pids_limit",
            "image",
            "entry_point",
            "entrypoint",
            "working_dir",
            "working_directory",
            "ports",
            "skip_tmp_mount",
            "uid",
            "header",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Volumes,
            Devices,
            VolumesFrom,
            Name,
            NetworkMode,
            CpuShares,
            InteractiveApps,
            MemoryLimit,
            MinMemoryLimit,
            MaxCpuCores,
            MinCpuCores,
            MinDiskSpace,
            PidsLimit,
            Image,
            EntryPoint,
            WorkingDir,
            Ports,
            SkipTmpMount,
            Uid,
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
                            "id" => Ok(GeneratedField::Id),
                            "container_volumes" | "volumes" => Ok(GeneratedField::Volumes),
                            "container_devices" | "devices" => Ok(GeneratedField::Devices),
                            "container_volumes_from" | "volumes_from" => Ok(GeneratedField::VolumesFrom),
                            "name" => Ok(GeneratedField::Name),
                            "network_mode" => Ok(GeneratedField::NetworkMode),
                            "cpu_shares" => Ok(GeneratedField::CpuShares),
                            "interactive_apps" => Ok(GeneratedField::InteractiveApps),
                            "memory_limit" => Ok(GeneratedField::MemoryLimit),
                            "min_memory_limit" => Ok(GeneratedField::MinMemoryLimit),
                            "max_cpu_cores" => Ok(GeneratedField::MaxCpuCores),
                            "min_cpu_cores" => Ok(GeneratedField::MinCpuCores),
                            "min_disk_space" => Ok(GeneratedField::MinDiskSpace),
                            "pids_limit" => Ok(GeneratedField::PidsLimit),
                            "image" => Ok(GeneratedField::Image),
                            "entrypoint" | "entry_point" => Ok(GeneratedField::EntryPoint),
                            "working_directory" | "working_dir" => Ok(GeneratedField::WorkingDir),
                            "ports" => Ok(GeneratedField::Ports),
                            "skip_tmp_mount" => Ok(GeneratedField::SkipTmpMount),
                            "uid" => Ok(GeneratedField::Uid),
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
            type Value = Container;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.Container")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Container, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut volumes__ = None;
                let mut devices__ = None;
                let mut volumes_from__ = None;
                let mut name__ = None;
                let mut network_mode__ = None;
                let mut cpu_shares__ = None;
                let mut interactive_apps__ = None;
                let mut memory_limit__ = None;
                let mut min_memory_limit__ = None;
                let mut max_cpu_cores__ = None;
                let mut min_cpu_cores__ = None;
                let mut min_disk_space__ = None;
                let mut pids_limit__ = None;
                let mut image__ = None;
                let mut entry_point__ = None;
                let mut working_dir__ = None;
                let mut ports__ = None;
                let mut skip_tmp_mount__ = None;
                let mut uid__ = None;
                let mut header__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Volumes => {
                            if volumes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("container_volumes"));
                            }
                            volumes__ = Some(map.next_value()?);
                        }
                        GeneratedField::Devices => {
                            if devices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("container_devices"));
                            }
                            devices__ = Some(map.next_value()?);
                        }
                        GeneratedField::VolumesFrom => {
                            if volumes_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("container_volumes_from"));
                            }
                            volumes_from__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::NetworkMode => {
                            if network_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("network_mode"));
                            }
                            network_mode__ = Some(map.next_value()?);
                        }
                        GeneratedField::CpuShares => {
                            if cpu_shares__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cpu_shares"));
                            }
                            cpu_shares__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::InteractiveApps => {
                            if interactive_apps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interactive_apps"));
                            }
                            interactive_apps__ = map.next_value()?;
                        }
                        GeneratedField::MemoryLimit => {
                            if memory_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memory_limit"));
                            }
                            memory_limit__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinMemoryLimit => {
                            if min_memory_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("min_memory_limit"));
                            }
                            min_memory_limit__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxCpuCores => {
                            if max_cpu_cores__.is_some() {
                                return Err(serde::de::Error::duplicate_field("max_cpu_cores"));
                            }
                            max_cpu_cores__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinCpuCores => {
                            if min_cpu_cores__.is_some() {
                                return Err(serde::de::Error::duplicate_field("min_cpu_cores"));
                            }
                            min_cpu_cores__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinDiskSpace => {
                            if min_disk_space__.is_some() {
                                return Err(serde::de::Error::duplicate_field("min_disk_space"));
                            }
                            min_disk_space__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PidsLimit => {
                            if pids_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pids_limit"));
                            }
                            pids_limit__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Image => {
                            if image__.is_some() {
                                return Err(serde::de::Error::duplicate_field("image"));
                            }
                            image__ = map.next_value()?;
                        }
                        GeneratedField::EntryPoint => {
                            if entry_point__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entrypoint"));
                            }
                            entry_point__ = Some(map.next_value()?);
                        }
                        GeneratedField::WorkingDir => {
                            if working_dir__.is_some() {
                                return Err(serde::de::Error::duplicate_field("working_directory"));
                            }
                            working_dir__ = Some(map.next_value()?);
                        }
                        GeneratedField::Ports => {
                            if ports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ports"));
                            }
                            ports__ = Some(map.next_value()?);
                        }
                        GeneratedField::SkipTmpMount => {
                            if skip_tmp_mount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skip_tmp_mount"));
                            }
                            skip_tmp_mount__ = Some(map.next_value()?);
                        }
                        GeneratedField::Uid => {
                            if uid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uid"));
                            }
                            uid__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                    }
                }
                Ok(Container {
                    id: id__.unwrap_or_default(),
                    volumes: volumes__.unwrap_or_default(),
                    devices: devices__.unwrap_or_default(),
                    volumes_from: volumes_from__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    network_mode: network_mode__.unwrap_or_default(),
                    cpu_shares: cpu_shares__.unwrap_or_default(),
                    interactive_apps: interactive_apps__,
                    memory_limit: memory_limit__.unwrap_or_default(),
                    min_memory_limit: min_memory_limit__.unwrap_or_default(),
                    max_cpu_cores: max_cpu_cores__.unwrap_or_default(),
                    min_cpu_cores: min_cpu_cores__.unwrap_or_default(),
                    min_disk_space: min_disk_space__.unwrap_or_default(),
                    pids_limit: pids_limit__.unwrap_or_default(),
                    image: image__,
                    entry_point: entry_point__.unwrap_or_default(),
                    working_dir: working_dir__.unwrap_or_default(),
                    ports: ports__.unwrap_or_default(),
                    skip_tmp_mount: skip_tmp_mount__.unwrap_or_default(),
                    uid: uid__.unwrap_or_default(),
                    header: header__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.Container", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for container::Device {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.host_path.is_empty() {
            len += 1;
        }
        if !self.container_path.is_empty() {
            len += 1;
        }
        if !self.cgroup_permissions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.Container.Device", len)?;
        if !self.host_path.is_empty() {
            struct_ser.serialize_field("host_path", &self.host_path)?;
        }
        if !self.container_path.is_empty() {
            struct_ser.serialize_field("container_path", &self.container_path)?;
        }
        if !self.cgroup_permissions.is_empty() {
            struct_ser.serialize_field("cgroup_permissions", &self.cgroup_permissions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for container::Device {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "host_path",
            "container_path",
            "cgroup_permissions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostPath,
            ContainerPath,
            CgroupPermissions,
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
                            "host_path" => Ok(GeneratedField::HostPath),
                            "container_path" => Ok(GeneratedField::ContainerPath),
                            "cgroup_permissions" => Ok(GeneratedField::CgroupPermissions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = container::Device;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.Container.Device")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<container::Device, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut host_path__ = None;
                let mut container_path__ = None;
                let mut cgroup_permissions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HostPath => {
                            if host_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("host_path"));
                            }
                            host_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::ContainerPath => {
                            if container_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("container_path"));
                            }
                            container_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::CgroupPermissions => {
                            if cgroup_permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cgroup_permissions"));
                            }
                            cgroup_permissions__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(container::Device {
                    host_path: host_path__.unwrap_or_default(),
                    container_path: container_path__.unwrap_or_default(),
                    cgroup_permissions: cgroup_permissions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.Container.Device", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for container::Image {
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
        if !self.tag.is_empty() {
            len += 1;
        }
        if !self.auth.is_empty() {
            len += 1;
        }
        if !self.url.is_empty() {
            len += 1;
        }
        if !self.osg_image_path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.Container.Image", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.tag.is_empty() {
            struct_ser.serialize_field("tag", &self.tag)?;
        }
        if !self.auth.is_empty() {
            struct_ser.serialize_field("auth", &self.auth)?;
        }
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if !self.osg_image_path.is_empty() {
            struct_ser.serialize_field("osg_image_path", &self.osg_image_path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for container::Image {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "tag",
            "auth",
            "url",
            "osg_image_path",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Tag,
            Auth,
            Url,
            OsgImagePath,
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
                            "tag" => Ok(GeneratedField::Tag),
                            "auth" => Ok(GeneratedField::Auth),
                            "url" => Ok(GeneratedField::Url),
                            "osg_image_path" => Ok(GeneratedField::OsgImagePath),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = container::Image;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.Container.Image")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<container::Image, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut tag__ = None;
                let mut auth__ = None;
                let mut url__ = None;
                let mut osg_image_path__ = None;
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
                        GeneratedField::Tag => {
                            if tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            tag__ = Some(map.next_value()?);
                        }
                        GeneratedField::Auth => {
                            if auth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("auth"));
                            }
                            auth__ = Some(map.next_value()?);
                        }
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map.next_value()?);
                        }
                        GeneratedField::OsgImagePath => {
                            if osg_image_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("osg_image_path"));
                            }
                            osg_image_path__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(container::Image {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    tag: tag__.unwrap_or_default(),
                    auth: auth__.unwrap_or_default(),
                    url: url__.unwrap_or_default(),
                    osg_image_path: osg_image_path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.Container.Image", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for container::Port {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.host_port != 0 {
            len += 1;
        }
        if self.container_port != 0 {
            len += 1;
        }
        if self.bind_to_host {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.Container.Port", len)?;
        if self.host_port != 0 {
            struct_ser.serialize_field("host_port", &self.host_port)?;
        }
        if self.container_port != 0 {
            struct_ser.serialize_field("container_port", &self.container_port)?;
        }
        if self.bind_to_host {
            struct_ser.serialize_field("bind_to_host", &self.bind_to_host)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for container::Port {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "host_port",
            "container_port",
            "bind_to_host",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostPort,
            ContainerPort,
            BindToHost,
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
                            "host_port" => Ok(GeneratedField::HostPort),
                            "container_port" => Ok(GeneratedField::ContainerPort),
                            "bind_to_host" => Ok(GeneratedField::BindToHost),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = container::Port;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.Container.Port")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<container::Port, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut host_port__ = None;
                let mut container_port__ = None;
                let mut bind_to_host__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HostPort => {
                            if host_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("host_port"));
                            }
                            host_port__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ContainerPort => {
                            if container_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("container_port"));
                            }
                            container_port__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BindToHost => {
                            if bind_to_host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bind_to_host"));
                            }
                            bind_to_host__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(container::Port {
                    host_port: host_port__.unwrap_or_default(),
                    container_port: container_port__.unwrap_or_default(),
                    bind_to_host: bind_to_host__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.Container.Port", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for container::Volume {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.host_path.is_empty() {
            len += 1;
        }
        if !self.container_path.is_empty() {
            len += 1;
        }
        if self.read_only {
            len += 1;
        }
        if !self.mode.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.Container.Volume", len)?;
        if !self.host_path.is_empty() {
            struct_ser.serialize_field("host_path", &self.host_path)?;
        }
        if !self.container_path.is_empty() {
            struct_ser.serialize_field("container_path", &self.container_path)?;
        }
        if self.read_only {
            struct_ser.serialize_field("read_only", &self.read_only)?;
        }
        if !self.mode.is_empty() {
            struct_ser.serialize_field("mode", &self.mode)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for container::Volume {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "host_path",
            "container_path",
            "read_only",
            "mode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostPath,
            ContainerPath,
            ReadOnly,
            Mode,
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
                            "host_path" => Ok(GeneratedField::HostPath),
                            "container_path" => Ok(GeneratedField::ContainerPath),
                            "read_only" => Ok(GeneratedField::ReadOnly),
                            "mode" => Ok(GeneratedField::Mode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = container::Volume;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.Container.Volume")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<container::Volume, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut host_path__ = None;
                let mut container_path__ = None;
                let mut read_only__ = None;
                let mut mode__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HostPath => {
                            if host_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("host_path"));
                            }
                            host_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::ContainerPath => {
                            if container_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("container_path"));
                            }
                            container_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::ReadOnly => {
                            if read_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("read_only"));
                            }
                            read_only__ = Some(map.next_value()?);
                        }
                        GeneratedField::Mode => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            mode__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(container::Volume {
                    host_path: host_path__.unwrap_or_default(),
                    container_path: container_path__.unwrap_or_default(),
                    read_only: read_only__.unwrap_or_default(),
                    mode: mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.Container.Volume", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for container::VolumesFrom {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tag.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.auth.is_empty() {
            len += 1;
        }
        if !self.name_prefix.is_empty() {
            len += 1;
        }
        if !self.url.is_empty() {
            len += 1;
        }
        if !self.host_path.is_empty() {
            len += 1;
        }
        if !self.container_path.is_empty() {
            len += 1;
        }
        if self.read_only {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.Container.VolumesFrom", len)?;
        if !self.tag.is_empty() {
            struct_ser.serialize_field("tag", &self.tag)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.auth.is_empty() {
            struct_ser.serialize_field("auth", &self.auth)?;
        }
        if !self.name_prefix.is_empty() {
            struct_ser.serialize_field("name_prefix", &self.name_prefix)?;
        }
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if !self.host_path.is_empty() {
            struct_ser.serialize_field("host_path", &self.host_path)?;
        }
        if !self.container_path.is_empty() {
            struct_ser.serialize_field("container_path", &self.container_path)?;
        }
        if self.read_only {
            struct_ser.serialize_field("read_only", &self.read_only)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for container::VolumesFrom {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tag",
            "name",
            "auth",
            "name_prefix",
            "url",
            "host_path",
            "container_path",
            "read_only",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tag,
            Name,
            Auth,
            NamePrefix,
            Url,
            HostPath,
            ContainerPath,
            ReadOnly,
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
                            "tag" => Ok(GeneratedField::Tag),
                            "name" => Ok(GeneratedField::Name),
                            "auth" => Ok(GeneratedField::Auth),
                            "name_prefix" => Ok(GeneratedField::NamePrefix),
                            "url" => Ok(GeneratedField::Url),
                            "host_path" => Ok(GeneratedField::HostPath),
                            "container_path" => Ok(GeneratedField::ContainerPath),
                            "read_only" => Ok(GeneratedField::ReadOnly),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = container::VolumesFrom;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.Container.VolumesFrom")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<container::VolumesFrom, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tag__ = None;
                let mut name__ = None;
                let mut auth__ = None;
                let mut name_prefix__ = None;
                let mut url__ = None;
                let mut host_path__ = None;
                let mut container_path__ = None;
                let mut read_only__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Tag => {
                            if tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            tag__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Auth => {
                            if auth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("auth"));
                            }
                            auth__ = Some(map.next_value()?);
                        }
                        GeneratedField::NamePrefix => {
                            if name_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name_prefix"));
                            }
                            name_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map.next_value()?);
                        }
                        GeneratedField::HostPath => {
                            if host_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("host_path"));
                            }
                            host_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::ContainerPath => {
                            if container_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("container_path"));
                            }
                            container_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::ReadOnly => {
                            if read_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("read_only"));
                            }
                            read_only__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(container::VolumesFrom {
                    tag: tag__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    auth: auth__.unwrap_or_default(),
                    name_prefix: name_prefix__.unwrap_or_default(),
                    url: url__.unwrap_or_default(),
                    host_path: host_path__.unwrap_or_default(),
                    container_path: container_path__.unwrap_or_default(),
                    read_only: read_only__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.Container.VolumesFrom", FIELDS, GeneratedVisitor)
    }
}
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
        let mut struct_ser = serializer.serialize_struct("debuff.DNSCheckResult", len)?;
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
                formatter.write_str("struct debuff.DNSCheckResult")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DnsCheckResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut lookups__ = None;
                let mut node__ = None;
                let mut date_sent__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::Lookups => {
                            if lookups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lookups"));
                            }
                            lookups__ = Some(map.next_value()?);
                        }
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = Some(map.next_value()?);
                        }
                        GeneratedField::DateSent => {
                            if date_sent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("date_sent"));
                            }
                            date_sent__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.DNSCheckResult", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.DNSLookup", len)?;
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
                formatter.write_str("struct debuff.DNSLookup")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DnsLookup, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut host__ = None;
                let mut addresses__ = None;
                let mut r#type__ = None;
                let mut error__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Host => {
                            if host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            host__ = Some(map.next_value()?);
                        }
                        GeneratedField::Addresses => {
                            if addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addresses"));
                            }
                            addresses__ = Some(map.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value()?);
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.DNSLookup", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unset => "UNSET",
            Self::Unspecified => "UNSPECIFIED",
            Self::Internal => "INTERNAL",
            Self::NotFound => "NOT_FOUND",
            Self::BadRequest => "BAD_REQUEST",
            Self::MarshalFailure => "MARSHAL_FAILURE",
            Self::UnmarshalFailure => "UNMARSHAL_FAILURE",
            Self::ParameterMissing => "PARAMETER_MISSING",
            Self::ParameterInvalid => "PARAMETER_INVALID",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ErrorCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNSET",
            "UNSPECIFIED",
            "INTERNAL",
            "NOT_FOUND",
            "BAD_REQUEST",
            "MARSHAL_FAILURE",
            "UNMARSHAL_FAILURE",
            "PARAMETER_MISSING",
            "PARAMETER_INVALID",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ErrorCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ErrorCode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ErrorCode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNSET" => Ok(ErrorCode::Unset),
                    "UNSPECIFIED" => Ok(ErrorCode::Unspecified),
                    "INTERNAL" => Ok(ErrorCode::Internal),
                    "NOT_FOUND" => Ok(ErrorCode::NotFound),
                    "BAD_REQUEST" => Ok(ErrorCode::BadRequest),
                    "MARSHAL_FAILURE" => Ok(ErrorCode::MarshalFailure),
                    "UNMARSHAL_FAILURE" => Ok(ErrorCode::UnmarshalFailure),
                    "PARAMETER_MISSING" => Ok(ErrorCode::ParameterMissing),
                    "PARAMETER_INVALID" => Ok(ErrorCode::ParameterInvalid),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Extra {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ht_condor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.Extra", len)?;
        if let Some(v) = self.ht_condor.as_ref() {
            struct_ser.serialize_field("htCondor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Extra {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ht_condor",
            "htCondor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HtCondor,
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
                            "htCondor" | "ht_condor" => Ok(GeneratedField::HtCondor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Extra;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.Extra")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Extra, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ht_condor__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HtCondor => {
                            if ht_condor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("htCondor"));
                            }
                            ht_condor__ = map.next_value()?;
                        }
                    }
                }
                Ok(Extra {
                    ht_condor: ht_condor__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.Extra", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.attribute.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.unit.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.FileMetadata", len)?;
        if !self.attribute.is_empty() {
            struct_ser.serialize_field("attribute", &self.attribute)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attribute",
            "value",
            "unit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Attribute,
            Value,
            Unit,
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
                            "attribute" => Ok(GeneratedField::Attribute),
                            "value" => Ok(GeneratedField::Value),
                            "unit" => Ok(GeneratedField::Unit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.FileMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute__ = None;
                let mut value__ = None;
                let mut unit__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Attribute => {
                            if attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attribute"));
                            }
                            attribute__ = Some(map.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FileMetadata {
                    attribute: attribute__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.FileMetadata", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.GetUsages", len)?;
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
                formatter.write_str("struct debuff.GetUsages")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetUsages, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut username__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetUsages {
                    header: header__,
                    username: username__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.GetUsages", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HtCondorExtraInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.extra_requirements.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.HTCondorExtraInfo", len)?;
        if !self.extra_requirements.is_empty() {
            struct_ser.serialize_field("extraRequirements", &self.extra_requirements)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HtCondorExtraInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "extra_requirements",
            "extraRequirements",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExtraRequirements,
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
                            "extraRequirements" | "extra_requirements" => Ok(GeneratedField::ExtraRequirements),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HtCondorExtraInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.HTCondorExtraInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HtCondorExtraInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut extra_requirements__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExtraRequirements => {
                            if extra_requirements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extraRequirements"));
                            }
                            extra_requirements__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(HtCondorExtraInfo {
                    extra_requirements: extra_requirements__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.HTCondorExtraInfo", FIELDS, GeneratedVisitor)
    }
}
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
        let mut struct_ser = serializer.serialize_struct("debuff.Header", len)?;
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
                formatter.write_str("struct debuff.Header")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Header, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut map__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Map => {
                            if map__.is_some() {
                                return Err(serde::de::Error::duplicate_field("map"));
                            }
                            map__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(Header {
                    map: map__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.Header", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.Header.Value", len)?;
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
                formatter.write_str("struct debuff.Header.Value")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<header::Value, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(header::Value {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.Header.Value", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.Heartbeat", len)?;
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
                formatter.write_str("struct debuff.Heartbeat")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Heartbeat, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut node__ = None;
                let mut date_sent__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = Some(map.next_value()?);
                        }
                        GeneratedField::DateSent => {
                            if date_sent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("date_sent"));
                            }
                            date_sent__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.Heartbeat", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InteractiveApps {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.proxy_image.is_empty() {
            len += 1;
        }
        if !self.proxy_name.is_empty() {
            len += 1;
        }
        if !self.frontend_url.is_empty() {
            len += 1;
        }
        if !self.cas_url.is_empty() {
            len += 1;
        }
        if !self.cas_validate.is_empty() {
            len += 1;
        }
        if !self.ssl_cert_path.is_empty() {
            len += 1;
        }
        if !self.ssl_key_path.is_empty() {
            len += 1;
        }
        if !self.websocket_path.is_empty() {
            len += 1;
        }
        if !self.websocket_port.is_empty() {
            len += 1;
        }
        if !self.websocket_proto.is_empty() {
            len += 1;
        }
        if !self.backend_url.is_empty() {
            len += 1;
        }
        if self.header.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.InteractiveApps", len)?;
        if !self.proxy_image.is_empty() {
            struct_ser.serialize_field("proxyImage", &self.proxy_image)?;
        }
        if !self.proxy_name.is_empty() {
            struct_ser.serialize_field("proxyName", &self.proxy_name)?;
        }
        if !self.frontend_url.is_empty() {
            struct_ser.serialize_field("frontendUrl", &self.frontend_url)?;
        }
        if !self.cas_url.is_empty() {
            struct_ser.serialize_field("casUrl", &self.cas_url)?;
        }
        if !self.cas_validate.is_empty() {
            struct_ser.serialize_field("casValidate", &self.cas_validate)?;
        }
        if !self.ssl_cert_path.is_empty() {
            struct_ser.serialize_field("sslCertPath", &self.ssl_cert_path)?;
        }
        if !self.ssl_key_path.is_empty() {
            struct_ser.serialize_field("sslKeyPath", &self.ssl_key_path)?;
        }
        if !self.websocket_path.is_empty() {
            struct_ser.serialize_field("websocketPath", &self.websocket_path)?;
        }
        if !self.websocket_port.is_empty() {
            struct_ser.serialize_field("websocketPort", &self.websocket_port)?;
        }
        if !self.websocket_proto.is_empty() {
            struct_ser.serialize_field("websocketProto", &self.websocket_proto)?;
        }
        if !self.backend_url.is_empty() {
            struct_ser.serialize_field("backendUrl", &self.backend_url)?;
        }
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InteractiveApps {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "proxy_image",
            "proxyImage",
            "proxy_name",
            "proxyName",
            "frontend_url",
            "frontendUrl",
            "cas_url",
            "casUrl",
            "cas_validate",
            "casValidate",
            "ssl_cert_path",
            "sslCertPath",
            "ssl_key_path",
            "sslKeyPath",
            "websocket_path",
            "websocketPath",
            "websocket_port",
            "websocketPort",
            "websocket_proto",
            "websocketProto",
            "backend_url",
            "backendUrl",
            "header",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProxyImage,
            ProxyName,
            FrontendUrl,
            CasUrl,
            CasValidate,
            SslCertPath,
            SslKeyPath,
            WebsocketPath,
            WebsocketPort,
            WebsocketProto,
            BackendUrl,
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
                            "proxyImage" | "proxy_image" => Ok(GeneratedField::ProxyImage),
                            "proxyName" | "proxy_name" => Ok(GeneratedField::ProxyName),
                            "frontendUrl" | "frontend_url" => Ok(GeneratedField::FrontendUrl),
                            "casUrl" | "cas_url" => Ok(GeneratedField::CasUrl),
                            "casValidate" | "cas_validate" => Ok(GeneratedField::CasValidate),
                            "sslCertPath" | "ssl_cert_path" => Ok(GeneratedField::SslCertPath),
                            "sslKeyPath" | "ssl_key_path" => Ok(GeneratedField::SslKeyPath),
                            "websocketPath" | "websocket_path" => Ok(GeneratedField::WebsocketPath),
                            "websocketPort" | "websocket_port" => Ok(GeneratedField::WebsocketPort),
                            "websocketProto" | "websocket_proto" => Ok(GeneratedField::WebsocketProto),
                            "backendUrl" | "backend_url" => Ok(GeneratedField::BackendUrl),
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
            type Value = InteractiveApps;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.InteractiveApps")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InteractiveApps, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut proxy_image__ = None;
                let mut proxy_name__ = None;
                let mut frontend_url__ = None;
                let mut cas_url__ = None;
                let mut cas_validate__ = None;
                let mut ssl_cert_path__ = None;
                let mut ssl_key_path__ = None;
                let mut websocket_path__ = None;
                let mut websocket_port__ = None;
                let mut websocket_proto__ = None;
                let mut backend_url__ = None;
                let mut header__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProxyImage => {
                            if proxy_image__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proxyImage"));
                            }
                            proxy_image__ = Some(map.next_value()?);
                        }
                        GeneratedField::ProxyName => {
                            if proxy_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proxyName"));
                            }
                            proxy_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::FrontendUrl => {
                            if frontend_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("frontendUrl"));
                            }
                            frontend_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::CasUrl => {
                            if cas_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("casUrl"));
                            }
                            cas_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::CasValidate => {
                            if cas_validate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("casValidate"));
                            }
                            cas_validate__ = Some(map.next_value()?);
                        }
                        GeneratedField::SslCertPath => {
                            if ssl_cert_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sslCertPath"));
                            }
                            ssl_cert_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::SslKeyPath => {
                            if ssl_key_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sslKeyPath"));
                            }
                            ssl_key_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::WebsocketPath => {
                            if websocket_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websocketPath"));
                            }
                            websocket_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::WebsocketPort => {
                            if websocket_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websocketPort"));
                            }
                            websocket_port__ = Some(map.next_value()?);
                        }
                        GeneratedField::WebsocketProto => {
                            if websocket_proto__.is_some() {
                                return Err(serde::de::Error::duplicate_field("websocketProto"));
                            }
                            websocket_proto__ = Some(map.next_value()?);
                        }
                        GeneratedField::BackendUrl => {
                            if backend_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backendUrl"));
                            }
                            backend_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                    }
                }
                Ok(InteractiveApps {
                    proxy_image: proxy_image__.unwrap_or_default(),
                    proxy_name: proxy_name__.unwrap_or_default(),
                    frontend_url: frontend_url__.unwrap_or_default(),
                    cas_url: cas_url__.unwrap_or_default(),
                    cas_validate: cas_validate__.unwrap_or_default(),
                    ssl_cert_path: ssl_cert_path__.unwrap_or_default(),
                    ssl_key_path: ssl_key_path__.unwrap_or_default(),
                    websocket_path: websocket_path__.unwrap_or_default(),
                    websocket_port: websocket_port__.unwrap_or_default(),
                    websocket_proto: websocket_proto__.unwrap_or_default(),
                    backend_url: backend_url__.unwrap_or_default(),
                    header: header__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.InteractiveApps", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.IsOverage", len)?;
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
                formatter.write_str("struct debuff.IsOverage")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IsOverage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut is_overage__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::IsOverage => {
                            if is_overage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("is_overage"));
                            }
                            is_overage__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.IsOverage", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.IsOverageRequest", len)?;
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
                formatter.write_str("struct debuff.IsOverageRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IsOverageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut username__ = None;
                let mut resource_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResourceName => {
                            if resource_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_name"));
                            }
                            resource_name__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.IsOverageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Job {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.app_description.is_empty() {
            len += 1;
        }
        if !self.app_id.is_empty() {
            len += 1;
        }
        if !self.app_name.is_empty() {
            len += 1;
        }
        if self.archive_logs {
            len += 1;
        }
        if !self.batch_id.is_empty() {
            len += 1;
        }
        if !self.condor_id.is_empty() {
            len += 1;
        }
        if !self.condor_log_path.is_empty() {
            len += 1;
        }
        if self.create_output_subdir {
            len += 1;
        }
        if self.date_submitted.is_some() {
            len += 1;
        }
        if self.date_started.is_some() {
            len += 1;
        }
        if self.date_completed.is_some() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.email.is_empty() {
            len += 1;
        }
        if self.extra.is_some() {
            len += 1;
        }
        if !self.execution_target.is_empty() {
            len += 1;
        }
        if self.exit_code != 0 {
            len += 1;
        }
        if self.failure_count != 0 {
            len += 1;
        }
        if self.failure_threshold != 0 {
            len += 1;
        }
        if !self.file_metadata.is_empty() {
            len += 1;
        }
        if !self.filter_files.is_empty() {
            len += 1;
        }
        if !self.group.is_empty() {
            len += 1;
        }
        if !self.input_path_list_file.is_empty() {
            len += 1;
        }
        if !self.input_tickets_file.is_empty() {
            len += 1;
        }
        if !self.invocation_id.is_empty() {
            len += 1;
        }
        if !self.irods_base.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.nfs_base.is_empty() {
            len += 1;
        }
        if self.notify {
            len += 1;
        }
        if !self.now_date.is_empty() {
            len += 1;
        }
        if !self.output_dir.is_empty() {
            len += 1;
        }
        if !self.output_dir_ticket.is_empty() {
            len += 1;
        }
        if !self.output_ticket_file.is_empty() {
            len += 1;
        }
        if !self.request_type.is_empty() {
            len += 1;
        }
        if self.run_on_nfs {
            len += 1;
        }
        if self.skip_parent_metadata {
            len += 1;
        }
        if !self.steps.is_empty() {
            len += 1;
        }
        if !self.submission_date.is_empty() {
            len += 1;
        }
        if !self.submitter.is_empty() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if !self.user_groups.is_empty() {
            len += 1;
        }
        if !self.user_home.is_empty() {
            len += 1;
        }
        if !self.wiki_url.is_empty() {
            len += 1;
        }
        if !self.config_file.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.Job", len)?;
        if !self.app_description.is_empty() {
            struct_ser.serialize_field("app_description", &self.app_description)?;
        }
        if !self.app_id.is_empty() {
            struct_ser.serialize_field("app_id", &self.app_id)?;
        }
        if !self.app_name.is_empty() {
            struct_ser.serialize_field("app_name", &self.app_name)?;
        }
        if self.archive_logs {
            struct_ser.serialize_field("archive_logs", &self.archive_logs)?;
        }
        if !self.batch_id.is_empty() {
            struct_ser.serialize_field("batch_id", &self.batch_id)?;
        }
        if !self.condor_id.is_empty() {
            struct_ser.serialize_field("condor_id", &self.condor_id)?;
        }
        if !self.condor_log_path.is_empty() {
            struct_ser.serialize_field("condor_log_path", &self.condor_log_path)?;
        }
        if self.create_output_subdir {
            struct_ser.serialize_field("create_output_subdir", &self.create_output_subdir)?;
        }
        if let Some(v) = self.date_submitted.as_ref() {
            struct_ser.serialize_field("date_submitted", v)?;
        }
        if let Some(v) = self.date_started.as_ref() {
            struct_ser.serialize_field("date_started", v)?;
        }
        if let Some(v) = self.date_completed.as_ref() {
            struct_ser.serialize_field("date_completed", v)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if let Some(v) = self.extra.as_ref() {
            struct_ser.serialize_field("extra", v)?;
        }
        if !self.execution_target.is_empty() {
            struct_ser.serialize_field("execution_target", &self.execution_target)?;
        }
        if self.exit_code != 0 {
            struct_ser.serialize_field("exit_code", &self.exit_code)?;
        }
        if self.failure_count != 0 {
            struct_ser.serialize_field("failure_count", ToString::to_string(&self.failure_count).as_str())?;
        }
        if self.failure_threshold != 0 {
            struct_ser.serialize_field("failure_threshold", ToString::to_string(&self.failure_threshold).as_str())?;
        }
        if !self.file_metadata.is_empty() {
            struct_ser.serialize_field("file-metadata", &self.file_metadata)?;
        }
        if !self.filter_files.is_empty() {
            struct_ser.serialize_field("filter_files", &self.filter_files)?;
        }
        if !self.group.is_empty() {
            struct_ser.serialize_field("group", &self.group)?;
        }
        if !self.input_path_list_file.is_empty() {
            struct_ser.serialize_field("input_path_list", &self.input_path_list_file)?;
        }
        if !self.input_tickets_file.is_empty() {
            struct_ser.serialize_field("input_ticket_list", &self.input_tickets_file)?;
        }
        if !self.invocation_id.is_empty() {
            struct_ser.serialize_field("invocation_id", &self.invocation_id)?;
        }
        if !self.irods_base.is_empty() {
            struct_ser.serialize_field("irods_base", &self.irods_base)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.nfs_base.is_empty() {
            struct_ser.serialize_field("nfs_base", &self.nfs_base)?;
        }
        if self.notify {
            struct_ser.serialize_field("notify", &self.notify)?;
        }
        if !self.now_date.is_empty() {
            struct_ser.serialize_field("now_date", &self.now_date)?;
        }
        if !self.output_dir.is_empty() {
            struct_ser.serialize_field("output_dir", &self.output_dir)?;
        }
        if !self.output_dir_ticket.is_empty() {
            struct_ser.serialize_field("output_dir_ticket", &self.output_dir_ticket)?;
        }
        if !self.output_ticket_file.is_empty() {
            struct_ser.serialize_field("output_ticket_file", &self.output_ticket_file)?;
        }
        if !self.request_type.is_empty() {
            struct_ser.serialize_field("request_type", &self.request_type)?;
        }
        if self.run_on_nfs {
            struct_ser.serialize_field("run-on-nfs", &self.run_on_nfs)?;
        }
        if self.skip_parent_metadata {
            struct_ser.serialize_field("skip-parent-meta", &self.skip_parent_metadata)?;
        }
        if !self.steps.is_empty() {
            struct_ser.serialize_field("steps", &self.steps)?;
        }
        if !self.submission_date.is_empty() {
            struct_ser.serialize_field("submission_date", &self.submission_date)?;
        }
        if !self.submitter.is_empty() {
            struct_ser.serialize_field("username", &self.submitter)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("user_id", &self.user_id)?;
        }
        if !self.user_groups.is_empty() {
            struct_ser.serialize_field("user_groups", &self.user_groups)?;
        }
        if !self.user_home.is_empty() {
            struct_ser.serialize_field("user_home", &self.user_home)?;
        }
        if !self.wiki_url.is_empty() {
            struct_ser.serialize_field("wiki_url", &self.wiki_url)?;
        }
        if !self.config_file.is_empty() {
            struct_ser.serialize_field("config_file", &self.config_file)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Job {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "app_description",
            "app_id",
            "app_name",
            "archive_logs",
            "batch_id",
            "condor_id",
            "condor_log_path",
            "create_output_subdir",
            "date_submitted",
            "date_started",
            "date_completed",
            "description",
            "email",
            "extra",
            "execution_target",
            "exit_code",
            "failure_count",
            "failure_threshold",
            "file_metadata",
            "file-metadata",
            "filter_files",
            "group",
            "input_path_list_file",
            "input_path_list",
            "input_tickets_file",
            "input_ticket_list",
            "invocation_id",
            "irods_base",
            "name",
            "nfs_base",
            "notify",
            "now_date",
            "output_dir",
            "output_dir_ticket",
            "output_ticket_file",
            "request_type",
            "run_on_nfs",
            "run-on-nfs",
            "skip_parent_metadata",
            "skip-parent-meta",
            "steps",
            "submission_date",
            "submitter",
            "username",
            "type",
            "user_id",
            "user_groups",
            "user_home",
            "wiki_url",
            "config_file",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AppDescription,
            AppId,
            AppName,
            ArchiveLogs,
            BatchId,
            CondorId,
            CondorLogPath,
            CreateOutputSubdir,
            DateSubmitted,
            DateStarted,
            DateCompleted,
            Description,
            Email,
            Extra,
            ExecutionTarget,
            ExitCode,
            FailureCount,
            FailureThreshold,
            FileMetadata,
            FilterFiles,
            Group,
            InputPathListFile,
            InputTicketsFile,
            InvocationId,
            IrodsBase,
            Name,
            NfsBase,
            Notify,
            NowDate,
            OutputDir,
            OutputDirTicket,
            OutputTicketFile,
            RequestType,
            RunOnNfs,
            SkipParentMetadata,
            Steps,
            SubmissionDate,
            Submitter,
            Type,
            UserId,
            UserGroups,
            UserHome,
            WikiUrl,
            ConfigFile,
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
                            "app_description" => Ok(GeneratedField::AppDescription),
                            "app_id" => Ok(GeneratedField::AppId),
                            "app_name" => Ok(GeneratedField::AppName),
                            "archive_logs" => Ok(GeneratedField::ArchiveLogs),
                            "batch_id" => Ok(GeneratedField::BatchId),
                            "condor_id" => Ok(GeneratedField::CondorId),
                            "condor_log_path" => Ok(GeneratedField::CondorLogPath),
                            "create_output_subdir" => Ok(GeneratedField::CreateOutputSubdir),
                            "date_submitted" => Ok(GeneratedField::DateSubmitted),
                            "date_started" => Ok(GeneratedField::DateStarted),
                            "date_completed" => Ok(GeneratedField::DateCompleted),
                            "description" => Ok(GeneratedField::Description),
                            "email" => Ok(GeneratedField::Email),
                            "extra" => Ok(GeneratedField::Extra),
                            "execution_target" => Ok(GeneratedField::ExecutionTarget),
                            "exit_code" => Ok(GeneratedField::ExitCode),
                            "failure_count" => Ok(GeneratedField::FailureCount),
                            "failure_threshold" => Ok(GeneratedField::FailureThreshold),
                            "file-metadata" | "file_metadata" => Ok(GeneratedField::FileMetadata),
                            "filter_files" => Ok(GeneratedField::FilterFiles),
                            "group" => Ok(GeneratedField::Group),
                            "input_path_list" | "input_path_list_file" => Ok(GeneratedField::InputPathListFile),
                            "input_ticket_list" | "input_tickets_file" => Ok(GeneratedField::InputTicketsFile),
                            "invocation_id" => Ok(GeneratedField::InvocationId),
                            "irods_base" => Ok(GeneratedField::IrodsBase),
                            "name" => Ok(GeneratedField::Name),
                            "nfs_base" => Ok(GeneratedField::NfsBase),
                            "notify" => Ok(GeneratedField::Notify),
                            "now_date" => Ok(GeneratedField::NowDate),
                            "output_dir" => Ok(GeneratedField::OutputDir),
                            "output_dir_ticket" => Ok(GeneratedField::OutputDirTicket),
                            "output_ticket_file" => Ok(GeneratedField::OutputTicketFile),
                            "request_type" => Ok(GeneratedField::RequestType),
                            "run-on-nfs" | "run_on_nfs" => Ok(GeneratedField::RunOnNfs),
                            "skip-parent-meta" | "skip_parent_metadata" => Ok(GeneratedField::SkipParentMetadata),
                            "steps" => Ok(GeneratedField::Steps),
                            "submission_date" => Ok(GeneratedField::SubmissionDate),
                            "username" | "submitter" => Ok(GeneratedField::Submitter),
                            "type" => Ok(GeneratedField::Type),
                            "user_id" => Ok(GeneratedField::UserId),
                            "user_groups" => Ok(GeneratedField::UserGroups),
                            "user_home" => Ok(GeneratedField::UserHome),
                            "wiki_url" => Ok(GeneratedField::WikiUrl),
                            "config_file" => Ok(GeneratedField::ConfigFile),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Job;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.Job")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Job, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut app_description__ = None;
                let mut app_id__ = None;
                let mut app_name__ = None;
                let mut archive_logs__ = None;
                let mut batch_id__ = None;
                let mut condor_id__ = None;
                let mut condor_log_path__ = None;
                let mut create_output_subdir__ = None;
                let mut date_submitted__ = None;
                let mut date_started__ = None;
                let mut date_completed__ = None;
                let mut description__ = None;
                let mut email__ = None;
                let mut extra__ = None;
                let mut execution_target__ = None;
                let mut exit_code__ = None;
                let mut failure_count__ = None;
                let mut failure_threshold__ = None;
                let mut file_metadata__ = None;
                let mut filter_files__ = None;
                let mut group__ = None;
                let mut input_path_list_file__ = None;
                let mut input_tickets_file__ = None;
                let mut invocation_id__ = None;
                let mut irods_base__ = None;
                let mut name__ = None;
                let mut nfs_base__ = None;
                let mut notify__ = None;
                let mut now_date__ = None;
                let mut output_dir__ = None;
                let mut output_dir_ticket__ = None;
                let mut output_ticket_file__ = None;
                let mut request_type__ = None;
                let mut run_on_nfs__ = None;
                let mut skip_parent_metadata__ = None;
                let mut steps__ = None;
                let mut submission_date__ = None;
                let mut submitter__ = None;
                let mut r#type__ = None;
                let mut user_id__ = None;
                let mut user_groups__ = None;
                let mut user_home__ = None;
                let mut wiki_url__ = None;
                let mut config_file__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AppDescription => {
                            if app_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app_description"));
                            }
                            app_description__ = Some(map.next_value()?);
                        }
                        GeneratedField::AppId => {
                            if app_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app_id"));
                            }
                            app_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::AppName => {
                            if app_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app_name"));
                            }
                            app_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::ArchiveLogs => {
                            if archive_logs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archive_logs"));
                            }
                            archive_logs__ = Some(map.next_value()?);
                        }
                        GeneratedField::BatchId => {
                            if batch_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batch_id"));
                            }
                            batch_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::CondorId => {
                            if condor_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("condor_id"));
                            }
                            condor_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::CondorLogPath => {
                            if condor_log_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("condor_log_path"));
                            }
                            condor_log_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::CreateOutputSubdir => {
                            if create_output_subdir__.is_some() {
                                return Err(serde::de::Error::duplicate_field("create_output_subdir"));
                            }
                            create_output_subdir__ = Some(map.next_value()?);
                        }
                        GeneratedField::DateSubmitted => {
                            if date_submitted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("date_submitted"));
                            }
                            date_submitted__ = map.next_value()?;
                        }
                        GeneratedField::DateStarted => {
                            if date_started__.is_some() {
                                return Err(serde::de::Error::duplicate_field("date_started"));
                            }
                            date_started__ = map.next_value()?;
                        }
                        GeneratedField::DateCompleted => {
                            if date_completed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("date_completed"));
                            }
                            date_completed__ = map.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map.next_value()?);
                        }
                        GeneratedField::Extra => {
                            if extra__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extra"));
                            }
                            extra__ = map.next_value()?;
                        }
                        GeneratedField::ExecutionTarget => {
                            if execution_target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("execution_target"));
                            }
                            execution_target__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExitCode => {
                            if exit_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exit_code"));
                            }
                            exit_code__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FailureCount => {
                            if failure_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failure_count"));
                            }
                            failure_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FailureThreshold => {
                            if failure_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failure_threshold"));
                            }
                            failure_threshold__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FileMetadata => {
                            if file_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("file-metadata"));
                            }
                            file_metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::FilterFiles => {
                            if filter_files__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter_files"));
                            }
                            filter_files__ = Some(map.next_value()?);
                        }
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = Some(map.next_value()?);
                        }
                        GeneratedField::InputPathListFile => {
                            if input_path_list_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input_path_list"));
                            }
                            input_path_list_file__ = Some(map.next_value()?);
                        }
                        GeneratedField::InputTicketsFile => {
                            if input_tickets_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input_ticket_list"));
                            }
                            input_tickets_file__ = Some(map.next_value()?);
                        }
                        GeneratedField::InvocationId => {
                            if invocation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("invocation_id"));
                            }
                            invocation_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::IrodsBase => {
                            if irods_base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("irods_base"));
                            }
                            irods_base__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::NfsBase => {
                            if nfs_base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nfs_base"));
                            }
                            nfs_base__ = Some(map.next_value()?);
                        }
                        GeneratedField::Notify => {
                            if notify__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notify"));
                            }
                            notify__ = Some(map.next_value()?);
                        }
                        GeneratedField::NowDate => {
                            if now_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("now_date"));
                            }
                            now_date__ = Some(map.next_value()?);
                        }
                        GeneratedField::OutputDir => {
                            if output_dir__.is_some() {
                                return Err(serde::de::Error::duplicate_field("output_dir"));
                            }
                            output_dir__ = Some(map.next_value()?);
                        }
                        GeneratedField::OutputDirTicket => {
                            if output_dir_ticket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("output_dir_ticket"));
                            }
                            output_dir_ticket__ = Some(map.next_value()?);
                        }
                        GeneratedField::OutputTicketFile => {
                            if output_ticket_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("output_ticket_file"));
                            }
                            output_ticket_file__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestType => {
                            if request_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request_type"));
                            }
                            request_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::RunOnNfs => {
                            if run_on_nfs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("run-on-nfs"));
                            }
                            run_on_nfs__ = Some(map.next_value()?);
                        }
                        GeneratedField::SkipParentMetadata => {
                            if skip_parent_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skip-parent-meta"));
                            }
                            skip_parent_metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::Steps => {
                            if steps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("steps"));
                            }
                            steps__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubmissionDate => {
                            if submission_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("submission_date"));
                            }
                            submission_date__ = Some(map.next_value()?);
                        }
                        GeneratedField::Submitter => {
                            if submitter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            submitter__ = Some(map.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_id"));
                            }
                            user_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::UserGroups => {
                            if user_groups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_groups"));
                            }
                            user_groups__ = Some(map.next_value()?);
                        }
                        GeneratedField::UserHome => {
                            if user_home__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_home"));
                            }
                            user_home__ = Some(map.next_value()?);
                        }
                        GeneratedField::WikiUrl => {
                            if wiki_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wiki_url"));
                            }
                            wiki_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::ConfigFile => {
                            if config_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config_file"));
                            }
                            config_file__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Job {
                    app_description: app_description__.unwrap_or_default(),
                    app_id: app_id__.unwrap_or_default(),
                    app_name: app_name__.unwrap_or_default(),
                    archive_logs: archive_logs__.unwrap_or_default(),
                    batch_id: batch_id__.unwrap_or_default(),
                    condor_id: condor_id__.unwrap_or_default(),
                    condor_log_path: condor_log_path__.unwrap_or_default(),
                    create_output_subdir: create_output_subdir__.unwrap_or_default(),
                    date_submitted: date_submitted__,
                    date_started: date_started__,
                    date_completed: date_completed__,
                    description: description__.unwrap_or_default(),
                    email: email__.unwrap_or_default(),
                    extra: extra__,
                    execution_target: execution_target__.unwrap_or_default(),
                    exit_code: exit_code__.unwrap_or_default(),
                    failure_count: failure_count__.unwrap_or_default(),
                    failure_threshold: failure_threshold__.unwrap_or_default(),
                    file_metadata: file_metadata__.unwrap_or_default(),
                    filter_files: filter_files__.unwrap_or_default(),
                    group: group__.unwrap_or_default(),
                    input_path_list_file: input_path_list_file__.unwrap_or_default(),
                    input_tickets_file: input_tickets_file__.unwrap_or_default(),
                    invocation_id: invocation_id__.unwrap_or_default(),
                    irods_base: irods_base__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    nfs_base: nfs_base__.unwrap_or_default(),
                    notify: notify__.unwrap_or_default(),
                    now_date: now_date__.unwrap_or_default(),
                    output_dir: output_dir__.unwrap_or_default(),
                    output_dir_ticket: output_dir_ticket__.unwrap_or_default(),
                    output_ticket_file: output_ticket_file__.unwrap_or_default(),
                    request_type: request_type__.unwrap_or_default(),
                    run_on_nfs: run_on_nfs__.unwrap_or_default(),
                    skip_parent_metadata: skip_parent_metadata__.unwrap_or_default(),
                    steps: steps__.unwrap_or_default(),
                    submission_date: submission_date__.unwrap_or_default(),
                    submitter: submitter__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    user_groups: user_groups__.unwrap_or_default(),
                    user_home: user_home__.unwrap_or_default(),
                    wiki_url: wiki_url__.unwrap_or_default(),
                    config_file: config_file__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.Job", FIELDS, GeneratedVisitor)
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
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(LookupType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(LookupType::from_i32)
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
impl serde::Serialize for NoParams {
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
        let mut struct_ser = serializer.serialize_struct("debuff.NoParams", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NoParams {
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
            type Value = NoParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.NoParams")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<NoParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                    }
                }
                Ok(NoParams {
                    header: header__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.NoParams", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.NoParamsRequest", len)?;
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
                formatter.write_str("struct debuff.NoParamsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<NoParamsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                    }
                }
                Ok(NoParamsRequest {
                    header: header__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.NoParamsRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.Overage", len)?;
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
                formatter.write_str("struct debuff.Overage")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Overage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_name__ = None;
                let mut quota__ = None;
                let mut usage__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResourceName => {
                            if resource_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_name"));
                            }
                            resource_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Quota => {
                            if quota__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota"));
                            }
                            quota__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Usage => {
                            if usage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usage"));
                            }
                            usage__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
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
        deserializer.deserialize_struct("debuff.Overage", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.OverageList", len)?;
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
                formatter.write_str("struct debuff.OverageList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OverageList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut overages__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::Overages => {
                            if overages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overages"));
                            }
                            overages__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.OverageList", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.OverageResponse", len)?;
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
                formatter.write_str("struct debuff.OverageResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OverageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut overage__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::Overage => {
                            if overage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overage"));
                            }
                            overage__ = map.next_value()?;
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
        deserializer.deserialize_struct("debuff.OverageResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.Plan", len)?;
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
                formatter.write_str("struct debuff.Plan")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Plan, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut plan_quota_defaults__ = None;
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
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::PlanQuotaDefaults => {
                            if plan_quota_defaults__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plan_quota_defaults"));
                            }
                            plan_quota_defaults__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.Plan", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.PlanList", len)?;
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
                formatter.write_str("struct debuff.PlanList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PlanList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut plans__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::Plans => {
                            if plans__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plans"));
                            }
                            plans__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.PlanList", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.PlanRequest", len)?;
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
                formatter.write_str("struct debuff.PlanRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PlanRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut plan_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::PlanId => {
                            if plan_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plan_id"));
                            }
                            plan_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PlanRequest {
                    header: header__,
                    plan_id: plan_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.PlanRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.PlanResponse", len)?;
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
                formatter.write_str("struct debuff.PlanResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PlanResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut plan__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::Plan => {
                            if plan__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plan"));
                            }
                            plan__ = map.next_value()?;
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
        deserializer.deserialize_struct("debuff.PlanResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.QMSUser", len)?;
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
                formatter.write_str("struct debuff.QMSUser")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QmsUser, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut username__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map.next_value()?);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QmsUser {
                    uuid: uuid__.unwrap_or_default(),
                    username: username__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.QMSUser", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.QMSUserList", len)?;
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
                formatter.write_str("struct debuff.QMSUserList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QmsUserList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut users__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::Users => {
                            if users__.is_some() {
                                return Err(serde::de::Error::duplicate_field("users"));
                            }
                            users__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.QMSUserList", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.QMSUserResponse", len)?;
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
                formatter.write_str("struct debuff.QMSUserResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QmsUserResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut user__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map.next_value()?;
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
        deserializer.deserialize_struct("debuff.QMSUserResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.Quota", len)?;
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
                formatter.write_str("struct debuff.Quota")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Quota, V::Error>
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
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map.next_value()?);
                        }
                        GeneratedField::Quota => {
                            if quota__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota"));
                            }
                            quota__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_type"));
                            }
                            resource_type__ = map.next_value()?;
                        }
                        GeneratedField::CreatedBy => {
                            if created_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("created_by"));
                            }
                            created_by__ = Some(map.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("created_at"));
                            }
                            created_at__ = map.next_value()?;
                        }
                        GeneratedField::LastModifiedBy => {
                            if last_modified_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("last_modified_by"));
                            }
                            last_modified_by__ = Some(map.next_value()?);
                        }
                        GeneratedField::LastModifiedAt => {
                            if last_modified_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("last_modified_at"));
                            }
                            last_modified_at__ = map.next_value()?;
                        }
                        GeneratedField::SubscriptionId => {
                            if subscription_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscription_id"));
                            }
                            subscription_id__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.Quota", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.QuotaDefault", len)?;
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
                formatter.write_str("struct debuff.QuotaDefault")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QuotaDefault, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut quota_value__ = None;
                let mut resource_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map.next_value()?);
                        }
                        GeneratedField::QuotaValue => {
                            if quota_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota_value"));
                            }
                            quota_value__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_type"));
                            }
                            resource_type__ = map.next_value()?;
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
        deserializer.deserialize_struct("debuff.QuotaDefault", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.QuotaDefaultList", len)?;
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
                formatter.write_str("struct debuff.QuotaDefaultList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QuotaDefaultList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut quota_defaults__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::QuotaDefaults => {
                            if quota_defaults__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota_defaults"));
                            }
                            quota_defaults__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.QuotaDefaultList", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.QuotaDefaultResponse", len)?;
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
                formatter.write_str("struct debuff.QuotaDefaultResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QuotaDefaultResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut quota_default__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::QuotaDefault => {
                            if quota_default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota_default"));
                            }
                            quota_default__ = map.next_value()?;
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
        deserializer.deserialize_struct("debuff.QuotaDefaultResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.QuotaList", len)?;
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
                formatter.write_str("struct debuff.QuotaList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QuotaList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut quotas__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::Quotas => {
                            if quotas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotas"));
                            }
                            quotas__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.QuotaList", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.QuotaResponse", len)?;
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
                formatter.write_str("struct debuff.QuotaResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QuotaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut quota__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::Quota => {
                            if quota__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota"));
                            }
                            quota__ = map.next_value()?;
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
        deserializer.deserialize_struct("debuff.QuotaResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.RequestByUserID", len)?;
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
                formatter.write_str("struct debuff.RequestByUserID")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RequestByUserId, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut user_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_id"));
                            }
                            user_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RequestByUserId {
                    header: header__,
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.RequestByUserID", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.RequestByUsername", len)?;
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
                formatter.write_str("struct debuff.RequestByUsername")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RequestByUsername, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut username__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RequestByUsername {
                    header: header__,
                    username: username__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.RequestByUsername", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.ResourceType", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            Name,
            Unit,
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
                formatter.write_str("struct debuff.ResourceType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ResourceType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut name__ = None;
                let mut unit__ = None;
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
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ResourceType {
                    uuid: uuid__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.ResourceType", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.ResourceTypeList", len)?;
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
                formatter.write_str("struct debuff.ResourceTypeList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ResourceTypeList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut resource_types__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::ResourceTypes => {
                            if resource_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_types"));
                            }
                            resource_types__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.ResourceTypeList", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.ResourceTypeResponse", len)?;
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
                formatter.write_str("struct debuff.ResourceTypeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ResourceTypeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut resource_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_type"));
                            }
                            resource_type__ = map.next_value()?;
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
        deserializer.deserialize_struct("debuff.ResourceTypeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceError {
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
        if self.error_code != 0 {
            len += 1;
        }
        if self.status_code != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.ServiceError", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if self.error_code != 0 {
            let v = ErrorCode::from_i32(self.error_code)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.error_code)))?;
            struct_ser.serialize_field("error_code", &v)?;
        }
        if self.status_code != 0 {
            struct_ser.serialize_field("status_code", &self.status_code)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServiceError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "error_code",
            "status_code",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            ErrorCode,
            StatusCode,
            Message,
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
                            "error_code" => Ok(GeneratedField::ErrorCode),
                            "status_code" => Ok(GeneratedField::StatusCode),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.ServiceError")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServiceError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error_code__ = None;
                let mut status_code__ = None;
                let mut message__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::ErrorCode => {
                            if error_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error_code"));
                            }
                            error_code__ = Some(map.next_value::<ErrorCode>()? as i32);
                        }
                        GeneratedField::StatusCode => {
                            if status_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status_code"));
                            }
                            status_code__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ServiceError {
                    header: header__,
                    error_code: error_code__.unwrap_or_default(),
                    status_code: status_code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.ServiceError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Step {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.component.is_some() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.stdin_path.is_empty() {
            len += 1;
        }
        if !self.stdout_path.is_empty() {
            len += 1;
        }
        if !self.stderr_path.is_empty() {
            len += 1;
        }
        if !self.log_file.is_empty() {
            len += 1;
        }
        if !self.environment.is_empty() {
            len += 1;
        }
        if !self.input.is_empty() {
            len += 1;
        }
        if !self.output.is_empty() {
            len += 1;
        }
        if self.header.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.Step", len)?;
        if let Some(v) = self.component.as_ref() {
            struct_ser.serialize_field("component", v)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.stdin_path.is_empty() {
            struct_ser.serialize_field("stdin", &self.stdin_path)?;
        }
        if !self.stdout_path.is_empty() {
            struct_ser.serialize_field("stdout", &self.stdout_path)?;
        }
        if !self.stderr_path.is_empty() {
            struct_ser.serialize_field("stderr", &self.stderr_path)?;
        }
        if !self.log_file.is_empty() {
            struct_ser.serialize_field("log-file", &self.log_file)?;
        }
        if !self.environment.is_empty() {
            struct_ser.serialize_field("environment", &self.environment)?;
        }
        if !self.input.is_empty() {
            struct_ser.serialize_field("input", &self.input)?;
        }
        if !self.output.is_empty() {
            struct_ser.serialize_field("output", &self.output)?;
        }
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Step {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "component",
            "config",
            "type",
            "stdin_path",
            "stdin",
            "stdout_path",
            "stdout",
            "stderr_path",
            "stderr",
            "log_file",
            "log-file",
            "environment",
            "input",
            "output",
            "header",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Component,
            Config,
            Type,
            StdinPath,
            StdoutPath,
            StderrPath,
            LogFile,
            Environment,
            Input,
            Output,
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
                            "component" => Ok(GeneratedField::Component),
                            "config" => Ok(GeneratedField::Config),
                            "type" => Ok(GeneratedField::Type),
                            "stdin" | "stdin_path" => Ok(GeneratedField::StdinPath),
                            "stdout" | "stdout_path" => Ok(GeneratedField::StdoutPath),
                            "stderr" | "stderr_path" => Ok(GeneratedField::StderrPath),
                            "log-file" | "log_file" => Ok(GeneratedField::LogFile),
                            "environment" => Ok(GeneratedField::Environment),
                            "input" => Ok(GeneratedField::Input),
                            "output" => Ok(GeneratedField::Output),
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
            type Value = Step;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.Step")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Step, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut component__ = None;
                let mut config__ = None;
                let mut r#type__ = None;
                let mut stdin_path__ = None;
                let mut stdout_path__ = None;
                let mut stderr_path__ = None;
                let mut log_file__ = None;
                let mut environment__ = None;
                let mut input__ = None;
                let mut output__ = None;
                let mut header__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Component => {
                            if component__.is_some() {
                                return Err(serde::de::Error::duplicate_field("component"));
                            }
                            component__ = map.next_value()?;
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map.next_value()?;
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value()?);
                        }
                        GeneratedField::StdinPath => {
                            if stdin_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stdin"));
                            }
                            stdin_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::StdoutPath => {
                            if stdout_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stdout"));
                            }
                            stdout_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::StderrPath => {
                            if stderr_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stderr"));
                            }
                            stderr_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::LogFile => {
                            if log_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("log-file"));
                            }
                            log_file__ = Some(map.next_value()?);
                        }
                        GeneratedField::Environment => {
                            if environment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("environment"));
                            }
                            environment__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Input => {
                            if input__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input"));
                            }
                            input__ = Some(map.next_value()?);
                        }
                        GeneratedField::Output => {
                            if output__.is_some() {
                                return Err(serde::de::Error::duplicate_field("output"));
                            }
                            output__ = Some(map.next_value()?);
                        }
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                    }
                }
                Ok(Step {
                    component: component__,
                    config: config__,
                    r#type: r#type__.unwrap_or_default(),
                    stdin_path: stdin_path__.unwrap_or_default(),
                    stdout_path: stdout_path__.unwrap_or_default(),
                    stderr_path: stderr_path__.unwrap_or_default(),
                    log_file: log_file__.unwrap_or_default(),
                    environment: environment__.unwrap_or_default(),
                    input: input__.unwrap_or_default(),
                    output: output__.unwrap_or_default(),
                    header: header__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.Step", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for step::Component {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.container.is_some() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.location.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.time_limit != 0 {
            len += 1;
        }
        if self.restricted {
            len += 1;
        }
        if self.is_interactive {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.Step.Component", len)?;
        if let Some(v) = self.container.as_ref() {
            struct_ser.serialize_field("container", v)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.location.is_empty() {
            struct_ser.serialize_field("location", &self.location)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.time_limit != 0 {
            struct_ser.serialize_field("time_limit_seconds", &self.time_limit)?;
        }
        if self.restricted {
            struct_ser.serialize_field("restricted", &self.restricted)?;
        }
        if self.is_interactive {
            struct_ser.serialize_field("interactive", &self.is_interactive)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for step::Component {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "container",
            "type",
            "name",
            "location",
            "description",
            "time_limit",
            "time_limit_seconds",
            "restricted",
            "is_interactive",
            "interactive",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Container,
            Type,
            Name,
            Location,
            Description,
            TimeLimit,
            Restricted,
            IsInteractive,
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
                            "container" => Ok(GeneratedField::Container),
                            "type" => Ok(GeneratedField::Type),
                            "name" => Ok(GeneratedField::Name),
                            "location" => Ok(GeneratedField::Location),
                            "description" => Ok(GeneratedField::Description),
                            "time_limit_seconds" | "time_limit" => Ok(GeneratedField::TimeLimit),
                            "restricted" => Ok(GeneratedField::Restricted),
                            "interactive" | "is_interactive" => Ok(GeneratedField::IsInteractive),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = step::Component;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.Step.Component")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<step::Component, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut container__ = None;
                let mut r#type__ = None;
                let mut name__ = None;
                let mut location__ = None;
                let mut description__ = None;
                let mut time_limit__ = None;
                let mut restricted__ = None;
                let mut is_interactive__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Container => {
                            if container__.is_some() {
                                return Err(serde::de::Error::duplicate_field("container"));
                            }
                            container__ = map.next_value()?;
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Location => {
                            if location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            location__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::TimeLimit => {
                            if time_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time_limit_seconds"));
                            }
                            time_limit__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Restricted => {
                            if restricted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restricted"));
                            }
                            restricted__ = Some(map.next_value()?);
                        }
                        GeneratedField::IsInteractive => {
                            if is_interactive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interactive"));
                            }
                            is_interactive__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(step::Component {
                    container: container__,
                    r#type: r#type__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    location: location__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    time_limit: time_limit__.unwrap_or_default(),
                    restricted: restricted__.unwrap_or_default(),
                    is_interactive: is_interactive__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.Step.Component", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for step::Config {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.params.is_empty() {
            len += 1;
        }
        if !self.inputs.is_empty() {
            len += 1;
        }
        if !self.outputs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.Step.Config", len)?;
        if !self.params.is_empty() {
            struct_ser.serialize_field("params", &self.params)?;
        }
        if !self.inputs.is_empty() {
            struct_ser.serialize_field("input", &self.inputs)?;
        }
        if !self.outputs.is_empty() {
            struct_ser.serialize_field("output", &self.outputs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for step::Config {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "params",
            "inputs",
            "input",
            "outputs",
            "output",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            Inputs,
            Outputs,
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
                            "params" => Ok(GeneratedField::Params),
                            "input" | "inputs" => Ok(GeneratedField::Inputs),
                            "output" | "outputs" => Ok(GeneratedField::Outputs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = step::Config;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.Step.Config")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<step::Config, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut inputs__ = None;
                let mut outputs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = Some(map.next_value()?);
                        }
                        GeneratedField::Inputs => {
                            if inputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input"));
                            }
                            inputs__ = Some(map.next_value()?);
                        }
                        GeneratedField::Outputs => {
                            if outputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("output"));
                            }
                            outputs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(step::Config {
                    params: params__.unwrap_or_default(),
                    inputs: inputs__.unwrap_or_default(),
                    outputs: outputs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.Step.Config", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for step::Input {
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
        if !self.ticket.is_empty() {
            len += 1;
        }
        if !self.multiplicity.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.property.is_empty() {
            len += 1;
        }
        if self.retain {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.Step.Input", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.ticket.is_empty() {
            struct_ser.serialize_field("ticket", &self.ticket)?;
        }
        if !self.multiplicity.is_empty() {
            struct_ser.serialize_field("multiplicity", &self.multiplicity)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.property.is_empty() {
            struct_ser.serialize_field("property", &self.property)?;
        }
        if self.retain {
            struct_ser.serialize_field("retain", &self.retain)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for step::Input {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "ticket",
            "multiplicity",
            "name",
            "property",
            "retain",
            "type",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Ticket,
            Multiplicity,
            Name,
            Property,
            Retain,
            Type,
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
                            "id" => Ok(GeneratedField::Id),
                            "ticket" => Ok(GeneratedField::Ticket),
                            "multiplicity" => Ok(GeneratedField::Multiplicity),
                            "name" => Ok(GeneratedField::Name),
                            "property" => Ok(GeneratedField::Property),
                            "retain" => Ok(GeneratedField::Retain),
                            "type" => Ok(GeneratedField::Type),
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
            type Value = step::Input;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.Step.Input")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<step::Input, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut ticket__ = None;
                let mut multiplicity__ = None;
                let mut name__ = None;
                let mut property__ = None;
                let mut retain__ = None;
                let mut r#type__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Ticket => {
                            if ticket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ticket"));
                            }
                            ticket__ = Some(map.next_value()?);
                        }
                        GeneratedField::Multiplicity => {
                            if multiplicity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multiplicity"));
                            }
                            multiplicity__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Property => {
                            if property__.is_some() {
                                return Err(serde::de::Error::duplicate_field("property"));
                            }
                            property__ = Some(map.next_value()?);
                        }
                        GeneratedField::Retain => {
                            if retain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retain"));
                            }
                            retain__ = Some(map.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(step::Input {
                    id: id__.unwrap_or_default(),
                    ticket: ticket__.unwrap_or_default(),
                    multiplicity: multiplicity__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    property: property__.unwrap_or_default(),
                    retain: retain__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.Step.Input", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for step::Output {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.multiplicity.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.property.is_empty() {
            len += 1;
        }
        if !self.qual_id.is_empty() {
            len += 1;
        }
        if self.retain {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.Step.Output", len)?;
        if !self.multiplicity.is_empty() {
            struct_ser.serialize_field("multiplicity", &self.multiplicity)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.property.is_empty() {
            struct_ser.serialize_field("property", &self.property)?;
        }
        if !self.qual_id.is_empty() {
            struct_ser.serialize_field("qual-id", &self.qual_id)?;
        }
        if self.retain {
            struct_ser.serialize_field("retain", &self.retain)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for step::Output {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "multiplicity",
            "name",
            "property",
            "qual_id",
            "qual-id",
            "retain",
            "type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Multiplicity,
            Name,
            Property,
            QualId,
            Retain,
            Type,
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
                            "multiplicity" => Ok(GeneratedField::Multiplicity),
                            "name" => Ok(GeneratedField::Name),
                            "property" => Ok(GeneratedField::Property),
                            "qual-id" | "qual_id" => Ok(GeneratedField::QualId),
                            "retain" => Ok(GeneratedField::Retain),
                            "type" => Ok(GeneratedField::Type),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = step::Output;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.Step.Output")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<step::Output, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut multiplicity__ = None;
                let mut name__ = None;
                let mut property__ = None;
                let mut qual_id__ = None;
                let mut retain__ = None;
                let mut r#type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Multiplicity => {
                            if multiplicity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multiplicity"));
                            }
                            multiplicity__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Property => {
                            if property__.is_some() {
                                return Err(serde::de::Error::duplicate_field("property"));
                            }
                            property__ = Some(map.next_value()?);
                        }
                        GeneratedField::QualId => {
                            if qual_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("qual-id"));
                            }
                            qual_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Retain => {
                            if retain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retain"));
                            }
                            retain__ = Some(map.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(step::Output {
                    multiplicity: multiplicity__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    property: property__.unwrap_or_default(),
                    qual_id: qual_id__.unwrap_or_default(),
                    retain: retain__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.Step.Output", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for step::Param {
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
        if !self.value.is_empty() {
            len += 1;
        }
        if self.order != 0 {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.Step.Param", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if self.order != 0 {
            struct_ser.serialize_field("order", &self.order)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for step::Param {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "value",
            "order",
            "type",
            "path",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Value,
            Order,
            Type,
            Path,
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
                            "value" => Ok(GeneratedField::Value),
                            "order" => Ok(GeneratedField::Order),
                            "type" => Ok(GeneratedField::Type),
                            "path" => Ok(GeneratedField::Path),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = step::Param;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.Step.Param")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<step::Param, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut value__ = None;
                let mut order__ = None;
                let mut r#type__ = None;
                let mut path__ = None;
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
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map.next_value()?);
                        }
                        GeneratedField::Order => {
                            if order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            order__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(step::Param {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    order: order__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.Step.Param", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.Subscription", len)?;
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
                formatter.write_str("struct debuff.Subscription")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Subscription, V::Error>
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
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map.next_value()?);
                        }
                        GeneratedField::EffectiveStartDate => {
                            if effective_start_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("effective_start_date"));
                            }
                            effective_start_date__ = map.next_value()?;
                        }
                        GeneratedField::EffectiveEndDate => {
                            if effective_end_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("effective_end_date"));
                            }
                            effective_end_date__ = map.next_value()?;
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map.next_value()?;
                        }
                        GeneratedField::Plan => {
                            if plan__.is_some() {
                                return Err(serde::de::Error::duplicate_field("plan"));
                            }
                            plan__ = map.next_value()?;
                        }
                        GeneratedField::Quotas => {
                            if quotas__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotas"));
                            }
                            quotas__ = Some(map.next_value()?);
                        }
                        GeneratedField::Usages => {
                            if usages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usages"));
                            }
                            usages__ = Some(map.next_value()?);
                        }
                        GeneratedField::Paid => {
                            if paid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paid"));
                            }
                            paid__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.Subscription", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.SubscriptionAddon", len)?;
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
                formatter.write_str("struct debuff.SubscriptionAddon")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SubscriptionAddon, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut addon__ = None;
                let mut subscription__ = None;
                let mut amount__ = None;
                let mut paid__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map.next_value()?);
                        }
                        GeneratedField::Addon => {
                            if addon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addon"));
                            }
                            addon__ = map.next_value()?;
                        }
                        GeneratedField::Subscription => {
                            if subscription__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscription"));
                            }
                            subscription__ = map.next_value()?;
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Paid => {
                            if paid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paid"));
                            }
                            paid__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.SubscriptionAddon", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.SubscriptionAddonListResponse", len)?;
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
                formatter.write_str("struct debuff.SubscriptionAddonListResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SubscriptionAddonListResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut subscription_addons__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::SubscriptionAddons => {
                            if subscription_addons__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscription_addons"));
                            }
                            subscription_addons__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.SubscriptionAddonListResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.SubscriptionAddonResponse", len)?;
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
                formatter.write_str("struct debuff.SubscriptionAddonResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SubscriptionAddonResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut subscription_addon__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::SubscriptionAddon => {
                            if subscription_addon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscription_addon"));
                            }
                            subscription_addon__ = map.next_value()?;
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
        deserializer.deserialize_struct("debuff.SubscriptionAddonResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.SubscriptionList", len)?;
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
                formatter.write_str("struct debuff.SubscriptionList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SubscriptionList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut subscriptions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::Subscriptions => {
                            if subscriptions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscriptions"));
                            }
                            subscriptions__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.SubscriptionList", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.SubscriptionResponse", len)?;
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
                formatter.write_str("struct debuff.SubscriptionResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SubscriptionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut subscription__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::Subscription => {
                            if subscription__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscription"));
                            }
                            subscription__ = map.next_value()?;
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
        deserializer.deserialize_struct("debuff.SubscriptionResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.Update", len)?;
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
                formatter.write_str("struct debuff.Update")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Update, V::Error>
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
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map.next_value()?);
                        }
                        GeneratedField::ValueType => {
                            if value_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value_type"));
                            }
                            value_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EffectiveDate => {
                            if effective_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("effective_date"));
                            }
                            effective_date__ = map.next_value()?;
                        }
                        GeneratedField::Operation => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation__ = map.next_value()?;
                        }
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_type"));
                            }
                            resource_type__ = map.next_value()?;
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map.next_value()?;
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
        deserializer.deserialize_struct("debuff.Update", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.UpdateAddonRequest", len)?;
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
                formatter.write_str("struct debuff.UpdateAddonRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateAddonRequest, V::Error>
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
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Addon => {
                            if addon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addon"));
                            }
                            addon__ = map.next_value()?;
                        }
                        GeneratedField::UpdateName => {
                            if update_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateName"));
                            }
                            update_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpdateDescription => {
                            if update_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateDescription"));
                            }
                            update_description__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpdateResourceType => {
                            if update_resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateResourceType"));
                            }
                            update_resource_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpdateDefaultAmount => {
                            if update_default_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateDefaultAmount"));
                            }
                            update_default_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpdateDefaultPaid => {
                            if update_default_paid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateDefaultPaid"));
                            }
                            update_default_paid__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.UpdateAddonRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.UpdateListRequest", len)?;
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
                formatter.write_str("struct debuff.UpdateListRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateListRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut user__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpdateListRequest {
                    header: header__,
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.UpdateListRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.UpdateListResponse", len)?;
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
                formatter.write_str("struct debuff.UpdateListResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateListResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut updates__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::Updates => {
                            if updates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updates"));
                            }
                            updates__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.UpdateListResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.UpdateOperation", len)?;
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
                formatter.write_str("struct debuff.UpdateOperation")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateOperation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut name__ = None;
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
                    }
                }
                Ok(UpdateOperation {
                    uuid: uuid__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.UpdateOperation", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.UpdateSubscriptionAddonRequest", len)?;
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
                formatter.write_str("struct debuff.UpdateSubscriptionAddonRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateSubscriptionAddonRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut subscription_addon__ = None;
                let mut update_addon_id__ = None;
                let mut update_subscription_id__ = None;
                let mut update_amount__ = None;
                let mut update_paid__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::SubscriptionAddon => {
                            if subscription_addon__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscription_addon"));
                            }
                            subscription_addon__ = map.next_value()?;
                        }
                        GeneratedField::UpdateAddonId => {
                            if update_addon_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update_addon_id"));
                            }
                            update_addon_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpdateSubscriptionId => {
                            if update_subscription_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update_subscription_id"));
                            }
                            update_subscription_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpdateAmount => {
                            if update_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update_amount"));
                            }
                            update_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpdatePaid => {
                            if update_paid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update_paid"));
                            }
                            update_paid__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.UpdateSubscriptionAddonRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.Usage", len)?;
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
                formatter.write_str("struct debuff.Usage")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Usage, V::Error>
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
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map.next_value()?);
                        }
                        GeneratedField::Usage => {
                            if usage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usage"));
                            }
                            usage__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SubscriptionId => {
                            if subscription_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscription_id"));
                            }
                            subscription_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_type"));
                            }
                            resource_type__ = map.next_value()?;
                        }
                        GeneratedField::CreatedBy => {
                            if created_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("created_by"));
                            }
                            created_by__ = Some(map.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("created_at"));
                            }
                            created_at__ = map.next_value()?;
                        }
                        GeneratedField::LastModifiedBy => {
                            if last_modified_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("last_modified_by"));
                            }
                            last_modified_by__ = Some(map.next_value()?);
                        }
                        GeneratedField::LastModifiedAt => {
                            if last_modified_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("last_modified_at"));
                            }
                            last_modified_at__ = map.next_value()?;
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
        deserializer.deserialize_struct("debuff.Usage", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.UsageList", len)?;
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
                formatter.write_str("struct debuff.UsageList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UsageList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut usages__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::Usages => {
                            if usages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usages"));
                            }
                            usages__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.UsageList", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.UsageResponse", len)?;
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
                formatter.write_str("struct debuff.UsageResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UsageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut error__ = None;
                let mut usage__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                        GeneratedField::Usage => {
                            if usage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usage"));
                            }
                            usage__ = map.next_value()?;
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
        deserializer.deserialize_struct("debuff.UsageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for User {
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
        if self.preferences.is_some() {
            len += 1;
        }
        if !self.logins.is_empty() {
            len += 1;
        }
        if self.login_count != 0 {
            len += 1;
        }
        if self.saved_searches.is_some() {
            len += 1;
        }
        if self.header.is_some() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.User", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if let Some(v) = self.preferences.as_ref() {
            struct_ser.serialize_field("preferences", v)?;
        }
        if !self.logins.is_empty() {
            struct_ser.serialize_field("logins", &self.logins)?;
        }
        if self.login_count != 0 {
            struct_ser.serialize_field("loginCount", &self.login_count)?;
        }
        if let Some(v) = self.saved_searches.as_ref() {
            struct_ser.serialize_field("savedSearches", v)?;
        }
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for User {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uuid",
            "username",
            "preferences",
            "logins",
            "login_count",
            "loginCount",
            "saved_searches",
            "savedSearches",
            "header",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            Username,
            Preferences,
            Logins,
            LoginCount,
            SavedSearches,
            Header,
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
                            "uuid" => Ok(GeneratedField::Uuid),
                            "username" => Ok(GeneratedField::Username),
                            "preferences" => Ok(GeneratedField::Preferences),
                            "logins" => Ok(GeneratedField::Logins),
                            "loginCount" | "login_count" => Ok(GeneratedField::LoginCount),
                            "savedSearches" | "saved_searches" => Ok(GeneratedField::SavedSearches),
                            "header" => Ok(GeneratedField::Header),
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
            type Value = User;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.User")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<User, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut username__ = None;
                let mut preferences__ = None;
                let mut logins__ = None;
                let mut login_count__ = None;
                let mut saved_searches__ = None;
                let mut header__ = None;
                let mut error__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map.next_value()?);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map.next_value()?);
                        }
                        GeneratedField::Preferences => {
                            if preferences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferences"));
                            }
                            preferences__ = map.next_value()?;
                        }
                        GeneratedField::Logins => {
                            if logins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logins"));
                            }
                            logins__ = Some(map.next_value()?);
                        }
                        GeneratedField::LoginCount => {
                            if login_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginCount"));
                            }
                            login_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SavedSearches => {
                            if saved_searches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("savedSearches"));
                            }
                            saved_searches__ = map.next_value()?;
                        }
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map.next_value()?;
                        }
                    }
                }
                Ok(User {
                    uuid: uuid__.unwrap_or_default(),
                    username: username__.unwrap_or_default(),
                    preferences: preferences__,
                    logins: logins__.unwrap_or_default(),
                    login_count: login_count__.unwrap_or_default(),
                    saved_searches: saved_searches__,
                    header: header__,
                    error: error__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.User", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for user::Login {
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
        if !self.ip_address.is_empty() {
            len += 1;
        }
        if !self.user_agent.is_empty() {
            len += 1;
        }
        if self.login_time.is_some() {
            len += 1;
        }
        if self.logout_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.User.Login", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.ip_address.is_empty() {
            struct_ser.serialize_field("ipAddress", &self.ip_address)?;
        }
        if !self.user_agent.is_empty() {
            struct_ser.serialize_field("userAgent", &self.user_agent)?;
        }
        if let Some(v) = self.login_time.as_ref() {
            struct_ser.serialize_field("loginTime", v)?;
        }
        if let Some(v) = self.logout_time.as_ref() {
            struct_ser.serialize_field("logoutTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for user::Login {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uuid",
            "ip_address",
            "ipAddress",
            "user_agent",
            "userAgent",
            "login_time",
            "loginTime",
            "logout_time",
            "logoutTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            IpAddress,
            UserAgent,
            LoginTime,
            LogoutTime,
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
                            "ipAddress" | "ip_address" => Ok(GeneratedField::IpAddress),
                            "userAgent" | "user_agent" => Ok(GeneratedField::UserAgent),
                            "loginTime" | "login_time" => Ok(GeneratedField::LoginTime),
                            "logoutTime" | "logout_time" => Ok(GeneratedField::LogoutTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = user::Login;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.User.Login")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<user::Login, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut ip_address__ = None;
                let mut user_agent__ = None;
                let mut login_time__ = None;
                let mut logout_time__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map.next_value()?);
                        }
                        GeneratedField::IpAddress => {
                            if ip_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipAddress"));
                            }
                            ip_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::UserAgent => {
                            if user_agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAgent"));
                            }
                            user_agent__ = Some(map.next_value()?);
                        }
                        GeneratedField::LoginTime => {
                            if login_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginTime"));
                            }
                            login_time__ = map.next_value()?;
                        }
                        GeneratedField::LogoutTime => {
                            if logout_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logoutTime"));
                            }
                            logout_time__ = map.next_value()?;
                        }
                    }
                }
                Ok(user::Login {
                    uuid: uuid__.unwrap_or_default(),
                    ip_address: ip_address__.unwrap_or_default(),
                    user_agent: user_agent__.unwrap_or_default(),
                    login_time: login_time__,
                    logout_time: logout_time__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.User.Login", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for user::Preferences {
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
        if !self.preferences.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.User.Preferences", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.preferences.is_empty() {
            struct_ser.serialize_field("preferences", &self.preferences)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for user::Preferences {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uuid",
            "preferences",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            Preferences,
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
                            "preferences" => Ok(GeneratedField::Preferences),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = user::Preferences;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.User.Preferences")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<user::Preferences, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut preferences__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map.next_value()?);
                        }
                        GeneratedField::Preferences => {
                            if preferences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferences"));
                            }
                            preferences__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(user::Preferences {
                    uuid: uuid__.unwrap_or_default(),
                    preferences: preferences__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.User.Preferences", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for user::SavedSearches {
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
        if !self.saved_searches.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.User.SavedSearches", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.saved_searches.is_empty() {
            struct_ser.serialize_field("savedSearches", &self.saved_searches)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for user::SavedSearches {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uuid",
            "saved_searches",
            "savedSearches",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            SavedSearches,
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
                            "savedSearches" | "saved_searches" => Ok(GeneratedField::SavedSearches),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = user::SavedSearches;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.User.SavedSearches")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<user::SavedSearches, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut saved_searches__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map.next_value()?);
                        }
                        GeneratedField::SavedSearches => {
                            if saved_searches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("savedSearches"));
                            }
                            saved_searches__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(user::SavedSearches {
                    uuid: uuid__.unwrap_or_default(),
                    saved_searches: saved_searches__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("debuff.User.SavedSearches", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserLookupRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.include_logins {
            len += 1;
        }
        if self.include_preferences {
            len += 1;
        }
        if self.include_saved_searches {
            len += 1;
        }
        if self.login_limit != 0 {
            len += 1;
        }
        if self.login_offset != 0 {
            len += 1;
        }
        if self.header.is_some() {
            len += 1;
        }
        if self.lookup_ids.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("debuff.UserLookupRequest", len)?;
        if self.include_logins {
            struct_ser.serialize_field("includeLogins", &self.include_logins)?;
        }
        if self.include_preferences {
            struct_ser.serialize_field("includePreferences", &self.include_preferences)?;
        }
        if self.include_saved_searches {
            struct_ser.serialize_field("includeSavedSearches", &self.include_saved_searches)?;
        }
        if self.login_limit != 0 {
            struct_ser.serialize_field("loginLimit", &self.login_limit)?;
        }
        if self.login_offset != 0 {
            struct_ser.serialize_field("loginOffset", &self.login_offset)?;
        }
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.lookup_ids.as_ref() {
            match v {
                user_lookup_request::LookupIds::Username(v) => {
                    struct_ser.serialize_field("username", v)?;
                }
                user_lookup_request::LookupIds::UserId(v) => {
                    struct_ser.serialize_field("userId", v)?;
                }
                user_lookup_request::LookupIds::AnalysisId(v) => {
                    struct_ser.serialize_field("analysisId", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserLookupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "include_logins",
            "includeLogins",
            "include_preferences",
            "includePreferences",
            "include_saved_searches",
            "includeSavedSearches",
            "login_limit",
            "loginLimit",
            "login_offset",
            "loginOffset",
            "header",
            "username",
            "user_id",
            "userId",
            "analysis_id",
            "analysisId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IncludeLogins,
            IncludePreferences,
            IncludeSavedSearches,
            LoginLimit,
            LoginOffset,
            Header,
            Username,
            UserId,
            AnalysisId,
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
                            "includeLogins" | "include_logins" => Ok(GeneratedField::IncludeLogins),
                            "includePreferences" | "include_preferences" => Ok(GeneratedField::IncludePreferences),
                            "includeSavedSearches" | "include_saved_searches" => Ok(GeneratedField::IncludeSavedSearches),
                            "loginLimit" | "login_limit" => Ok(GeneratedField::LoginLimit),
                            "loginOffset" | "login_offset" => Ok(GeneratedField::LoginOffset),
                            "header" => Ok(GeneratedField::Header),
                            "username" => Ok(GeneratedField::Username),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "analysisId" | "analysis_id" => Ok(GeneratedField::AnalysisId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserLookupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct debuff.UserLookupRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UserLookupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut include_logins__ = None;
                let mut include_preferences__ = None;
                let mut include_saved_searches__ = None;
                let mut login_limit__ = None;
                let mut login_offset__ = None;
                let mut header__ = None;
                let mut lookup_ids__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IncludeLogins => {
                            if include_logins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeLogins"));
                            }
                            include_logins__ = Some(map.next_value()?);
                        }
                        GeneratedField::IncludePreferences => {
                            if include_preferences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includePreferences"));
                            }
                            include_preferences__ = Some(map.next_value()?);
                        }
                        GeneratedField::IncludeSavedSearches => {
                            if include_saved_searches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeSavedSearches"));
                            }
                            include_saved_searches__ = Some(map.next_value()?);
                        }
                        GeneratedField::LoginLimit => {
                            if login_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginLimit"));
                            }
                            login_limit__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LoginOffset => {
                            if login_offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginOffset"));
                            }
                            login_offset__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Username => {
                            if lookup_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            lookup_ids__ = map.next_value::<::std::option::Option<_>>()?.map(user_lookup_request::LookupIds::Username);
                        }
                        GeneratedField::UserId => {
                            if lookup_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            lookup_ids__ = map.next_value::<::std::option::Option<_>>()?.map(user_lookup_request::LookupIds::UserId);
                        }
                        GeneratedField::AnalysisId => {
                            if lookup_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("analysisId"));
                            }
                            lookup_ids__ = map.next_value::<::std::option::Option<_>>()?.map(user_lookup_request::LookupIds::AnalysisId);
                        }
                    }
                }
                Ok(UserLookupRequest {
                    include_logins: include_logins__.unwrap_or_default(),
                    include_preferences: include_preferences__.unwrap_or_default(),
                    include_saved_searches: include_saved_searches__.unwrap_or_default(),
                    login_limit: login_limit__.unwrap_or_default(),
                    login_offset: login_offset__.unwrap_or_default(),
                    header: header__,
                    lookup_ids: lookup_ids__,
                })
            }
        }
        deserializer.deserialize_struct("debuff.UserLookupRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("debuff.UserResourceOveragesRequest", len)?;
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
                formatter.write_str("struct debuff.UserResourceOveragesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UserResourceOveragesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut username__ = None;
                let mut resource_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResourceName => {
                            if resource_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_name"));
                            }
                            resource_name__ = Some(map.next_value()?);
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
        deserializer.deserialize_struct("debuff.UserResourceOveragesRequest", FIELDS, GeneratedVisitor)
    }
}
