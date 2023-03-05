// @generated
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
        let mut struct_ser = serializer.serialize_struct("user.User", len)?;
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
                formatter.write_str("struct user.User")
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
        deserializer.deserialize_struct("user.User", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("user.User.Login", len)?;
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
                formatter.write_str("struct user.User.Login")
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
        deserializer.deserialize_struct("user.User.Login", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("user.User.Preferences", len)?;
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
                formatter.write_str("struct user.User.Preferences")
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
        deserializer.deserialize_struct("user.User.Preferences", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("user.User.SavedSearches", len)?;
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
                formatter.write_str("struct user.User.SavedSearches")
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
        deserializer.deserialize_struct("user.User.SavedSearches", FIELDS, GeneratedVisitor)
    }
}
