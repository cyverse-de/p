// @generated
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
        let mut struct_ser = serializer.serialize_struct("containers.Container", len)?;
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
                formatter.write_str("struct containers.Container")
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
                })
            }
        }
        deserializer.deserialize_struct("containers.Container", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Device {
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
        let mut struct_ser = serializer.serialize_struct("containers.Device", len)?;
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
impl<'de> serde::Deserialize<'de> for Device {
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
            type Value = Device;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct containers.Device")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Device, V::Error>
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
                Ok(Device {
                    host_path: host_path__.unwrap_or_default(),
                    container_path: container_path__.unwrap_or_default(),
                    cgroup_permissions: cgroup_permissions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("containers.Device", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Image {
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
        let mut struct_ser = serializer.serialize_struct("containers.Image", len)?;
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
impl<'de> serde::Deserialize<'de> for Image {
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
            type Value = Image;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct containers.Image")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Image, V::Error>
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
                Ok(Image {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    tag: tag__.unwrap_or_default(),
                    auth: auth__.unwrap_or_default(),
                    url: url__.unwrap_or_default(),
                    osg_image_path: osg_image_path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("containers.Image", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("containers.InteractiveApps", len)?;
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
                formatter.write_str("struct containers.InteractiveApps")
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
                })
            }
        }
        deserializer.deserialize_struct("containers.InteractiveApps", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Port {
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
        let mut struct_ser = serializer.serialize_struct("containers.Port", len)?;
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
impl<'de> serde::Deserialize<'de> for Port {
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
            type Value = Port;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct containers.Port")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Port, V::Error>
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
                Ok(Port {
                    host_port: host_port__.unwrap_or_default(),
                    container_port: container_port__.unwrap_or_default(),
                    bind_to_host: bind_to_host__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("containers.Port", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Volume {
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
        let mut struct_ser = serializer.serialize_struct("containers.Volume", len)?;
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
impl<'de> serde::Deserialize<'de> for Volume {
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
            type Value = Volume;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct containers.Volume")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Volume, V::Error>
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
                Ok(Volume {
                    host_path: host_path__.unwrap_or_default(),
                    container_path: container_path__.unwrap_or_default(),
                    read_only: read_only__.unwrap_or_default(),
                    mode: mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("containers.Volume", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VolumesFrom {
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
        let mut struct_ser = serializer.serialize_struct("containers.VolumesFrom", len)?;
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
impl<'de> serde::Deserialize<'de> for VolumesFrom {
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
            type Value = VolumesFrom;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct containers.VolumesFrom")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VolumesFrom, V::Error>
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
                Ok(VolumesFrom {
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
        deserializer.deserialize_struct("containers.VolumesFrom", FIELDS, GeneratedVisitor)
    }
}
