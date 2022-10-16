# binum

binum (short for *binary numbers*) is a small library to work with binary (network) data in Rust.

## Usage

### Reading integers from byte slices

binum provides an easy way to read binary data (`u16`, `u32` and `u64`) from byte slices. To read a `u16` from a slice
of bytes we can simply use:

```rust
use binum;

let b = vec![69, 88, 65, 77, 80, 76, 69, 33];

match binum::BigEndian::read::<u16>(&b) {
    Ok(n) => assert_eq!(n, 17752),
    Err(err) => panic!("{}", err),
};
```

---

We can also read multiple integers at once. This is especially useful when dealing with multiple integers of the same
kind in a row:

```rust
use binum;

let b = vec![69, 88, 65, 77, 80, 76, 69, 33];

match binum::BigEndian::read_multi::<u16>(&b, 4) {
    Ok(n) => assert_eq!(n, vec![17752, 16717, 20556, 17697]),
    Err(err) => panic!("{}", err),
};
```

---

Usually we have to deal with byte slice offsets manually. In order to more easily handle offsets (and to automatically
advance them) we can use a reader (a cursor for example):

```rust
use binum;

let mut reader = Cursor::new(vec![69, 88, 65, 77, 80, 76, 69, 33]);

match reader.read_from::<u16, BigEndian>() {
    Ok(n) => assert_eq!(n, 17752),
    Err(err) => panic!("{}", err),
};
```

We can alos read multiple values at once:

```rust
use binum;

let mut reader = Cursor::new(vec![69, 88, 65, 77, 80, 76, 69, 33]);

match reader.read_multi::<u16, BigEndian>(2) {
    Ok(n) => assert_eq!(n, vec![17752, 16717]);
    Err(err) => panic!("{}", err),
};
```

### Writing integer values to byte slices

TODO

### `Readable` and `Writeable` derive macros