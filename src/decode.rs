use data::*;

pub fn decode(b: &Bytes) -> Result<Data, &str> {
    decode_with_last_pos(b, 0).0
}

fn decode_with_last_pos<'a>(b: &Bytes, start: usize) -> (Result<Data, &'a str>, usize) {
    match b[start] {
        STRING_PREFIX => {
            match parse(b, start + 1) {
                Ok((s, i)) => (Ok(Data::String(String::from_utf8(s).unwrap())), i),
                Err(e) => (Err(e), 0),
            }
        }
        ERROR_PREFIX => {
            match parse(b, start + 1) {
                Ok((e, i)) => (Ok(Data::Error(String::from_utf8(e).unwrap())), i),
                Err(e) => (Err(e), 0),
            }
        }
        INT_PREFIX => {
            match parse(b, start + 1) {
                Ok((i, pos)) => {
                    (Ok(Data::Integer(String::from_utf8(i)
                         .unwrap()
                         .parse::<i64>()
                         .unwrap())),
                     pos)
                }
                Err(e) => (Err(e), 0),
            }
        }
        BULK_PREFIX => {
            match parse(b, start + 1) {
                Ok((ref bl, bulk_start_index)) => {
                    let bulk_len: usize = String::from_utf8(bl.to_vec())
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    let bulk_end_index: usize = bulk_start_index + bulk_len;
                    let bulk: Bytes = b[bulk_start_index..bulk_end_index].to_vec();

                    (Ok(Data::BulkString(String::from_utf8(bulk).unwrap())),
                     bulk_start_index + bulk_len + 1)
                }
                Err(e) => (Err(e), 0),
            }
        }
        ARRAY_PREFIX => {
            match parse(b, start + 1) {
                Ok((ref a, mut pos)) => {
                    let arr_len: usize = String::from_utf8(a.to_vec())
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    let mut result: Vec<Data> = Vec::new();

                    for _ in 0..arr_len {
                        println!("{}", 1);
                        let (res, i) = decode_with_last_pos(b, pos);
                        match res {
                            Ok(data) => {
                                result.push(data);
                                pos = i;
                            }
                            Err(e) => return (Err(e), 0),
                        }
                    }

                    (Ok(Data::Array(result)), pos)
                }
                Err(e) => (Err(e), 0),
            }
        }

        _ => (Err("Missing prefix"), 0),
    }
}

fn parse<'a>(b: &Bytes, start: usize) -> Result<(Bytes, usize), &'a str> {
    for i in start..b.len() - 1 {
        if b[i] == CRLF[0] && b[i + 1] == CRLF[1] {
            return Ok((b[start..i].to_vec(), i + 2));
        }
    }

    Err("Invalid bytes")
}

#[cfg(test)]
mod test {
    use super::*;
    use encode::encode;

    #[test]
    fn decode_string() {
        let encoded_string = encode(&Data::String("test".to_string()));
        assert_eq!(decode(&encoded_string).ok().unwrap(),
                   Data::String("test".to_string()));
    }

    #[test]
    fn decode_error() {
        let encoded_error = encode(&Data::Error("test".to_string()));
        assert_eq!(decode(&encoded_error).ok().unwrap(),
                   Data::Error("test".to_string()));
    }

    #[test]
    fn decode_int() {
        let encoded_int = encode(&Data::Integer(888));
        assert_eq!(decode(&encoded_int).ok().unwrap(), Data::Integer(888));
    }

    #[test]
    fn decode_bulk_string() {
        let encoded_bulk_string = encode(&Data::BulkString("test".to_string()));
        assert_eq!(decode(&encoded_bulk_string).ok().unwrap(),
                   Data::BulkString("test".to_string()));
    }

    #[test]
    fn decode_array() {
        let array = vec![Data::String("s1".to_string()),
                         Data::String("s2".to_string()),
                         Data::String("s3".to_string())];

        let data_array = Data::Array(array);

        let encoded_array = encode(&data_array);
        assert_eq!(decode(&encoded_array).ok().unwrap(), data_array);
    }
}
