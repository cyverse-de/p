pub mod debuff;

pub trait Creator<T> {
    fn new() -> T;
}

#[cfg(test)]
mod test {
    use crate::debuff::{header, Header};
    use serde_json;

    #[test]
    fn test_header() {
        // Initialize a header.
        let mut h = Header::default();

        // Initialize a value and set it.
        let mut v = header::Value::default();
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
    fn test_
}
