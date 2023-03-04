pub mod gen;

use crate::gen::{header, Header};
use prost::Message;

#[cfg(test)]
mod test {
    use crate::gen::{header, Header};

    #[test]
    fn test_header() {
        let mut h = Header::default();
        let v = header::Value {
            value: vec![String::from("value")],
        };
        h.map.insert(String::from("foo"), v);
    }
}
