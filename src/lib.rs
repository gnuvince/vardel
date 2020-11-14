use std::fmt;
use std::error;
use std::io::{self, Write};

#[derive(Debug)]
pub enum Error {
    BufferTooSmall,
    InvalidVarInt,
    IoError(io::Error),
}

impl error::Error for Error {
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BufferTooSmall => write!(f, "output buffer is too small"),
            Error::InvalidVarInt => write!(f, "invalid var int: no terminator byte found"),
            Error::IoError(ref err) => write!(f, "io error: {}", err),
        }
    }
}

macro_rules! make_encoder {
    ($func_name:ident, $ty:ty) => {
        pub fn $func_name(mut x: $ty, out: &mut [u8]) -> Result<usize, Error> {
            for (i, byte) in out.iter_mut().enumerate() {
                // All bytes have MSB == 1...
                *byte = (x & 0x7f) as u8 | 0x80;
                x >>= 7;
                if x == 0 {
                    // ... except the last byte where MSB == 0.
                    *byte &= 0x7f;
                    return Ok(i+1);
                }
            }
            return Err(Error::BufferTooSmall);
        }
    };
}

macro_rules! make_slice_encoder {
    ($func_name:ident, $single_encoder:ident, $ty:ty) => {
        pub fn $func_name<W: Write>(xs: &[$ty], w: &mut W) -> Result<usize, Error> {
            let mut buf: [u8; 16] = [0; 16];
            let mut total_bytes_written: usize = 0;

            if xs.is_empty() {
                return Ok(total_bytes_written);
            }

            let mut prev = xs[0];
            let n = $single_encoder(prev, &mut buf)?;
            w.write(&buf[..n])?;
            total_bytes_written += n;

            for x in &xs[1..] {
                let delta = *x - prev;
                let n = $single_encoder(delta, &mut buf)?;
                w.write(&buf[..n])?;
                total_bytes_written += n;
                prev = *x;
            }

            return Ok(total_bytes_written);
        }
    };
}

pub fn decode_u32(bytes: &[u8]) -> Result<(u32, &[u8]), Error> {
    let mut x: u32 = 0;
    for (i, b) in bytes.iter().enumerate() {
        x |= ((*b & 0x7f) as u32) << (7*i);
        if *b & 0x80 == 0 {
            return Ok((x, &bytes[i+1..]));
        }
    }
    return Err(Error::InvalidVarInt);
}

pub fn contains_u32(target: u32, mut bytes: &[u8]) -> bool {
    let mut curr: u32 = 0;
    while let Ok((delta, rest)) = decode_u32(&bytes[..]) {
        curr += delta;
        if curr == target {
            return true;
        }
        bytes = rest;
    }
    return false;
}

make_encoder!(encode_u16, u16);
make_encoder!(encode_u32, u32);
make_encoder!(encode_u64, u64);

make_slice_encoder!(encode_u16_slice, encode_u16, u16);
make_slice_encoder!(encode_u32_slice, encode_u32, u32);
make_slice_encoder!(encode_u64_slice, encode_u64, u64);


#[test]
fn test_encode_fail() {
    let mut out: [u8; 1] = [0; 1];
    assert!(matches!(encode_u32(46, &mut out), Ok(1)));
    assert!(matches!(encode_u32(150, &mut out), Err(Error::BufferTooSmall)));
}

#[test]
fn test_encode_u16() {
    let mut out: [u8; 16] = [0; 16];

    for i in 0_u16 ..= 127_u16 {
        assert!(matches!(encode_u16(i, &mut out[..]), Ok(1)));
        assert_eq!(&[i as u8], &out[0..1]);
    }

    for i in 128_u16 ..= 16383_u16 {
        assert!(matches!(encode_u16(i, &mut out[..]), Ok(2)));
    }

    for i in 16384_u16 ..= 65535_u16 {
        assert!(matches!(encode_u16(i, &mut out[..]), Ok(3)));
    }
}

#[test]
fn test_encode_u32() {
    let mut out: [u8; 16] = [0; 16];

    for i in 0_u32 ..= 127_u32 {
        assert!(matches!(encode_u32(i, &mut out[..]), Ok(1)));
        assert_eq!(&[i as u8], &out[0..1]);
    }

    assert!(matches!(encode_u32(150, &mut out[..]), Ok(2)));
    assert_eq!(&[0x96, 0x01], &out[0..2]);

    assert!(matches!(encode_u32(300, &mut out[..]), Ok(2)));
    assert_eq!(&[0xac, 0x02], &out[0..2]);
}


#[test]
fn test_encode_u32_slice() {
    let mut w: Vec<u8> = Vec::new();
    let r = encode_u32_slice(&[], &mut w);
    assert!(r.is_ok());
    assert_eq!(0, r.unwrap());
    assert!(w.is_empty());

    let mut w: Vec<u8> = Vec::new();
    let r = encode_u32_slice(&[1, 2, 3], &mut w);
    assert!(r.is_ok());
    assert_eq!(3, r.unwrap());
    assert_eq!(vec![1,1,1], w);

    let mut w: Vec<u8> = Vec::new();
    let r = encode_u32_slice(&[5, 10, 160], &mut w);
    assert!(r.is_ok());
    assert_eq!(4, r.unwrap());
    assert_eq!(vec![5, 5, 0x96, 0x01], w);
}

#[test]
fn test_decode_u32() {
    let bytes = &[0x96, 0x01];
    if let Ok((x, rest)) = decode_u32(&bytes[..]) {
        assert_eq!(x, 150);
        assert!(rest.is_empty());
    }

    let bytes = &[0x96, 0x01, 0x97, 0x01];
    if let Ok((x, rest)) = decode_u32(&bytes[..]) {
        assert_eq!(x, 150);
        assert!(!rest.is_empty());
        if let Ok((x, rest)) = decode_u32(&rest[..]) {
            assert_eq!(x, 151);
            assert!(rest.is_empty());
        }
    }
}

#[test]
fn test_contains_u32() {
    let mut v: Vec<u8> = Vec::new();
    assert!(encode_u32_slice(&[4, 8, 15, 16, 23, 42], &mut v).is_ok());
    assert!(contains_u32(4, &v[..]));
    assert!(contains_u32(8, &v[..]));
    assert!(contains_u32(15, &v[..]));
    assert!(contains_u32(16, &v[..]));
    assert!(contains_u32(23, &v[..]));
    assert!(contains_u32(42, &v[..]));

    assert!(!contains_u32(5, &v[..]));
    assert!(!contains_u32(9, &v[..]));
    assert!(!contains_u32(17, &v[..]));
    assert!(!contains_u32(18, &v[..]));
    assert!(!contains_u32(24, &v[..]));
    assert!(!contains_u32(43, &v[..]));
}
