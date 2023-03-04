pub mod debuff;

#[cfg(test)]
mod test {
    use crate::debuff::{header, Header};
    use serde_json;

    #[test]
    fn test_header() {
        let mut h = Header::default();

        let v = header::Value {
            value: vec![String::from("value")],
        };

        h.map.insert(String::from("foo"), v);

        let test_value = h.clone();

        let s = serde_json::to_string(&h).unwrap();
        let u: Header = serde_json::from_str(&s).unwrap();
        let u_val = u.map.get("foo").unwrap();

        let v_val = test_value.map.get("foo").unwrap();

        assert_eq!(u_val.value[0], v_val.value[0]);
    }
}
