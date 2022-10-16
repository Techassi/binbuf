use binum::{self, BigEndian, Readable, Writeable};

#[test]
fn test_derive_readable() {
    #[derive(Readable)]
    pub struct Target {
        pub a: u16,
        pub b: u16,
        pub c: u32,
    }

    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];

    match Target::read_from::<BigEndian>(&b) {
        Ok(t) => {
            assert_eq!(t.a, 17752);
            assert_eq!(t.b, 16717);
            assert_eq!(t.c, 1347175713);
        }
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_derive_writeable() {
    #[derive(Writeable)]
    pub struct Source {
        pub a: u16,
        pub b: u16,
        pub c: u32,
    }

    let mut b = vec![0; 8];
    let s = Source {
        a: 17752,
        b: 16717,
        c: 1347175713,
    };

    match s.write_into::<BigEndian>(&mut b) {
        Ok(n) => {
            assert_eq!(n, 8);
            assert_eq!(b, vec![69, 88, 65, 77, 80, 76, 69, 33]);
        }
        Err(err) => panic!("{}", err),
    }
}
