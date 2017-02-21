# resp.rs
[![Build Status](https://travis-ci.org/DavidCai1993/resp.rs.svg?branch=master)](https://travis-ci.org/DavidCai1993/resp.rs)

A RESP (REdis Serialization Protocol) parser for Rust.

## Usage

```rust
extern crate resp;

use resp::encode;
use resp::Data;
use resp::decode;

fn main() {
    let encoded_string = encode(&Data::String("test".to_string()));
    assert_eq!(String::from_utf8(encoded_string).unwrap(), "+test\r\n");

    let array = Data::Array(vec![Data::String("s1".to_string()),
                                 Data::String("s2".to_string()),
                                 Data::String("s3".to_string())]);

    assert_eq!(String::from_utf8(encode(&array)).unwrap(),
               "*3\r\n+s1\r\n+s2\r\n+s3\r\n");
    assert_eq!(decode(&encode(&array)).ok().unwrap(), array);
}
```

## API

### `pub fn encode(data: &Data) -> Vec<u8>`

Encode given data to RESP buffer.

### `pub fn decode(bytes: &Vec<u8>) -> Result<Data, &str>`

Decode the RESP buffer to real data.
