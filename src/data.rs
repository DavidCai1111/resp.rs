pub static CRLF: &'static str = "\r\n";
pub static STRING_PREFIX: &'static str = "+";
pub static ERROR_PREFIX: &'static str = "-";
pub static INT_PREFIX: &'static str = ":";
pub static BULK_PREFIX: &'static str = "$";
pub static ARRAY_PREFIX: &'static str = "*";


#[derive(Debug)]
pub enum Data {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(String),
    Array(Vec<Data>),
    Null,
    NullArray,
}
