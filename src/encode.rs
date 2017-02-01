use data::*;

pub fn encode(d: &Data) -> Bytes {
    let mut result: Bytes = Vec::new();

    match *d {
        Data::String(ref s) => {
            result.push(STRING_PREFIX);
            result.extend_from_slice(s.as_bytes());
            result.extend_from_slice(CRLF);
        }
        Data::Integer(ref i) => {
            result.push(INT_PREFIX);
            result.extend_from_slice(i.to_string().as_bytes());
            result.extend_from_slice(CRLF);
        }
        Data::Error(ref e) => {
            result.push(ERROR_PREFIX);
            result.extend_from_slice(e.as_bytes());
            result.extend_from_slice(CRLF);
        }
        Data::BulkString(ref bs) => {
            result.push(BULK_PREFIX);
            result.extend_from_slice(bs.len().to_string().as_bytes());
            result.extend_from_slice(CRLF);
            result.extend_from_slice(bs.as_bytes());
            result.extend_from_slice(CRLF);
        }
        Data::Array(ref a) => {
            result.push(ARRAY_PREFIX);
            result.extend_from_slice(a.len().to_string().as_bytes());
            result.extend_from_slice(CRLF);

            for e in a {
                result.extend(encode(e).iter().cloned())
            }
        }
        Data::Null => {
            result.push(BULK_PREFIX);
            result.extend_from_slice(b"-1");
            result.extend_from_slice(CRLF);
        }
        Data::NullArray => {
            result.push(ARRAY_PREFIX);
            result.extend_from_slice(b"-1");
            result.extend_from_slice(CRLF);
        }
    }

    result
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
