use data::*;

pub fn decode_with_pos<'a>(b: &Bytes, start: usize) -> Result<Data, &'a str> {
    match b[start] {
        STRING_PREFIX => {
            match parse(b, start + 1) {
                Ok((s, ..)) => Ok(Data::String(String::from_utf8(s).unwrap())),
                Err(e) => Err(e),
            }
        }
        ERROR_PREFIX => {
            match parse(b, start + 1) {
                Ok((e, ..)) => Ok(Data::Error(String::from_utf8(e).unwrap())),
                Err(e) => Err(e),
            }
        }
        INT_PREFIX => {
            match parse(b, start + 1) {
                Ok((i, ..)) => {
                    Ok(Data::Integer(String::from_utf8(i)
                        .unwrap()
                        .parse::<i64>()
                        .unwrap()))
                }
                Err(e) => Err(e),
            }
        }
        BULK_PREFIX => {
            match parse(b, start + 1) {
                Ok((ref b, bulk_start_index)) => {
                    let bulk_len: usize = String::from_utf8(b.to_vec())
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    let bulk_end_index: usize = bulk_start_index + bulk_len;
                    let bulk: Bytes = b[bulk_start_index..bulk_end_index].to_vec();

                    Ok(Data::BulkString(String::from_utf8(bulk).unwrap()))
                }
                Err(e) => Err(e),
            }
        }
        ARRAY_PREFIX => {
            match parse(b, start + 1) {
                Ok((ref a, pos)) => {
                    let arr_len: usize = String::from_utf8(a.to_vec())
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();

                    let mut result: Vec<Data> = Vec::new();

                    for i in 0..arr_len {
                        match decode_with_pos(b, pos) {
                            Ok(data) => {
                                result[i] = data;
                            }
                            Err(e) => return Err(e),
                        }
                    }

                    Ok(Data::Array(result))
                }
                Err(e) => Err(e),
            }
        }

        _ => Err("Missing prefix"),
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
