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
