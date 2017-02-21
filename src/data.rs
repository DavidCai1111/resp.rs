// Common constants used in RESP.
pub const CRLF: &'static [u8; 2] = b"\r\n";
pub const STRING_PREFIX: u8 = b'+';
pub const ERROR_PREFIX: u8 = b'-';
pub const INT_PREFIX: u8 = b':';
pub const BULK_PREFIX: u8 = b'$';
pub const ARRAY_PREFIX: u8 = b'*';


#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    String(String),
    Error(String),
    Integer(i64),
    BulkString(String),
    Array(Vec<Data>),
    Null,
    NullArray,
}
