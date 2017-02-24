// Common constants used in RESP.
pub const CRLF: &'static str = "\r\n";
pub const STRING_PREFIX: &'static str = "+";
pub const ERROR_PREFIX: &'static str = "-";
pub const INT_PREFIX: &'static str = ":";
pub const BULK_PREFIX: &'static str = "$";
pub const ARRAY_PREFIX: &'static str = "*";

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
