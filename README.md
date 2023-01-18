# binbuf

binbuf (short for *binary buffers*) is a small library to work with binary (network) data in Rust.

## TODOs

- Try to implement elegant and easy-to-use String reading and writing
- Re-add Read and Write derive for structs

## Usage

Just add `use binbuf::prelude::*` to your imports. This imports the most important parts of the library.

### Reading basics

```rust
let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
let mut b = ReadBuffer::new(b.as_slice());

match u16::read::<BigEndian>(&mut b) {
    Ok(n) => assert_eq!(n, 17752),
    Err(err) => panic!("{}", err),
}
```

### Reading into custom data structures

To read custom data structures we need to implement the `Readable` trait for our type. Currently this is a manual process, but soon users will be able to derive it with `#[derive(Readable)]`.

```rust
struct Data {
    inner: u16,
}

impl Readable for Data {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut impl ToReadBuffer) -> Result<Self, Self::Error> {
        let inner = u16::read::<E>(buf)?;

        Ok(Self { inner })
    }
}

let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
let mut buf = ReadBuffer::new(b.as_slice());

let data = Data::read::<BigEndian>(&mut buf).unwrap();
assert_eq!(data.inner, 17752)
```

---

### Writing basics

```rust
let mut b = WriteBuffer::new();

match 17752u16.write::<BigEndian>(&mut b) {
    Ok(n) => {
        assert_eq!(n, 2);
        assert_eq!(b.bytes(), &[69, 88]);
    }
    Err(err) => panic!("{}", err),
}
```

### Writing custom data structures

Writing custom data structures is as straight forward as reading them. This currectly is also a manual process, but will
also be replaced by `#[derive(Writeable)]`.

```rust
struct Data {
    inner: u16,
}

impl Writeable for Data {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut impl ToWriteBuffer) -> Result<usize, Self::Error> {
        self.inner.write::<E>(buf)
    }
}

let d = Data { inner: 17752 };

let mut b = WriteBuffer::new();
match d.write::<BigEndian>(&mut b) {
    Ok(n) => {
        assert_eq!(n, 2);
        assert_eq!(b.bytes(), &[69, 88]);
    }
    Err(err) => panic!("{}", err),
};
```
