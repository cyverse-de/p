// @generated
impl serde::Serialize for AbbreviatedPermission {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.permission_id.is_empty() {
            len += 1;
        }
        if !self.resource_name.is_empty() {
            len += 1;
        }
        if !self.resource_type.is_empty() {
            len += 1;
        }
        if self.permission_level != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("groups.AbbreviatedPermission", len)?;
        if !self.permission_id.is_empty() {
            struct_ser.serialize_field("permission_id", &self.permission_id)?;
        }
        if !self.resource_name.is_empty() {
            struct_ser.serialize_field("resource_name", &self.resource_name)?;
        }
        if !self.resource_type.is_empty() {
            struct_ser.serialize_field("resource_type", &self.resource_type)?;
        }
        if self.permission_level != 0 {
            let v = PermissionLevel::try_from(self.permission_level)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.permission_level)))?;
            struct_ser.serialize_field("permission_level", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AbbreviatedPermission {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "permission_id",
            "resource_name",
            "resource_type",
            "permission_level",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PermissionId,
            ResourceName,
            ResourceType,
            PermissionLevel,
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
                            "permission_id" => Ok(GeneratedField::PermissionId),
                            "resource_name" => Ok(GeneratedField::ResourceName),
                            "resource_type" => Ok(GeneratedField::ResourceType),
                            "permission_level" => Ok(GeneratedField::PermissionLevel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AbbreviatedPermission;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct groups.AbbreviatedPermission")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AbbreviatedPermission, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut permission_id__ = None;
                let mut resource_name__ = None;
                let mut resource_type__ = None;
                let mut permission_level__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PermissionId => {
                            if permission_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permission_id"));
                            }
                            permission_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceName => {
                            if resource_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_name"));
                            }
                            resource_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_type"));
                            }
                            resource_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PermissionLevel => {
                            if permission_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permission_level"));
                            }
                            permission_level__ = Some(map_.next_value::<PermissionLevel>()? as i32);
                        }
                    }
                }
                Ok(AbbreviatedPermission {
                    permission_id: permission_id__.unwrap_or_default(),
                    resource_name: resource_name__.unwrap_or_default(),
                    resource_type: resource_type__.unwrap_or_default(),
                    permission_level: permission_level__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("groups.AbbreviatedPermission", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AbbreviatedPermissionList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.permissions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("groups.AbbreviatedPermissionList", len)?;
        if !self.permissions.is_empty() {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AbbreviatedPermissionList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "permissions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Permissions,
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
                            "permissions" => Ok(GeneratedField::Permissions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AbbreviatedPermissionList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct groups.AbbreviatedPermissionList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AbbreviatedPermissionList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut permissions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AbbreviatedPermissionList {
                    permissions: permissions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("groups.AbbreviatedPermissionList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GroupInfo {
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
        let mut struct_ser = serializer.serialize_struct("groups.GroupInfo", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GroupInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            "id" => Ok(GeneratedField::Id),
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
            type Value = GroupInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct groups.GroupInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GroupInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GroupInfo {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("groups.GroupInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Permission {
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
        if self.subject.is_some() {
            len += 1;
        }
        if self.resource.is_some() {
            len += 1;
        }
        if self.permission_level != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("groups.Permission", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.subject.as_ref() {
            struct_ser.serialize_field("subject", v)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        if self.permission_level != 0 {
            let v = PermissionLevel::try_from(self.permission_level)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.permission_level)))?;
            struct_ser.serialize_field("permission_level", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Permission {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "subject",
            "resource",
            "permission_level",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Subject,
            Resource,
            PermissionLevel,
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
                            "subject" => Ok(GeneratedField::Subject),
                            "resource" => Ok(GeneratedField::Resource),
                            "permission_level" => Ok(GeneratedField::PermissionLevel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Permission;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct groups.Permission")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Permission, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut subject__ = None;
                let mut resource__ = None;
                let mut permission_level__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = map_.next_value()?;
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map_.next_value()?;
                        }
                        GeneratedField::PermissionLevel => {
                            if permission_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permission_level"));
                            }
                            permission_level__ = Some(map_.next_value::<PermissionLevel>()? as i32);
                        }
                    }
                }
                Ok(Permission {
                    id: id__.unwrap_or_default(),
                    subject: subject__,
                    resource: resource__,
                    permission_level: permission_level__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("groups.Permission", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PermissionGrantRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subject.is_some() {
            len += 1;
        }
        if self.resource.is_some() {
            len += 1;
        }
        if self.permission_level != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("groups.PermissionGrantRequest", len)?;
        if let Some(v) = self.subject.as_ref() {
            struct_ser.serialize_field("subject", v)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        if self.permission_level != 0 {
            let v = PermissionLevel::try_from(self.permission_level)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.permission_level)))?;
            struct_ser.serialize_field("permission_level", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PermissionGrantRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subject",
            "resource",
            "permission_level",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Subject,
            Resource,
            PermissionLevel,
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
                            "subject" => Ok(GeneratedField::Subject),
                            "resource" => Ok(GeneratedField::Resource),
                            "permission_level" => Ok(GeneratedField::PermissionLevel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PermissionGrantRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct groups.PermissionGrantRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PermissionGrantRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subject__ = None;
                let mut resource__ = None;
                let mut permission_level__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = map_.next_value()?;
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map_.next_value()?;
                        }
                        GeneratedField::PermissionLevel => {
                            if permission_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permission_level"));
                            }
                            permission_level__ = Some(map_.next_value::<PermissionLevel>()? as i32);
                        }
                    }
                }
                Ok(PermissionGrantRequest {
                    subject: subject__,
                    resource: resource__,
                    permission_level: permission_level__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("groups.PermissionGrantRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PermissionLevel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Read => "read",
            Self::Admin => "admin",
            Self::Write => "write",
            Self::Own => "own",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PermissionLevel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "read",
            "admin",
            "write",
            "own",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PermissionLevel;

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
                    "read" => Ok(PermissionLevel::Read),
                    "admin" => Ok(PermissionLevel::Admin),
                    "write" => Ok(PermissionLevel::Write),
                    "own" => Ok(PermissionLevel::Own),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PermissionList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.permissions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("groups.PermissionList", len)?;
        if !self.permissions.is_empty() {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PermissionList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "permissions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Permissions,
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
                            "permissions" => Ok(GeneratedField::Permissions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PermissionList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct groups.PermissionList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PermissionList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut permissions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PermissionList {
                    permissions: permissions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("groups.PermissionList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PermissionPutRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.permission_level != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("groups.PermissionPutRequest", len)?;
        if self.permission_level != 0 {
            let v = PermissionLevel::try_from(self.permission_level)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.permission_level)))?;
            struct_ser.serialize_field("permission_level", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PermissionPutRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "permission_level",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PermissionLevel,
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
                            "permission_level" => Ok(GeneratedField::PermissionLevel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PermissionPutRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct groups.PermissionPutRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PermissionPutRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut permission_level__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PermissionLevel => {
                            if permission_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permission_level"));
                            }
                            permission_level__ = Some(map_.next_value::<PermissionLevel>()? as i32);
                        }
                    }
                }
                Ok(PermissionPutRequest {
                    permission_level: permission_level__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("groups.PermissionPutRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceIn {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.resource_type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("groups.ResourceIn", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.resource_type.is_empty() {
            struct_ser.serialize_field("resource_type", &self.resource_type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceIn {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "resource_type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            "name" => Ok(GeneratedField::Name),
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
            type Value = ResourceIn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct groups.ResourceIn")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceIn, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut resource_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_type"));
                            }
                            resource_type__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResourceIn {
                    name: name__.unwrap_or_default(),
                    resource_type: resource_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("groups.ResourceIn", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceOut {
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
        if !self.resource_type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("groups.ResourceOut", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.resource_type.is_empty() {
            struct_ser.serialize_field("resource_type", &self.resource_type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceOut {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "resource_type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
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
            type Value = ResourceOut;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct groups.ResourceOut")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceOut, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut resource_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceType => {
                            if resource_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_type"));
                            }
                            resource_type__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResourceOut {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    resource_type: resource_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("groups.ResourceOut", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceTypeIn {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("groups.ResourceTypeIn", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceTypeIn {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
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
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceTypeIn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct groups.ResourceTypeIn")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceTypeIn, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                    }
                }
                Ok(ResourceTypeIn {
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("groups.ResourceTypeIn", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceTypeOut {
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
        let mut struct_ser = serializer.serialize_struct("groups.ResourceTypeOut", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceTypeOut {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            Description,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceTypeOut;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct groups.ResourceTypeOut")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceTypeOut, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(ResourceTypeOut {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("groups.ResourceTypeOut", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceTypesOut {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_types.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("groups.ResourceTypesOut", len)?;
        if !self.resource_types.is_empty() {
            struct_ser.serialize_field("resource_types", &self.resource_types)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceTypesOut {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_types",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = ResourceTypesOut;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct groups.ResourceTypesOut")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceTypesOut, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_types__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceTypes => {
                            if resource_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource_types"));
                            }
                            resource_types__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResourceTypesOut {
                    resource_types: resource_types__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("groups.ResourceTypesOut", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("groups.ResourceUpdate", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = ResourceUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct groups.ResourceUpdate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResourceUpdate {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("groups.ResourceUpdate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourcesOut {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resources.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("groups.ResourcesOut", len)?;
        if !self.resources.is_empty() {
            struct_ser.serialize_field("resources", &self.resources)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourcesOut {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resources",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Resources,
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
                            "resources" => Ok(GeneratedField::Resources),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourcesOut;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct groups.ResourcesOut")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourcesOut, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resources__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Resources => {
                            if resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resources"));
                            }
                            resources__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResourcesOut {
                    resources: resources__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("groups.ResourcesOut", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.service.is_empty() {
            len += 1;
        }
        if !self.version.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("groups.ServiceInfo", len)?;
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.service.is_empty() {
            struct_ser.serialize_field("service", &self.service)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServiceInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "description",
            "service",
            "version",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
            Service,
            Version,
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
                            "description" => Ok(GeneratedField::Description),
                            "service" => Ok(GeneratedField::Service),
                            "version" => Ok(GeneratedField::Version),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct groups.ServiceInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ServiceInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                let mut service__ = None;
                let mut version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Service => {
                            if service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("service"));
                            }
                            service__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ServiceInfo {
                    description: description__.unwrap_or_default(),
                    service: service__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("groups.ServiceInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubjectIn {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.subject_id.is_empty() {
            len += 1;
        }
        if !self.subject_type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("groups.SubjectIn", len)?;
        if !self.subject_id.is_empty() {
            struct_ser.serialize_field("subject_id", &self.subject_id)?;
        }
        if !self.subject_type.is_empty() {
            struct_ser.serialize_field("subject_type", &self.subject_type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubjectIn {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subject_id",
            "subject_type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubjectId,
            SubjectType,
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
                            "subject_id" => Ok(GeneratedField::SubjectId),
                            "subject_type" => Ok(GeneratedField::SubjectType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubjectIn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct groups.SubjectIn")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubjectIn, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subject_id__ = None;
                let mut subject_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SubjectId => {
                            if subject_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject_id"));
                            }
                            subject_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SubjectType => {
                            if subject_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject_type"));
                            }
                            subject_type__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SubjectIn {
                    subject_id: subject_id__.unwrap_or_default(),
                    subject_type: subject_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("groups.SubjectIn", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubjectOut {
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
        if !self.subject_id.is_empty() {
            len += 1;
        }
        if !self.subject_type.is_empty() {
            len += 1;
        }
        if !self.subject_source_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("groups.SubjectOut", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.subject_id.is_empty() {
            struct_ser.serialize_field("subject_id", &self.subject_id)?;
        }
        if !self.subject_type.is_empty() {
            struct_ser.serialize_field("subject_type", &self.subject_type)?;
        }
        if !self.subject_source_id.is_empty() {
            struct_ser.serialize_field("subject_source_id", &self.subject_source_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubjectOut {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "subject_id",
            "subject_type",
            "subject_source_id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            SubjectId,
            SubjectType,
            SubjectSourceId,
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
                            "subject_id" => Ok(GeneratedField::SubjectId),
                            "subject_type" => Ok(GeneratedField::SubjectType),
                            "subject_source_id" => Ok(GeneratedField::SubjectSourceId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubjectOut;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct groups.SubjectOut")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubjectOut, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut subject_id__ = None;
                let mut subject_type__ = None;
                let mut subject_source_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SubjectId => {
                            if subject_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject_id"));
                            }
                            subject_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SubjectType => {
                            if subject_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject_type"));
                            }
                            subject_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SubjectSourceId => {
                            if subject_source_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject_source_id"));
                            }
                            subject_source_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SubjectOut {
                    id: id__.unwrap_or_default(),
                    subject_id: subject_id__.unwrap_or_default(),
                    subject_type: subject_type__.unwrap_or_default(),
                    subject_source_id: subject_source_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("groups.SubjectOut", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubjectType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::User => "user",
            Self::Group => "group",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SubjectType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
            "group",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubjectType;

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
                    "user" => Ok(SubjectType::User),
                    "group" => Ok(SubjectType::Group),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SubjectsIn {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.subjects.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("groups.SubjectsIn", len)?;
        if !self.subjects.is_empty() {
            struct_ser.serialize_field("subjects", &self.subjects)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubjectsIn {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subjects",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Subjects,
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
                            "subjects" => Ok(GeneratedField::Subjects),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubjectsIn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct groups.SubjectsIn")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubjectsIn, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subjects__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Subjects => {
                            if subjects__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjects"));
                            }
                            subjects__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SubjectsIn {
                    subjects: subjects__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("groups.SubjectsIn", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubjectsOut {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.subjects.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("groups.SubjectsOut", len)?;
        if !self.subjects.is_empty() {
            struct_ser.serialize_field("subjects", &self.subjects)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubjectsOut {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subjects",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Subjects,
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
                            "subjects" => Ok(GeneratedField::Subjects),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubjectsOut;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct groups.SubjectsOut")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SubjectsOut, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subjects__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Subjects => {
                            if subjects__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjects"));
                            }
                            subjects__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SubjectsOut {
                    subjects: subjects__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("groups.SubjectsOut", FIELDS, GeneratedVisitor)
    }
}
