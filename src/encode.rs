pub static CRLF: &'static str = "\r\n";
pub static STRING_PREFIX: &'static str = "+";
pub static ERROR_PREFIX: &'static str = "-";
pub static INT_PREFIX: &'static str = ":";
pub static BULK_PREFIX: &'static str = "$";
pub static ARRAY_PREFIX: &'static str = "*";

pub fn encode_string(string: &String) -> String {
    STRING_PREFIX.to_string() + string + &CRLF
}

pub fn int32(int: i32) -> String {
    INT_PREFIX.to_string() + &int.to_string() + &CRLF
}

pub fn int64(int: i64) -> String {
    INT_PREFIX.to_string() + &int.to_string() + &CRLF
}

pub fn null() -> String {
    BULK_PREFIX.to_string() + "-1" + &CRLF
}

pub fn bulk_string(bulk: &str) -> String {
    BULK_PREFIX.to_string() + &bulk.len().to_string() + &CRLF + bulk + &CRLF
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string() {
        assert_eq!("+test\r\n", encode_string(&"test".to_string()));
    }
}
