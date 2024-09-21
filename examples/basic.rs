use binbuf::{Read as _, Reader, Write as _, Writer};

fn main() {
    let answer: u64 = 42;

    let mut writer = Writer::new();
    let _bytes_written = answer.write_be(&mut writer).expect("can write");

    let mut reader = Reader::new(writer.bytes());
    let meaning_if_life = u64::read_be(&mut reader).expect("can read");

    assert_eq!(meaning_if_life, answer);
}
