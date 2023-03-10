mod analysis;
pub mod header;
pub mod qms;
pub mod svcerror;

pub mod vice {
    use crate::analysis::*;
    pub type VICE = InteractiveApps;

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_vice() {
            let mut v = VICE::default();
            v.proxy_image = "test_proxy_image".to_owned();
            v.proxy_name = "test_proxy_name".to_owned();
            v.frontend_url = "test_frontend_url".to_owned();
            v.cas_url = "test_cas_url".to_owned();
            v.cas_validate = "test_cas_validate".to_owned();

            let c = v.clone();

            let s = serde_json::to_string(&v).unwrap();
            let d: VICE = serde_json::from_str(&s).unwrap();

            assert_eq!(d.proxy_image, c.proxy_image);
            assert_eq!(d.proxy_name, c.proxy_name);
            assert_eq!(d.frontend_url, c.frontend_url);
            assert_eq!(d.cas_url, c.cas_url);
            assert_eq!(d.cas_validate, c.cas_validate);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::header::header::Value;
    use crate::header::Header;
    use crate::qms::{ResourceType, ResourceTypeResponse};
    use crate::svcerror::ErrorCode;
    use crate::svcerror::ServiceError;
    use serde_json;

    #[test]
    fn test_header() {
        // Initialize a header.
        let mut h = Header::default();

        // Initialize a value and set it.
        let mut v = Value::default();
        v.value.push(String::from("value"));

        // Insert the value into the header map.
        h.map.insert(String::from("foo"), v);

        // clone the header for comparisons later.
        let test_value = h.clone();

        // serialize and deserialize the header to/from JSON
        let s = serde_json::to_string(&h).unwrap();
        let u: Header = serde_json::from_str(&s).unwrap();

        // get the value from the deserialized header
        let u_val = u.map.get("foo").unwrap();

        // Pull the value from the cloned header
        let v_val = test_value.map.get("foo").unwrap();

        // make sure the cloned value matches the deserialized value
        assert_eq!(u_val.value[0], v_val.value[0]);
    }

    #[test]
    fn test_service_error() {
        let mut se = ServiceError::default();
        let mut h = Header::default();
        let mut v = Value::default();

        v.value.push("value".to_owned());
        h.map.insert("test_key".to_owned(), v);

        let comp_header = h.clone();

        se.error_code = ErrorCode::BadRequest.into();
        se.header = Some(h);
        se.message = "this is a test".to_owned();
        se.status_code = 400;

        let comp_se = se.clone();

        let ser = serde_json::to_string(&se).unwrap();
        let des: ServiceError = serde_json::from_str(&ser).unwrap();

        let des_header = des.header.as_ref().unwrap();
        let des_error_code = des.error_code();

        assert_eq!(
            des_header.map.get(&"test_key".to_owned()).unwrap().value[0],
            comp_header.map.get(&"test_key".to_owned()).unwrap().value[0],
        );

        assert_eq!(des_error_code, comp_se.error_code());
        assert_eq!(des.message, comp_se.message);
        assert_eq!(des.status_code, comp_se.status_code);
    }

    #[test]
    fn test_resource_type() {
        let mut rt = ResourceType::default();

        assert_eq!(rt.name, "");
        assert_eq!(rt.unit, "");
        assert_eq!(rt.uuid, "");

        rt.name = String::from("name");
        rt.unit = String::from("unit");
        rt.uuid = String::from("uuid");

        assert_eq!(rt.name, "name");
        assert_eq!(rt.unit, "unit");
        assert_eq!(rt.uuid, "uuid");

        let c = rt.clone();
        let s = serde_json::to_string(&rt).unwrap();
        let d: ResourceType = serde_json::from_str(&s).unwrap();

        assert_eq!(c.name, d.name);
        assert_eq!(c.unit, d.unit);
        assert_eq!(c.uuid, d.uuid);
    }

    #[test]
    fn test_resource_type_response() {
        let mut rtq = ResourceTypeResponse::default();

        assert!(rtq.resource_type.is_none());
        assert!(rtq.error.is_none());
        assert!(rtq.header.is_none());

        let rt = ResourceType::default();
        rtq.resource_type = Some(rt);

        let c1 = rtq.clone();
        let c1_rt = c1.resource_type.unwrap();

        assert_eq!(c1_rt.name, "");
        assert_eq!(c1_rt.unit, "");
        assert_eq!(c1_rt.uuid, "");
    }
}
