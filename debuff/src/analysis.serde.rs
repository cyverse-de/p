// @generated
impl serde::Serialize for Analysis {
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
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.app.is_some() {
            len += 1;
        }
        if self.app_version.is_some() {
            len += 1;
        }
        if self.kind.is_some() {
            len += 1;
        }
        if !self.result_folder_path.is_empty() {
            len += 1;
        }
        if self.start_date.is_some() {
            len += 1;
        }
        if self.end_date.is_some() {
            len += 1;
        }
        if self.planned_end_date.is_some() {
            len += 1;
        }
        if !self.status.is_empty() {
            len += 1;
        }
        if self.deleted {
            len += 1;
        }
        if self.notify {
            len += 1;
        }
        if self.user.is_some() {
            len += 1;
        }
        if !self.subdomain.is_empty() {
            len += 1;
        }
        if !self.parent_id.is_empty() {
            len += 1;
        }
        if self.millicores_reserved != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("analysis.Analysis", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.app.as_ref() {
            struct_ser.serialize_field("app", v)?;
        }
        if let Some(v) = self.app_version.as_ref() {
            struct_ser.serialize_field("app_version", v)?;
        }
        if let Some(v) = self.kind.as_ref() {
            struct_ser.serialize_field("type", v)?;
        }
        if !self.result_folder_path.is_empty() {
            struct_ser.serialize_field("result_folder_path", &self.result_folder_path)?;
        }
        if let Some(v) = self.start_date.as_ref() {
            struct_ser.serialize_field("start_date", v)?;
        }
        if let Some(v) = self.end_date.as_ref() {
            struct_ser.serialize_field("end_date", v)?;
        }
        if let Some(v) = self.planned_end_date.as_ref() {
            struct_ser.serialize_field("planned_end_date", v)?;
        }
        if !self.status.is_empty() {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if self.deleted {
            struct_ser.serialize_field("deleted", &self.deleted)?;
        }
        if self.notify {
            struct_ser.serialize_field("notify", &self.notify)?;
        }
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        if !self.subdomain.is_empty() {
            struct_ser.serialize_field("subdomain", &self.subdomain)?;
        }
        if !self.parent_id.is_empty() {
            struct_ser.serialize_field("parent_id", &self.parent_id)?;
        }
        if self.millicores_reserved != 0. {
            struct_ser.serialize_field("millicore_reserved", &self.millicores_reserved)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Analysis {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "description",
            "name",
            "app",
            "app_version",
            "kind",
            "type",
            "result_folder_path",
            "start_date",
            "end_date",
            "planned_end_date",
            "status",
            "deleted",
            "notify",
            "user",
            "subdomain",
            "parent_id",
            "millicores_reserved",
            "millicore_reserved",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Description,
            Name,
            App,
            AppVersion,
            Kind,
            ResultFolderPath,
            StartDate,
            EndDate,
            PlannedEndDate,
            Status,
            Deleted,
            Notify,
            User,
            Subdomain,
            ParentId,
            MillicoresReserved,
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
                            "description" => Ok(GeneratedField::Description),
                            "name" => Ok(GeneratedField::Name),
                            "app" => Ok(GeneratedField::App),
                            "app_version" => Ok(GeneratedField::AppVersion),
                            "type" | "kind" => Ok(GeneratedField::Kind),
                            "result_folder_path" => Ok(GeneratedField::ResultFolderPath),
                            "start_date" => Ok(GeneratedField::StartDate),
                            "end_date" => Ok(GeneratedField::EndDate),
                            "planned_end_date" => Ok(GeneratedField::PlannedEndDate),
                            "status" => Ok(GeneratedField::Status),
                            "deleted" => Ok(GeneratedField::Deleted),
                            "notify" => Ok(GeneratedField::Notify),
                            "user" => Ok(GeneratedField::User),
                            "subdomain" => Ok(GeneratedField::Subdomain),
                            "parent_id" => Ok(GeneratedField::ParentId),
                            "millicore_reserved" | "millicores_reserved" => Ok(GeneratedField::MillicoresReserved),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Analysis;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct analysis.Analysis")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Analysis, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut description__ = None;
                let mut name__ = None;
                let mut app__ = None;
                let mut app_version__ = None;
                let mut kind__ = None;
                let mut result_folder_path__ = None;
                let mut start_date__ = None;
                let mut end_date__ = None;
                let mut planned_end_date__ = None;
                let mut status__ = None;
                let mut deleted__ = None;
                let mut notify__ = None;
                let mut user__ = None;
                let mut subdomain__ = None;
                let mut parent_id__ = None;
                let mut millicores_reserved__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::App => {
                            if app__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app"));
                            }
                            app__ = map_.next_value()?;
                        }
                        GeneratedField::AppVersion => {
                            if app_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app_version"));
                            }
                            app_version__ = map_.next_value()?;
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            kind__ = map_.next_value()?;
                        }
                        GeneratedField::ResultFolderPath => {
                            if result_folder_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result_folder_path"));
                            }
                            result_folder_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartDate => {
                            if start_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start_date"));
                            }
                            start_date__ = map_.next_value()?;
                        }
                        GeneratedField::EndDate => {
                            if end_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end_date"));
                            }
                            end_date__ = map_.next_value()?;
                        }
                        GeneratedField::PlannedEndDate => {
                            if planned_end_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("planned_end_date"));
                            }
                            planned_end_date__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Deleted => {
                            if deleted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleted"));
                            }
                            deleted__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Notify => {
                            if notify__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notify"));
                            }
                            notify__ = Some(map_.next_value()?);
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                        GeneratedField::Subdomain => {
                            if subdomain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subdomain"));
                            }
                            subdomain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ParentId => {
                            if parent_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parent_id"));
                            }
                            parent_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MillicoresReserved => {
                            if millicores_reserved__.is_some() {
                                return Err(serde::de::Error::duplicate_field("millicore_reserved"));
                            }
                            millicores_reserved__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Analysis {
                    id: id__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    app: app__,
                    app_version: app_version__,
                    kind: kind__,
                    result_folder_path: result_folder_path__.unwrap_or_default(),
                    start_date: start_date__,
                    end_date: end_date__,
                    planned_end_date: planned_end_date__,
                    status: status__.unwrap_or_default(),
                    deleted: deleted__.unwrap_or_default(),
                    notify: notify__.unwrap_or_default(),
                    user: user__,
                    subdomain: subdomain__.unwrap_or_default(),
                    parent_id: parent_id__.unwrap_or_default(),
                    millicores_reserved: millicores_reserved__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("analysis.Analysis", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("analysis.AnalysisRecordList", len)?;
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
                formatter.write_str("struct analysis.AnalysisRecordList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnalysisRecordList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut analyses__ = None;
                let mut error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Analyses => {
                            if analyses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("analyses"));
                            }
                            analyses__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
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
        deserializer.deserialize_struct("analysis.AnalysisRecordList", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("analysis.AnalysisRecordLookupRequest", len)?;
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
                formatter.write_str("struct analysis.AnalysisRecordLookupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnalysisRecordLookupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut requesting_user__ = None;
                let mut lookup_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::RequestingUser => {
                            if requesting_user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestingUser"));
                            }
                            requesting_user__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnalysisId => {
                            if lookup_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("analysisId"));
                            }
                            lookup_ids__ = map_.next_value::<::std::option::Option<_>>()?.map(analysis_record_lookup_request::LookupIds::AnalysisId);
                        }
                        GeneratedField::ExternalId => {
                            if lookup_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalId"));
                            }
                            lookup_ids__ = map_.next_value::<::std::option::Option<_>>()?.map(analysis_record_lookup_request::LookupIds::ExternalId);
                        }
                        GeneratedField::UserId => {
                            if lookup_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            lookup_ids__ = map_.next_value::<::std::option::Option<_>>()?.map(analysis_record_lookup_request::LookupIds::UserId);
                        }
                        GeneratedField::Username => {
                            if lookup_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            lookup_ids__ = map_.next_value::<::std::option::Option<_>>()?.map(analysis_record_lookup_request::LookupIds::Username);
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
        deserializer.deserialize_struct("analysis.AnalysisRecordLookupRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("analysis.AnalysisRecordResponse", len)?;
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
            #[allow(clippy::needless_borrow)]
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
                formatter.write_str("struct analysis.AnalysisRecordResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnalysisRecordResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut analyses__ = None;
                let mut timestamp__ = None;
                let mut total__ = None;
                let mut status_count__ = None;
                let mut error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map_.next_value()?;
                        }
                        GeneratedField::Analyses => {
                            if analyses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("analyses"));
                            }
                            analyses__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StatusCount => {
                            if status_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status-count"));
                            }
                            status_count__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
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
        deserializer.deserialize_struct("analysis.AnalysisRecordResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("analysis.AnalysisRecordResponse.StatusCountRecord", len)?;
        if self.count != 0 {
            #[allow(clippy::needless_borrow)]
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
                formatter.write_str("struct analysis.AnalysisRecordResponse.StatusCountRecord")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<analysis_record_response::StatusCountRecord, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut count__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Count => {
                            if count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(analysis_record_response::StatusCountRecord {
                    count: count__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("analysis.AnalysisRecordResponse.StatusCountRecord", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("analysis.AnalysisStatus", len)?;
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
                formatter.write_str("struct analysis.AnalysisStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnalysisStatus, V::Error>
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
                        GeneratedField::Job => {
                            if job__.is_some() {
                                return Err(serde::de::Error::duplicate_field("job"));
                            }
                            job__ = map_.next_value()?;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SentOn => {
                            if sent_on__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sent_on"));
                            }
                            sent_on__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
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
        deserializer.deserialize_struct("analysis.AnalysisStatus", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("analysis.AnalysisSubmission", len)?;
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
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("failure_count", ToString::to_string(&self.failure_count).as_str())?;
        }
        if self.failure_threshold != 0 {
            #[allow(clippy::needless_borrow)]
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
            type Value = AnalysisSubmission;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct analysis.AnalysisSubmission")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnalysisSubmission, V::Error>
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
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AppDescription => {
                            if app_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app_description"));
                            }
                            app_description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppId => {
                            if app_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app_id"));
                            }
                            app_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppName => {
                            if app_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app_name"));
                            }
                            app_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ArchiveLogs => {
                            if archive_logs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archive_logs"));
                            }
                            archive_logs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BatchId => {
                            if batch_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batch_id"));
                            }
                            batch_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CondorId => {
                            if condor_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("condor_id"));
                            }
                            condor_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CondorLogPath => {
                            if condor_log_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("condor_log_path"));
                            }
                            condor_log_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreateOutputSubdir => {
                            if create_output_subdir__.is_some() {
                                return Err(serde::de::Error::duplicate_field("create_output_subdir"));
                            }
                            create_output_subdir__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DateSubmitted => {
                            if date_submitted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("date_submitted"));
                            }
                            date_submitted__ = map_.next_value()?;
                        }
                        GeneratedField::DateStarted => {
                            if date_started__.is_some() {
                                return Err(serde::de::Error::duplicate_field("date_started"));
                            }
                            date_started__ = map_.next_value()?;
                        }
                        GeneratedField::DateCompleted => {
                            if date_completed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("date_completed"));
                            }
                            date_completed__ = map_.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Extra => {
                            if extra__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extra"));
                            }
                            extra__ = map_.next_value()?;
                        }
                        GeneratedField::ExecutionTarget => {
                            if execution_target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("execution_target"));
                            }
                            execution_target__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExitCode => {
                            if exit_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exit_code"));
                            }
                            exit_code__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FailureCount => {
                            if failure_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failure_count"));
                            }
                            failure_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FailureThreshold => {
                            if failure_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failure_threshold"));
                            }
                            failure_threshold__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FileMetadata => {
                            if file_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("file-metadata"));
                            }
                            file_metadata__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FilterFiles => {
                            if filter_files__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter_files"));
                            }
                            filter_files__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InputPathListFile => {
                            if input_path_list_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input_path_list"));
                            }
                            input_path_list_file__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InputTicketsFile => {
                            if input_tickets_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input_ticket_list"));
                            }
                            input_tickets_file__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InvocationId => {
                            if invocation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("invocation_id"));
                            }
                            invocation_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IrodsBase => {
                            if irods_base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("irods_base"));
                            }
                            irods_base__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NfsBase => {
                            if nfs_base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nfs_base"));
                            }
                            nfs_base__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Notify => {
                            if notify__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notify"));
                            }
                            notify__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NowDate => {
                            if now_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("now_date"));
                            }
                            now_date__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OutputDir => {
                            if output_dir__.is_some() {
                                return Err(serde::de::Error::duplicate_field("output_dir"));
                            }
                            output_dir__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OutputDirTicket => {
                            if output_dir_ticket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("output_dir_ticket"));
                            }
                            output_dir_ticket__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OutputTicketFile => {
                            if output_ticket_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("output_ticket_file"));
                            }
                            output_ticket_file__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestType => {
                            if request_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request_type"));
                            }
                            request_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunOnNfs => {
                            if run_on_nfs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("run-on-nfs"));
                            }
                            run_on_nfs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SkipParentMetadata => {
                            if skip_parent_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skip-parent-meta"));
                            }
                            skip_parent_metadata__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Steps => {
                            if steps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("steps"));
                            }
                            steps__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SubmissionDate => {
                            if submission_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("submission_date"));
                            }
                            submission_date__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Submitter => {
                            if submitter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            submitter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_id"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserGroups => {
                            if user_groups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_groups"));
                            }
                            user_groups__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserHome => {
                            if user_home__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_home"));
                            }
                            user_home__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WikiUrl => {
                            if wiki_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wiki_url"));
                            }
                            wiki_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConfigFile => {
                            if config_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config_file"));
                            }
                            config_file__ = Some(map_.next_value()?);
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
                })
            }
        }
        deserializer.deserialize_struct("analysis.AnalysisSubmission", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnalysisType {
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
        if !self.system_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("analysis.AnalysisType", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.system_id.is_empty() {
            struct_ser.serialize_field("system_id", &self.system_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnalysisType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "system_id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            SystemId,
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
                            "system_id" => Ok(GeneratedField::SystemId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnalysisType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct analysis.AnalysisType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnalysisType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut system_id__ = None;
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
                        GeneratedField::SystemId => {
                            if system_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("system_id"));
                            }
                            system_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AnalysisType {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    system_id: system_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("analysis.AnalysisType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchStatus {
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
        let mut struct_ser = serializer.serialize_struct("analysis.BatchStatus", len)?;
        if self.total != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("total", ToString::to_string(&self.total).as_str())?;
        }
        if self.completed != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("completed", ToString::to_string(&self.completed).as_str())?;
        }
        if self.running != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("running", ToString::to_string(&self.running).as_str())?;
        }
        if self.submitted != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("submitted", ToString::to_string(&self.submitted).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchStatus {
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
            type Value = BatchStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct analysis.BatchStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut total__ = None;
                let mut completed__ = None;
                let mut running__ = None;
                let mut submitted__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Completed => {
                            if completed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("completed"));
                            }
                            completed__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Running => {
                            if running__.is_some() {
                                return Err(serde::de::Error::duplicate_field("running"));
                            }
                            running__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Submitted => {
                            if submitted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("submitted"));
                            }
                            submitted__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(BatchStatus {
                    total: total__.unwrap_or_default(),
                    completed: completed__.unwrap_or_default(),
                    running: running__.unwrap_or_default(),
                    submitted: submitted__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("analysis.BatchStatus", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("analysis.Extra", len)?;
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
                formatter.write_str("struct analysis.Extra")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Extra, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ht_condor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HtCondor => {
                            if ht_condor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("htCondor"));
                            }
                            ht_condor__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Extra {
                    ht_condor: ht_condor__,
                })
            }
        }
        deserializer.deserialize_struct("analysis.Extra", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("analysis.FileMetadata", len)?;
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
                formatter.write_str("struct analysis.FileMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FileMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attribute__ = None;
                let mut value__ = None;
                let mut unit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attribute => {
                            if attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attribute"));
                            }
                            attribute__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value()?);
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
        deserializer.deserialize_struct("analysis.FileMetadata", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("analysis.HTCondorExtraInfo", len)?;
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
                formatter.write_str("struct analysis.HTCondorExtraInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HtCondorExtraInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut extra_requirements__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExtraRequirements => {
                            if extra_requirements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extraRequirements"));
                            }
                            extra_requirements__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HtCondorExtraInfo {
                    extra_requirements: extra_requirements__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("analysis.HTCondorExtraInfo", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("analysis.Job", len)?;
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
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("failure_count", ToString::to_string(&self.failure_count).as_str())?;
        }
        if self.failure_threshold != 0 {
            #[allow(clippy::needless_borrow)]
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
                formatter.write_str("struct analysis.Job")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Job, V::Error>
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
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AppDescription => {
                            if app_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app_description"));
                            }
                            app_description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppId => {
                            if app_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app_id"));
                            }
                            app_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AppName => {
                            if app_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app_name"));
                            }
                            app_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ArchiveLogs => {
                            if archive_logs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archive_logs"));
                            }
                            archive_logs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BatchId => {
                            if batch_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batch_id"));
                            }
                            batch_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CondorId => {
                            if condor_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("condor_id"));
                            }
                            condor_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CondorLogPath => {
                            if condor_log_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("condor_log_path"));
                            }
                            condor_log_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreateOutputSubdir => {
                            if create_output_subdir__.is_some() {
                                return Err(serde::de::Error::duplicate_field("create_output_subdir"));
                            }
                            create_output_subdir__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DateSubmitted => {
                            if date_submitted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("date_submitted"));
                            }
                            date_submitted__ = map_.next_value()?;
                        }
                        GeneratedField::DateStarted => {
                            if date_started__.is_some() {
                                return Err(serde::de::Error::duplicate_field("date_started"));
                            }
                            date_started__ = map_.next_value()?;
                        }
                        GeneratedField::DateCompleted => {
                            if date_completed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("date_completed"));
                            }
                            date_completed__ = map_.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Extra => {
                            if extra__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extra"));
                            }
                            extra__ = map_.next_value()?;
                        }
                        GeneratedField::ExecutionTarget => {
                            if execution_target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("execution_target"));
                            }
                            execution_target__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExitCode => {
                            if exit_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exit_code"));
                            }
                            exit_code__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FailureCount => {
                            if failure_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failure_count"));
                            }
                            failure_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FailureThreshold => {
                            if failure_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failure_threshold"));
                            }
                            failure_threshold__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FileMetadata => {
                            if file_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("file-metadata"));
                            }
                            file_metadata__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FilterFiles => {
                            if filter_files__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter_files"));
                            }
                            filter_files__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InputPathListFile => {
                            if input_path_list_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input_path_list"));
                            }
                            input_path_list_file__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InputTicketsFile => {
                            if input_tickets_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input_ticket_list"));
                            }
                            input_tickets_file__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InvocationId => {
                            if invocation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("invocation_id"));
                            }
                            invocation_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IrodsBase => {
                            if irods_base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("irods_base"));
                            }
                            irods_base__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NfsBase => {
                            if nfs_base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nfs_base"));
                            }
                            nfs_base__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Notify => {
                            if notify__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notify"));
                            }
                            notify__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NowDate => {
                            if now_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("now_date"));
                            }
                            now_date__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OutputDir => {
                            if output_dir__.is_some() {
                                return Err(serde::de::Error::duplicate_field("output_dir"));
                            }
                            output_dir__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OutputDirTicket => {
                            if output_dir_ticket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("output_dir_ticket"));
                            }
                            output_dir_ticket__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OutputTicketFile => {
                            if output_ticket_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("output_ticket_file"));
                            }
                            output_ticket_file__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RequestType => {
                            if request_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request_type"));
                            }
                            request_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunOnNfs => {
                            if run_on_nfs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("run-on-nfs"));
                            }
                            run_on_nfs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SkipParentMetadata => {
                            if skip_parent_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skip-parent-meta"));
                            }
                            skip_parent_metadata__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Steps => {
                            if steps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("steps"));
                            }
                            steps__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SubmissionDate => {
                            if submission_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("submission_date"));
                            }
                            submission_date__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Submitter => {
                            if submitter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            submitter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_id"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserGroups => {
                            if user_groups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_groups"));
                            }
                            user_groups__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserHome => {
                            if user_home__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_home"));
                            }
                            user_home__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WikiUrl => {
                            if wiki_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wiki_url"));
                            }
                            wiki_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConfigFile => {
                            if config_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config_file"));
                            }
                            config_file__ = Some(map_.next_value()?);
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
        deserializer.deserialize_struct("analysis.Job", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("analysis.Step", len)?;
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
                formatter.write_str("struct analysis.Step")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Step, V::Error>
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
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Component => {
                            if component__.is_some() {
                                return Err(serde::de::Error::duplicate_field("component"));
                            }
                            component__ = map_.next_value()?;
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StdinPath => {
                            if stdin_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stdin"));
                            }
                            stdin_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StdoutPath => {
                            if stdout_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stdout"));
                            }
                            stdout_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StderrPath => {
                            if stderr_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stderr"));
                            }
                            stderr_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LogFile => {
                            if log_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("log-file"));
                            }
                            log_file__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Environment => {
                            if environment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("environment"));
                            }
                            environment__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Input => {
                            if input__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input"));
                            }
                            input__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Output => {
                            if output__.is_some() {
                                return Err(serde::de::Error::duplicate_field("output"));
                            }
                            output__ = Some(map_.next_value()?);
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
                })
            }
        }
        deserializer.deserialize_struct("analysis.Step", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("analysis.Step.Component", len)?;
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
                formatter.write_str("struct analysis.Step.Component")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<step::Component, V::Error>
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
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Container => {
                            if container__.is_some() {
                                return Err(serde::de::Error::duplicate_field("container"));
                            }
                            container__ = map_.next_value()?;
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Location => {
                            if location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            location__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TimeLimit => {
                            if time_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time_limit_seconds"));
                            }
                            time_limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Restricted => {
                            if restricted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restricted"));
                            }
                            restricted__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsInteractive => {
                            if is_interactive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interactive"));
                            }
                            is_interactive__ = Some(map_.next_value()?);
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
        deserializer.deserialize_struct("analysis.Step.Component", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("analysis.Step.Config", len)?;
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
                formatter.write_str("struct analysis.Step.Config")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<step::Config, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut inputs__ = None;
                let mut outputs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Inputs => {
                            if inputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input"));
                            }
                            inputs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Outputs => {
                            if outputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("output"));
                            }
                            outputs__ = Some(map_.next_value()?);
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
        deserializer.deserialize_struct("analysis.Step.Config", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("analysis.Step.Input", len)?;
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
                formatter.write_str("struct analysis.Step.Input")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<step::Input, V::Error>
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
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Ticket => {
                            if ticket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ticket"));
                            }
                            ticket__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Multiplicity => {
                            if multiplicity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multiplicity"));
                            }
                            multiplicity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Property => {
                            if property__.is_some() {
                                return Err(serde::de::Error::duplicate_field("property"));
                            }
                            property__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Retain => {
                            if retain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retain"));
                            }
                            retain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
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
        deserializer.deserialize_struct("analysis.Step.Input", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("analysis.Step.Output", len)?;
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
                formatter.write_str("struct analysis.Step.Output")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<step::Output, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut multiplicity__ = None;
                let mut name__ = None;
                let mut property__ = None;
                let mut qual_id__ = None;
                let mut retain__ = None;
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Multiplicity => {
                            if multiplicity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multiplicity"));
                            }
                            multiplicity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Property => {
                            if property__.is_some() {
                                return Err(serde::de::Error::duplicate_field("property"));
                            }
                            property__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QualId => {
                            if qual_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("qual-id"));
                            }
                            qual_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Retain => {
                            if retain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retain"));
                            }
                            retain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
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
        deserializer.deserialize_struct("analysis.Step.Output", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("analysis.Step.Param", len)?;
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
                formatter.write_str("struct analysis.Step.Param")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<step::Param, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut value__ = None;
                let mut order__ = None;
                let mut r#type__ = None;
                let mut path__ = None;
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
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Order => {
                            if order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            order__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
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
        deserializer.deserialize_struct("analysis.Step.Param", FIELDS, GeneratedVisitor)
    }
}
