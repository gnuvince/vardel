//! delta-var-int libray
//!
//! We use macros to generate the encoder and decoder functions,
//! because that's much easier than trying to figure out all the right
//! traits to use.
//!
//! We don't provide encoders and decoders for u8: u8s are already one
//! byte, but it takes two bytes with var-ints to represent the values
//! from 128 to 255.

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
            let mut buf: [u8; 24] = [0; 24];
            let mut total_bytes_written: usize = 0;

            for x in xs {
                let n = $single_encoder(*x, &mut buf)?;
                w.write(&buf[..n])?;
                total_bytes_written += n;
            }

            return Ok(total_bytes_written);
        }
    };
}

macro_rules! make_decoder {
    ($func_name:ident, $ty:ty) => {
        /// Decodes a `$ty` from the `bytes` slice and returns the
        /// value and the part of the slice that wasn't consumed.
        ///
        /// Returns an error if a value can't be parsed from `bytes`.
        pub fn $func_name(bytes: &[u8]) -> Result<($ty, &[u8]), Error> {
            let mut x: $ty = 0;
            for (i, b) in bytes.iter().enumerate() {
                x |= ((*b & 0x7f) as $ty) << (7*i);
                if *b & 0x80 == 0 {
                    return Ok((x, &bytes[i+1..]));
                }
            }
            return Err(Error::InvalidVarInt);
        }
    };
}

macro_rules! make_slice_decoder {
    ($func_name:ident, $decoder:ident, $ty:ty) => {
        pub fn $func_name(mut bytes: &[u8]) -> Result<Vec<$ty>, Error> {
            let mut out: Vec<$ty> = Vec::new();
            while !bytes.is_empty() {
                let (x, rest) = $decoder(bytes)?;
                out.push(x);
                bytes = rest;
            }
            return Ok(out);
        }
    };
}

make_encoder!(encode_u16, u16);
make_encoder!(encode_u32, u32);
make_encoder!(encode_u64, u64);
make_encoder!(encode_u128, u128);

make_decoder!(decode_u16, u16);
make_decoder!(decode_u32, u32);
make_decoder!(decode_u64, u64);
make_decoder!(decode_u128, u128);

make_slice_encoder!(encode_u16_slice, encode_u16, u16);
make_slice_encoder!(encode_u32_slice, encode_u32, u32);
make_slice_encoder!(encode_u64_slice, encode_u64, u64);
make_slice_encoder!(encode_u128_slice, encode_u128, u128);

make_slice_decoder!(decode_u16_slice, decode_u16, u16);
make_slice_decoder!(decode_u32_slice, decode_u32, u32);
make_slice_decoder!(decode_u64_slice, decode_u64, u64);
make_slice_decoder!(decode_u128_slice, decode_u128, u128);


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
