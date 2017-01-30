const CRLF: str = "\r\n";
const STRING_PREFIX: str = "+";
const ERROR_PREFIX: str = "-";
const INT_PREFIX: str = ":";
const BULK_PREFIX: str = "$";
const ARRAY_PREFIX: str = "*";

fn encode_string(string: &str) -> &str {
    STRING_PREFIX + string + CRLF
}

fn encode_error(error: std::error::Error) -> &str {
    ERROR_PREFIX + error.description() + CRLF
}

fn encode_int32(int: i32) -> &str {
    INT_PREFIX + int + CRLF
}

fn encode_int64(int: i64) -> &str {
    INT_PREFIX + int + CRLF
}

fn encode_null() -> &str {
    BULK_PREFIX + "-1" + CRLF
}

fn encode_bulk_string(bulk: &str) -> &str {
    BULK_PREFIX + bulk.len() + CRLF + bulk + CRLF
}

fn encode_null_array(array: Vec<str>) -> &str {
    let encoded: str = ARRAY_PREFIX + array.len() + CRLF;

    for s in array {
        encoded += s;
    }

    encoded
}
