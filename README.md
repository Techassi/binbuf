# binbuf

binbuf (short for *binary buffers*) is a small library to work with binary (network) data in Rust.

## TODOs

- Try to implement elegant and easy-to-use String packing and unpacking
- Re-add Read and Write derive for structs

## Usage

### Reading

```rust
let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
let mut b = ReadBuffer::new(b.as_slice());

match u16::read::<BigEndian>(&mut b) {
    Ok(n) => assert_eq!(n, 17752),
    Err(err) => panic!("{}", err),
}
```

### Writing

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
