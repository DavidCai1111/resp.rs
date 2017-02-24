use std::convert::From;
use data::*;

pub fn encode(d: &Data) -> Vec<u8> {
    match *d {
        Data::String(ref s) => From::from(STRING_PREFIX.to_string() + &s + &CRLF),
        Data::Integer(ref i) => From::from(INT_PREFIX.to_string() + &i.to_string() + &CRLF),
        Data::Error(ref e) => From::from(ERROR_PREFIX.to_string() + &e + &CRLF),
        Data::Null => From::from(BULK_PREFIX.to_string() + "-1" + &CRLF),
        Data::NullArray => From::from(ARRAY_PREFIX.to_string() + "-1" + &CRLF),
        Data::BulkString(ref bs) => {
            From::from(BULK_PREFIX.to_string() + &bs.len().to_string() + &CRLF + &bs + &CRLF)
        }
        Data::Array(ref a) => {
            let mut result: Vec<u8> = From::from(ARRAY_PREFIX.to_string() + &a.len().to_string() +
                                                 &CRLF);

            for e in a {
                result.extend(encode(e).iter().cloned())
            }

            result
        }
    }
}

#[cfg(test)]
mod tests {
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
