# binum

binum (short for *binary numbers*) is a small library to work with binary (network) data in Rust.

## Usage

### Reading integers from byte slices

binum provides an easy way to read binary data (`u16`, `u32` and `u64`) from byte slices. To read a `u16` from a slice
of bytes we can simply use:

```rust
use binum::prelude::*;

let buf = vec![69, 88, 65, 77, 80, 76, 69, 33];

match binum::read::<u16, BigEndian>(&buf) {
    Ok(n) => assert_eq!(n, 17752),
    Err(err) => panic!("{}", err),
};
```

---

We can also read multiple integers at once. This is especially useful when dealing with multiple integers of the same
kind in a row:

```rust
use binum::prelude::*;

let buf = vec![69, 88, 65, 77, 80, 76, 69, 33];

match binum::read_multi::<u16, BigEndian>(&buf, 4) {
    Ok(n) => assert_eq!(n, vec![17752, 16717, 20556, 17697]),
    Err(err) => panic!("{}", err),
};
```

---

Usually we have to deal with byte slice offsets manually. In order to more easily handle offsets (and to automatically
advance them) we can use a reader (a cursor for example) and the provided `ReadExt` trait:

```rust
use binum::prelude::*;

let mut reader = Cursor::new(vec![69, 88, 65, 77, 80, 76, 69, 33]);

match reader.read_from::<u16, BigEndian>() {
    Ok(n) => assert_eq!(n, 17752),
    Err(err) => panic!("{}", err),
};
```

We can also read multiple values at once:

```rust
use binum::prelude::*;

let mut reader = Cursor::new(vec![69, 88, 65, 77, 80, 76, 69, 33]);

match reader.read_multi::<u16, BigEndian>(2) {
    Ok(n) => assert_eq!(n, vec![17752, 16717]);
    Err(err) => panic!("{}", err),
};
```

### Writing integer values to byte slices

To write a `u16` into a byte buffer we can simply use:

```rust
use binum::prelude::*;

let mut buf = vec![0; 8];

match binum::write::<u16, BigEndian>(17752, &mut buf) {
    Ok(n) => assert_eq!(n, 2),
    Err(err) => panic!("{}", err),
};
```

---

We can also write multiple integers at once:

```rust
use binum::prelude::*;

let mut buf = vec![0; 8];

match binum::write_multi::<u16, BigEndian>(vec![17752, 16717], &mut buf) {
    Ok(n) => assert_eq!(n, 4),
    Err(err) => panic!("{}", err),
};
```

---

The `WriteExt` trait provides access to write methods for types which implement the `io::Write` trait. This enables
the automatic advancement of buffer offsets:

```rust
use binum::prelude::*;

let mut writer: Vec<u8> = Vec::new();

match writer.write_into::<u16, BigEndian>(17752) {
    Ok(n) => {
        assert_eq!(n, 2);
        assert_eq!(writer.len(), 2);
        assert_eq!(writer, vec![69, 88]);
    }
    Err(err) => panic!("{}", err),
}
```

It is also possible to write multiple integers at once:

```rust
use binum::prelude::*;

let mut writer: Vec<u8> = Vec::new();

match writer.write_multi::<u16, BigEndian>(vec![17752, 16717]) {
    Ok(n) => {
        assert_eq!(n, 4);
        assert_eq!(writer.len(), 4);
        assert_eq!(writer, vec![69, 88, 65, 77]);
    }
    Err(err) => panic!("{}", err),
}
```

### `Readable` and `Writeable` derive macros

The `Readable` and `Writeable` derive macros help to automatically read (network) data from byte buffers into supported
structs and write data from structs into a buffer.

#### `Readable`

```rust
#[derive(Readable)]
pub struct Target {
    pub a: u16,
    pub b: u16,
    pub c: u32,
}

let buf = vec![69, 88, 65, 77, 80, 76, 69, 33];

match Target::read_from::<BigEndian>(&buf) {
    Ok(t) => {
        assert_eq!(t.a, 17752);
        assert_eq!(t.b, 16717);
        assert_eq!(t.c, 1347175713);
    }
    Err(err) => panic!("{}", err),
};
```

#### `Writeable`

```rust
#[derive(Writeable)]
pub struct Source {
    pub a: u16,
    pub b: u16,
    pub c: u32,
}

let mut buf = vec![0; 8];
let source = Source {
    a: 17752,
    b: 16717,
    c: 1347175713,
};

match source.write_into::<BigEndian>(&mut buf) {
    Ok(n) => {
        assert_eq!(n, 8);
        assert_eq!(b, vec![69, 88, 65, 77, 80, 76, 69, 33]);
    }
    Err(err) => panic!("{}", err),
}
```