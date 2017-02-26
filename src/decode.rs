use std::fmt;
use std::result;
use data::*;

pub type Result<T> = result::Result<T, DecodeError>;

#[derive(Debug)]
pub enum DecodeError {
    MissingPrefix,
    InvalidBytes,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecodeError::MissingPrefix => write!(f, "Missing prefix"),
            DecodeError::InvalidBytes => write!(f, "Invalid bytes"),
        }
    }
}

pub fn decode(b: &[u8]) -> Result<Data> {
    decode_with_last_pos(b, 0).map(|decoded| decoded.data)
}

fn decode_with_last_pos(b: &[u8], start: usize) -> Result<Decoded> {
    match b[start] {
        b'+' => {
            let (s, i) = try!(parse(b, start + 1));

            Ok(Decoded {
                data: Data::String(String::from_utf8(s).unwrap()),
                pos: i,
            })
        }
        b'-' => {
            let (e, i) = try!(parse(b, start + 1));

            Ok(Decoded {
                data: Data::Error(String::from_utf8(e).unwrap()),
                pos: i,
            })
        }
        b':' => {
            let (i, pos) = try!(parse(b, start + 1));

            Ok(Decoded {
                data: Data::Integer(String::from_utf8(i)
                    .unwrap()
                    .parse::<i64>()
                    .unwrap()),
                pos: pos,
            })
        }
        b'$' => {
            let (bl, bulk_start_index) = try!(parse(b, start + 1));

            let bulk_len: usize = String::from_utf8(bl.to_vec())
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let bulk_end_index: usize = bulk_start_index + bulk_len;
            let bulk: Vec<u8> = b[bulk_start_index..bulk_end_index].to_vec();

            Ok(Decoded {
                data: Data::BulkString(String::from_utf8(bulk).unwrap()),
                pos: bulk_start_index + bulk_len + 1,
            })
        }
        b'*' => {
            let (a, mut pos) = try!(parse(b, start + 1));

            let arr_len: usize = String::from_utf8(a.to_vec())
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let mut result: Vec<Data> = Vec::new();

            for _ in 0..arr_len {
                let data = try!(decode_with_last_pos(b, pos));
                result.push(data.data);
                pos = data.pos;
            }

            Ok(Decoded {
                data: Data::Array(result),
                pos: pos,
            })
        }
        _ => Err(DecodeError::InvalidBytes),
    }
}

struct Decoded {
    data: Data,
    pos: usize,
}

fn parse(b: &[u8], start: usize) -> Result<(Vec<u8>, usize)> {
    for i in start..b.len() - 1 {
        if b[i] == b'\r' && b[i + 1] == b'\n' {
            return Ok((b[start..i].to_vec(), i + 2));
        }
    }

    Err(DecodeError::InvalidBytes)
}

#[cfg(test)]
mod tests {
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
