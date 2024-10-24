// @generated
impl serde::Serialize for Login {
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
        let mut struct_ser = serializer.serialize_struct("user.Login", len)?;
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
impl<'de> serde::Deserialize<'de> for Login {
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
            type Value = Login;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user.Login")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Login, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut ip_address__ = None;
                let mut user_agent__ = None;
                let mut login_time__ = None;
                let mut logout_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IpAddress => {
                            if ip_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipAddress"));
                            }
                            ip_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserAgent => {
                            if user_agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAgent"));
                            }
                            user_agent__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LoginTime => {
                            if login_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginTime"));
                            }
                            login_time__ = map_.next_value()?;
                        }
                        GeneratedField::LogoutTime => {
                            if logout_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logoutTime"));
                            }
                            logout_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Login {
                    uuid: uuid__.unwrap_or_default(),
                    ip_address: ip_address__.unwrap_or_default(),
                    user_agent: user_agent__.unwrap_or_default(),
                    login_time: login_time__,
                    logout_time: logout_time__,
                })
            }
        }
        deserializer.deserialize_struct("user.Login", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoginIp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user.LoginIP", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoginIp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoginIp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user.LoginIP")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoginIp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LoginIp {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("user.LoginIP", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoginListWire {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.logins.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user.LoginListWire", len)?;
        if !self.logins.is_empty() {
            struct_ser.serialize_field("logins", &self.logins)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoginListWire {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "logins",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Logins,
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
                            "logins" => Ok(GeneratedField::Logins),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoginListWire;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user.LoginListWire")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoginListWire, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut logins__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Logins => {
                            if logins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logins"));
                            }
                            logins__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LoginListWire {
                    logins: logins__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("user.LoginListWire", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoginStorage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.ip_address.is_some() {
            len += 1;
        }
        if self.user_agent.is_some() {
            len += 1;
        }
        if self.login_time.is_some() {
            len += 1;
        }
        if self.logout_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user.LoginStorage", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.ip_address.as_ref() {
            struct_ser.serialize_field("ipAddress", v)?;
        }
        if let Some(v) = self.user_agent.as_ref() {
            struct_ser.serialize_field("userAgent", v)?;
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
impl<'de> serde::Deserialize<'de> for LoginStorage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
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
            UserId,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
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
            type Value = LoginStorage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user.LoginStorage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoginStorage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut ip_address__ = None;
                let mut user_agent__ = None;
                let mut login_time__ = None;
                let mut logout_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IpAddress => {
                            if ip_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipAddress"));
                            }
                            ip_address__ = map_.next_value()?;
                        }
                        GeneratedField::UserAgent => {
                            if user_agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAgent"));
                            }
                            user_agent__ = map_.next_value()?;
                        }
                        GeneratedField::LoginTime => {
                            if login_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginTime"));
                            }
                            login_time__ = map_.next_value()?;
                        }
                        GeneratedField::LogoutTime => {
                            if logout_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logoutTime"));
                            }
                            logout_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(LoginStorage {
                    user_id: user_id__.unwrap_or_default(),
                    ip_address: ip_address__,
                    user_agent: user_agent__,
                    login_time: login_time__,
                    logout_time: logout_time__,
                })
            }
        }
        deserializer.deserialize_struct("user.LoginStorage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoginUserAgent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.full.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user.LoginUserAgent", len)?;
        if !self.full.is_empty() {
            struct_ser.serialize_field("full", &self.full)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoginUserAgent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "full",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Full,
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
                            "full" => Ok(GeneratedField::Full),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoginUserAgent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user.LoginUserAgent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoginUserAgent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut full__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Full => {
                            if full__.is_some() {
                                return Err(serde::de::Error::duplicate_field("full"));
                            }
                            full__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LoginUserAgent {
                    full: full__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("user.LoginUserAgent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoginWire {
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
        if self.ip.is_some() {
            len += 1;
        }
        if self.user_agent.is_some() {
            len += 1;
        }
        if self.login_time.is_some() {
            len += 1;
        }
        if self.logout_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user.LoginWire", len)?;
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        if let Some(v) = self.ip.as_ref() {
            struct_ser.serialize_field("ip", v)?;
        }
        if let Some(v) = self.user_agent.as_ref() {
            struct_ser.serialize_field("userAgent", v)?;
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
impl<'de> serde::Deserialize<'de> for LoginWire {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
            "ip",
            "user_agent",
            "userAgent",
            "login_time",
            "loginTime",
            "logout_time",
            "logoutTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Ip,
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
                            "user" => Ok(GeneratedField::User),
                            "ip" => Ok(GeneratedField::Ip),
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
            type Value = LoginWire;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user.LoginWire")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoginWire, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut ip__ = None;
                let mut user_agent__ = None;
                let mut login_time__ = None;
                let mut logout_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                        GeneratedField::Ip => {
                            if ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ip"));
                            }
                            ip__ = map_.next_value()?;
                        }
                        GeneratedField::UserAgent => {
                            if user_agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAgent"));
                            }
                            user_agent__ = map_.next_value()?;
                        }
                        GeneratedField::LoginTime => {
                            if login_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginTime"));
                            }
                            login_time__ = map_.next_value()?;
                        }
                        GeneratedField::LogoutTime => {
                            if logout_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logoutTime"));
                            }
                            logout_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(LoginWire {
                    user: user__,
                    ip: ip__,
                    user_agent: user_agent__,
                    login_time: login_time__,
                    logout_time: logout_time__,
                })
            }
        }
        deserializer.deserialize_struct("user.LoginWire", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Preferences {
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
        let mut struct_ser = serializer.serialize_struct("user.Preferences", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.preferences.is_empty() {
            struct_ser.serialize_field("preferences", &self.preferences)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Preferences {
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
            type Value = Preferences;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user.Preferences")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Preferences, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut preferences__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Preferences => {
                            if preferences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferences"));
                            }
                            preferences__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Preferences {
                    uuid: uuid__.unwrap_or_default(),
                    preferences: preferences__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("user.Preferences", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SavedSearches {
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
        let mut struct_ser = serializer.serialize_struct("user.SavedSearches", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.saved_searches.is_empty() {
            struct_ser.serialize_field("savedSearches", &self.saved_searches)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SavedSearches {
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
            type Value = SavedSearches;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user.SavedSearches")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SavedSearches, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut saved_searches__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SavedSearches => {
                            if saved_searches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("savedSearches"));
                            }
                            saved_searches__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SavedSearches {
                    uuid: uuid__.unwrap_or_default(),
                    saved_searches: saved_searches__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("user.SavedSearches", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("user.User", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
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
            type Value = User;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user.User")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<User, V::Error>
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
                Ok(User {
                    uuid: uuid__.unwrap_or_default(),
                    username: username__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("user.User", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserRef {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.username.is_some() {
            len += 1;
        }
        if self.uuid.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("user.UserRef", len)?;
        if let Some(v) = self.username.as_ref() {
            struct_ser.serialize_field("username", v)?;
        }
        if let Some(v) = self.uuid.as_ref() {
            struct_ser.serialize_field("uuid", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserRef {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "username",
            "uuid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Username,
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
                            "username" => Ok(GeneratedField::Username),
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
            type Value = UserRef;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct user.UserRef")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserRef, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut username__ = None;
                let mut uuid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = map_.next_value()?;
                        }
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UserRef {
                    username: username__,
                    uuid: uuid__,
                })
            }
        }
        deserializer.deserialize_struct("user.UserRef", FIELDS, GeneratedVisitor)
    }
}
