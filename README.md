# binbuf

binbuf (short for *binary buffers*) is a small library to work with binary (network) data in Rust. Just add
`binbuf::prelude::*` to your imports. This imports the most important parts of the library.

## Reading from `Reader`

### Reading basic types

The library provides multiple methods to read basic data types like `u8`, `u16`, `u32`, `u64`, `u128`, `usize`,
`Ipv4Addr`, and `Ipv6Addr` in big and little-endian byte order.

```rust
let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
let mut b = Reader::new(b.as_slice());

match u16::read::<BigEndian>(&mut b) {
    Ok(n) => assert_eq!(n, 17752),
    Err(err) => panic!("{}", err),
}
```

### Reading structs and enums

To read custom data structs or enums, we can use the derive macro `#[derive(Read)]` to annotate the structs.

```rust
#[derive(Read)]
struct Data {
    inner: u16,
}

let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
let mut buf = Reader::new(b.as_slice());

match Data::read::<BigEndian>(&mut buf) {
    Ok(data) => assert_eq!(data.inner, 17752),
    Err(err) => panic!("{}", err)
}
```

Customize the derive macro by annotating the struct with additional attributes: `#[binbuf()]`. Currently, the following
container attributes are supported:

- `#[binbuf(error = "...")]`

  > Default value: `binbuf::error::BufferError`

  Provide a custom error. The error has to implement these traits:

  - `std::fmt::Display`
  - `std::error::Error`
  - `From<BufferError>`

- `#[binbuf(endianness = "...")]`

  > Default value: `both`

  Specify the supported endianness for the `ReadableVerify` trait. Possible values are:

  - `little`
  - `both`
  - `big`

Enums can be tagged with one additional attribute:

- `#[binbuf(repr = "...")]`

  > Default value: `u8`

The library works well with the `thiserror` crate. Implementing custom errors with the `Error` derive macro is
straightforward:

```rust
use thiserror::Error;

#[derive(Error)]
enum CustomError {
    #[error("Invalid data")]
    Invalid,

    #[error("Buffer error: {0}")]
    BufferError(#[from] BufferError)
}
```
