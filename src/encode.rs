use data::*;

pub fn encode(d: Data) -> String {
    match d {
        Data::SimpleString(s) => STRING_PREFIX.to_string() + &s + CRLF,
        Data::Integer(i) => INT_PREFIX.to_string() + &i.to_string() + CRLF,
        Data::Null => BULK_PREFIX.to_string() + "-1" + CRLF,
        Data::NullArray => ARRAY_PREFIX.to_string() + "-1" + CRLF,
        Data::BulkString(b) => BULK_PREFIX.to_string() + &b.len().to_string() + CRLF + &b + CRLF,
        Data::Error(e) => ERROR_PREFIX.to_string() + &e + CRLF,
        Data::Array(a) => {
            let mut result = String::from(ARRAY_PREFIX.to_string() + &a.len().to_string() + CRLF);

            for e in a {
                result += &encode(e)
            }

            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_string() {
        assert_eq!("+test\r\n", encode(Data::SimpleString("test".to_string())));
    }

    #[test]
    fn encode_int() {
        assert_eq!(":888\r\n", encode(Data::Integer(888)));
    }

    #[test]
    fn encode_null() {
        assert_eq!("$-1\r\n", encode(Data::Null));
    }

    #[test]
    fn encode_null_array() {
        assert_eq!("*-1\r\n", encode(Data::NullArray));
    }

    #[test]
    fn encode_bluk_string() {
        assert_eq!("$4\r\ntest\r\n",
                   encode(Data::BulkString("test".to_string())));
    }

    #[test]
    fn encode_string_array() {
        let array = vec![Data::SimpleString("s1".to_string()),
                         Data::SimpleString("s2".to_string()),
                         Data::SimpleString("s3".to_string())];
        assert_eq!("*3\r\n+s1\r\n+s2\r\n+s3\r\n", encode(Data::Array(array)));
    }
}
