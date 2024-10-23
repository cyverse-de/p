// @generated
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
