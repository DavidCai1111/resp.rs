use data::*;

macro_rules! compose_encoded_res {
    ($res:ident, $prefix:ident, $bytes:expr) => (
        $res.push($prefix);
        $res.extend_from_slice($bytes);
        $res.extend_from_slice(CRLF);
    )
}

macro_rules! compose_encoded_bulk_res {
    ($res:ident, $prefix:ident, $bytes:ident) => (
        $res.push($prefix);
        $res.extend_from_slice($bytes.len().to_string().as_bytes());
        $res.extend_from_slice(CRLF);
        $res.extend_from_slice($bytes.as_bytes());
        $res.extend_from_slice(CRLF);
    )
}

pub fn encode(d: &Data) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    match *d {
        Data::String(ref s) => {
            compose_encoded_res!(res, STRING_PREFIX, s.as_bytes());
        }
        Data::Integer(ref i) => {
            compose_encoded_res!(res, INT_PREFIX, i.to_string().as_bytes());
        }
        Data::Error(ref e) => {
            compose_encoded_res!(res, ERROR_PREFIX, e.as_bytes());
        }
        Data::BulkString(ref bs) => {
            compose_encoded_bulk_res!(res, BULK_PREFIX, bs);
        }
        Data::Array(ref a) => {
            compose_encoded_res!(res, ARRAY_PREFIX, a.len().to_string().as_bytes());

            for e in a {
                res.extend(encode(e).iter().cloned())
            }
        }
        Data::Null => {
            compose_encoded_res!(res, BULK_PREFIX, b"-1");
        }
        Data::NullArray => {
            compose_encoded_res!(res, ARRAY_PREFIX, b"-1");
        }
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_string() {
        assert_eq!(String::from_utf8(encode(&Data::String("test".to_string()))).unwrap(),
                   "+test\r\n");
    }

    #[test]
    fn encode_int() {
        assert_eq!(String::from_utf8(encode(&Data::Integer(888))).unwrap(),
                   ":888\r\n");
    }

    #[test]
    fn encode_null() {
        assert_eq!(String::from_utf8(encode(&Data::Null)).unwrap(), "$-1\r\n");
    }

    #[test]
    fn encode_null_array() {
        assert_eq!(String::from_utf8(encode(&Data::NullArray)).unwrap(),
                   "*-1\r\n");
    }

    #[test]
    fn encode_bluk_string() {
        assert_eq!(String::from_utf8(encode(&Data::BulkString("test".to_string()))).unwrap(),
                   "$4\r\ntest\r\n");
    }

    #[test]
    fn encode_string_array() {
        let array = vec![Data::String("s1".to_string()),
                         Data::String("s2".to_string()),
                         Data::String("s3".to_string())];

        assert_eq!(String::from_utf8(encode(&Data::Array(array))).unwrap(),
                   "*3\r\n+s1\r\n+s2\r\n+s3\r\n");
    }
}
