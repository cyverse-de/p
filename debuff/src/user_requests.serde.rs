// @generated
impl serde::Serialize for AddLoginRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.login.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user_requests.AddLoginRequest", len)?;
        if let Some(v) = self.login.as_ref() {
            struct_ser.serialize_field("login", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddLoginRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "login",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Login,
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
                            "login" => Ok(GeneratedField::Login),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddLoginRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user_requests.AddLoginRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddLoginRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut login__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Login => {
                            if login__.is_some() {
                                return Err(serde::de::Error::duplicate_field("login"));
                            }
                            login__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddLoginRequest {
                    login: login__,
                })
            }
        }
        deserializer.deserialize_struct("user_requests.AddLoginRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteSavedSearchesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user_requests.DeleteSavedSearchesRequest", len)?;
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteSavedSearchesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = DeleteSavedSearchesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user_requests.DeleteSavedSearchesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteSavedSearchesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DeleteSavedSearchesRequest {
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("user_requests.DeleteSavedSearchesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteUserPreferencesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user_requests.DeleteUserPreferencesRequest", len)?;
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteUserPreferencesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = DeleteUserPreferencesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user_requests.DeleteUserPreferencesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteUserPreferencesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DeleteUserPreferencesRequest {
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("user_requests.DeleteUserPreferencesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetLoginsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user.is_some() {
            len += 1;
        }
        if self.continuation.is_some() {
            len += 1;
        }
        if self.page.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user_requests.GetLoginsRequest", len)?;
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        if let Some(v) = self.continuation.as_ref() {
            struct_ser.serialize_field("continuation", v)?;
        }
        if let Some(v) = self.page.as_ref() {
            struct_ser.serialize_field("page", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetLoginsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
            "continuation",
            "page",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Continuation,
            Page,
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
                            "user" => Ok(GeneratedField::User),
                            "continuation" => Ok(GeneratedField::Continuation),
                            "page" => Ok(GeneratedField::Page),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetLoginsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user_requests.GetLoginsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetLoginsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut continuation__ = None;
                let mut page__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                        GeneratedField::Continuation => {
                            if continuation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("continuation"));
                            }
                            continuation__ = map_.next_value()?;
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetLoginsRequest {
                    user: user__,
                    continuation: continuation__,
                    page: page__,
                })
            }
        }
        deserializer.deserialize_struct("user_requests.GetLoginsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSavedSearchesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user_requests.GetSavedSearchesRequest", len)?;
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSavedSearchesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = GetSavedSearchesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user_requests.GetSavedSearchesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSavedSearchesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetSavedSearchesRequest {
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("user_requests.GetSavedSearchesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserPreferencesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user_requests.GetUserPreferencesRequest", len)?;
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserPreferencesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = GetUserPreferencesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user_requests.GetUserPreferencesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserPreferencesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetUserPreferencesRequest {
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("user_requests.GetUserPreferencesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InternalPaginationContinuationToken {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.offset.is_some() {
            len += 1;
        }
        if self.number.is_some() {
            len += 1;
        }
        if self.size.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user_requests.InternalPaginationContinuationToken", len)?;
        if let Some(v) = self.offset.as_ref() {
            struct_ser.serialize_field("offset", v)?;
        }
        if let Some(v) = self.number.as_ref() {
            struct_ser.serialize_field("number", v)?;
        }
        if let Some(v) = self.size.as_ref() {
            struct_ser.serialize_field("size", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InternalPaginationContinuationToken {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offset",
            "number",
            "size",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Offset,
            Number,
            Size,
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
                            "offset" => Ok(GeneratedField::Offset),
                            "number" => Ok(GeneratedField::Number),
                            "size" => Ok(GeneratedField::Size),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InternalPaginationContinuationToken;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user_requests.InternalPaginationContinuationToken")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InternalPaginationContinuationToken, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offset__ = None;
                let mut number__ = None;
                let mut size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Offset => {
                            if offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            offset__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Number => {
                            if number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            number__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(InternalPaginationContinuationToken {
                    offset: offset__,
                    number: number__,
                    size: size__,
                })
            }
        }
        deserializer.deserialize_struct("user_requests.InternalPaginationContinuationToken", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoginsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.list.is_empty() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user_requests.LoginsResponse", len)?;
        if !self.list.is_empty() {
            struct_ser.serialize_field("list", &self.list)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoginsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "list",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            List,
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
                            "list" => Ok(GeneratedField::List),
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
            type Value = LoginsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user_requests.LoginsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoginsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut list__ = None;
                let mut error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::List => {
                            if list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("list"));
                            }
                            list__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
                        }
                    }
                }
                Ok(LoginsResponse {
                    list: list__.unwrap_or_default(),
                    error: error__,
                })
            }
        }
        deserializer.deserialize_struct("user_requests.LoginsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PageSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.offset != 0 {
            len += 1;
        }
        if self.number != 0 {
            len += 1;
        }
        if self.size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user_requests.PageSettings", len)?;
        if self.offset != 0 {
            struct_ser.serialize_field("offset", &self.offset)?;
        }
        if self.number != 0 {
            struct_ser.serialize_field("number", &self.number)?;
        }
        if self.size != 0 {
            struct_ser.serialize_field("size", &self.size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PageSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "offset",
            "number",
            "size",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Offset,
            Number,
            Size,
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
                            "offset" => Ok(GeneratedField::Offset),
                            "number" => Ok(GeneratedField::Number),
                            "size" => Ok(GeneratedField::Size),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PageSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user_requests.PageSettings")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PageSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut offset__ = None;
                let mut number__ = None;
                let mut size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Offset => {
                            if offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            offset__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Number => {
                            if number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PageSettings {
                    offset: offset__.unwrap_or_default(),
                    number: number__.unwrap_or_default(),
                    size: size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("user_requests.PageSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SavedSearchesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.searches.is_some() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user_requests.SavedSearchesResponse", len)?;
        if let Some(v) = self.searches.as_ref() {
            struct_ser.serialize_field("searches", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SavedSearchesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "searches",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Searches,
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
                            "searches" => Ok(GeneratedField::Searches),
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
            type Value = SavedSearchesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user_requests.SavedSearchesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SavedSearchesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut searches__ = None;
                let mut error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Searches => {
                            if searches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searches"));
                            }
                            searches__ = map_.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SavedSearchesResponse {
                    searches: searches__,
                    error: error__,
                })
            }
        }
        deserializer.deserialize_struct("user_requests.SavedSearchesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetSavedSearchesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user.is_some() {
            len += 1;
        }
        if self.searches.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user_requests.SetSavedSearchesRequest", len)?;
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        if let Some(v) = self.searches.as_ref() {
            struct_ser.serialize_field("searches", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetSavedSearchesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
            "searches",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Searches,
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
                            "user" => Ok(GeneratedField::User),
                            "searches" => Ok(GeneratedField::Searches),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetSavedSearchesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user_requests.SetSavedSearchesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetSavedSearchesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut searches__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                        GeneratedField::Searches => {
                            if searches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searches"));
                            }
                            searches__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SetSavedSearchesRequest {
                    user: user__,
                    searches: searches__,
                })
            }
        }
        deserializer.deserialize_struct("user_requests.SetSavedSearchesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetUserPreferencesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user.is_some() {
            len += 1;
        }
        if !self.preferences.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user_requests.SetUserPreferencesRequest", len)?;
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        if !self.preferences.is_empty() {
            struct_ser.serialize_field("preferences", &self.preferences)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetUserPreferencesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
            "preferences",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
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
                            "user" => Ok(GeneratedField::User),
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
            type Value = SetUserPreferencesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user_requests.SetUserPreferencesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetUserPreferencesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut preferences__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                        GeneratedField::Preferences => {
                            if preferences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferences"));
                            }
                            preferences__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SetUserPreferencesRequest {
                    user: user__,
                    preferences: preferences__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("user_requests.SetUserPreferencesRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("user_requests.UserLookupRequest", len)?;
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
                formatter.write_str("struct user_requests.UserLookupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserLookupRequest, V::Error>
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
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IncludeLogins => {
                            if include_logins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeLogins"));
                            }
                            include_logins__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IncludePreferences => {
                            if include_preferences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includePreferences"));
                            }
                            include_preferences__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IncludeSavedSearches => {
                            if include_saved_searches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeSavedSearches"));
                            }
                            include_saved_searches__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoginLimit => {
                            if login_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginLimit"));
                            }
                            login_limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LoginOffset => {
                            if login_offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginOffset"));
                            }
                            login_offset__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Username => {
                            if lookup_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            lookup_ids__ = map_.next_value::<::std::option::Option<_>>()?.map(user_lookup_request::LookupIds::Username);
                        }
                        GeneratedField::UserId => {
                            if lookup_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            lookup_ids__ = map_.next_value::<::std::option::Option<_>>()?.map(user_lookup_request::LookupIds::UserId);
                        }
                        GeneratedField::AnalysisId => {
                            if lookup_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("analysisId"));
                            }
                            lookup_ids__ = map_.next_value::<::std::option::Option<_>>()?.map(user_lookup_request::LookupIds::AnalysisId);
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
        deserializer.deserialize_struct("user_requests.UserLookupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserLookupResponse {
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
        if self.basic_info.is_some() {
            len += 1;
        }
        if !self.logins.is_empty() {
            len += 1;
        }
        if self.preferences.is_some() {
            len += 1;
        }
        if !self.saved_searches.is_empty() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user_requests.UserLookupResponse", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.basic_info.as_ref() {
            struct_ser.serialize_field("basicInfo", v)?;
        }
        if !self.logins.is_empty() {
            struct_ser.serialize_field("logins", &self.logins)?;
        }
        if let Some(v) = self.preferences.as_ref() {
            struct_ser.serialize_field("preferences", v)?;
        }
        if !self.saved_searches.is_empty() {
            struct_ser.serialize_field("savedSearches", &self.saved_searches)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserLookupResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "basic_info",
            "basicInfo",
            "logins",
            "preferences",
            "saved_searches",
            "savedSearches",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            BasicInfo,
            Logins,
            Preferences,
            SavedSearches,
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
                            "basicInfo" | "basic_info" => Ok(GeneratedField::BasicInfo),
                            "logins" => Ok(GeneratedField::Logins),
                            "preferences" => Ok(GeneratedField::Preferences),
                            "savedSearches" | "saved_searches" => Ok(GeneratedField::SavedSearches),
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
            type Value = UserLookupResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user_requests.UserLookupResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserLookupResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut basic_info__ = None;
                let mut logins__ = None;
                let mut preferences__ = None;
                let mut saved_searches__ = None;
                let mut error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::BasicInfo => {
                            if basic_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basicInfo"));
                            }
                            basic_info__ = map_.next_value()?;
                        }
                        GeneratedField::Logins => {
                            if logins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logins"));
                            }
                            logins__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Preferences => {
                            if preferences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferences"));
                            }
                            preferences__ = map_.next_value()?;
                        }
                        GeneratedField::SavedSearches => {
                            if saved_searches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("savedSearches"));
                            }
                            saved_searches__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UserLookupResponse {
                    header: header__,
                    basic_info: basic_info__,
                    logins: logins__.unwrap_or_default(),
                    preferences: preferences__,
                    saved_searches: saved_searches__.unwrap_or_default(),
                    error: error__,
                })
            }
        }
        deserializer.deserialize_struct("user_requests.UserLookupResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserPreferencesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user.is_some() {
            len += 1;
        }
        if self.preferences.is_some() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user_requests.UserPreferencesResponse", len)?;
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        if let Some(v) = self.preferences.as_ref() {
            struct_ser.serialize_field("preferences", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserPreferencesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
            "preferences",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Preferences,
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
                            "user" => Ok(GeneratedField::User),
                            "preferences" => Ok(GeneratedField::Preferences),
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
            type Value = UserPreferencesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user_requests.UserPreferencesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserPreferencesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut preferences__ = None;
                let mut error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                        GeneratedField::Preferences => {
                            if preferences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferences"));
                            }
                            preferences__ = map_.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UserPreferencesResponse {
                    user: user__,
                    preferences: preferences__,
                    error: error__,
                })
            }
        }
        deserializer.deserialize_struct("user_requests.UserPreferencesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user_requests.UserRequest", len)?;
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = UserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user_requests.UserRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UserRequest {
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("user_requests.UserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user.is_some() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user_requests.UserResponse", len)?;
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
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
                            "user" => Ok(GeneratedField::User),
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
            type Value = UserResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user_requests.UserResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UserResponse {
                    user: user__,
                    error: error__,
                })
            }
        }
        deserializer.deserialize_struct("user_requests.UserResponse", FIELDS, GeneratedVisitor)
    }
}
