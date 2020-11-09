macro_rules! make_encoder {
    ($func_name:ident, $ty:ty) => {
        pub fn $func_name(mut x: $ty, out: &mut [u8]) -> Option<usize> {
            for (i, byte) in out.iter_mut().enumerate() {
                // All bytes have MSB == 1...
                *byte = (x & 0x7f) as u8 | 0x80;
                x >>= 7;
                if x == 0 {
                    // ... except the last byte where MSB == 0.
                    *byte &= 0x7f;
                    return Some(i+1);
                }
            }
            return None;
        }
    };
}

macro_rules! make_slice_encoder {
    ($func_name:ident, $single_encoder:ident, $ty:ty) => {
        pub fn $func_name(xs: &[$ty]) -> Option<Vec<u8>> {
            let mut out: Vec<u8> = Vec::new();
            let mut buf: [u8; 16] = [0; 16];

            if xs.is_empty() {
                return Some(out);
            }

            let mut prev = xs[0];
            let n = $single_encoder(prev, &mut buf)?;
            out.extend_from_slice(&buf[..n]);

            for x in &xs[1..] {
                let delta = *x - prev;
                let n = $single_encoder(delta, &mut buf)?;
                out.extend_from_slice(&buf[..n]);
                prev = *x;
            }

            return Some(out);
        }
    };
}

make_encoder!(encode_u8, u8);
make_encoder!(encode_u16, u16);
make_encoder!(encode_u32, u32);
make_encoder!(encode_u64, u64);

make_slice_encoder!(encode_u8_slice, encode_u8, u8);
make_slice_encoder!(encode_u16_slice, encode_u16, u16);
make_slice_encoder!(encode_u32_slice, encode_u32, u32);
make_slice_encoder!(encode_u64_slice, encode_u64, u64);

#[test]
fn test_encode_u8() {
    let mut out: [u8; 16] = [0; 16];

    for i in 0_u8 ..= 127_u8 {
        assert_eq!(Some(1), encode_u8(i, &mut out[..]));
        assert_eq!(&[i], &out[0..1]);
    }

    for i in 128_u8 ..= 255_u8 {
        assert_eq!(Some(2), encode_u8(i, &mut out[..]));
        assert_eq!(&[i | (1 << 7), 1], &out[0..2]);
    }
}

#[test]
fn test_encode_fail() {
    let mut out: [u8; 1] = [0; 1];
    assert_eq!(Some(1), encode_u8(46, &mut out));
    assert_eq!(None, encode_u8(150, &mut out));
}

#[test]
fn test_encode_u16() {
    let mut out: [u8; 16] = [0; 16];

    for i in 0_u16 ..= 127_u16 {
        assert_eq!(Some(1), encode_u16(i, &mut out[..]));
        assert_eq!(&[i as u8], &out[0..1]);
    }

    for i in 128_u16 ..= 16383_u16 {
        assert_eq!(Some(2), encode_u16(i, &mut out[..]));
    }

    for i in 16384_u16 ..= 65535_u16 {
        assert_eq!(Some(3), encode_u16(i, &mut out[..]));
    }
}

#[test]
fn test_encode_u32() {
    let mut out: [u8; 16] = [0; 16];

    for i in 0_u32 ..= 127_u32 {
        assert_eq!(Some(1), encode_u32(i, &mut out[..]));
        assert_eq!(&[i as u8], &out[0..1]);
    }

    assert_eq!(Some(2), encode_u32(150, &mut out[..]));
    assert_eq!(&[0x96, 0x01], &out[0..2]);

    assert_eq!(Some(2), encode_u32(300, &mut out[..]));
    assert_eq!(&[0xac, 0x02], &out[0..2]);
}


#[test]
fn test_encode_u8_slice() {
    let r = encode_u8_slice(&[]);
    assert!(r.is_some());
    assert!(r.unwrap().is_empty());

    let r = encode_u8_slice(&[1, 2, 3]);
    assert!(r.is_some());
    assert_eq!(vec![1, 1, 1], r.unwrap());

    let r = encode_u8_slice(&[5, 10, 160]);
    assert!(r.is_some());
    assert_eq!(vec![5, 5, 0x96, 0x01], r.unwrap());
}

#[test]
fn test_encode_u32_slice() {
    let r = encode_u32_slice(&[150, 300]);
    assert!(r.is_some());
    assert_eq!(vec![0x96, 0x01, 0x96, 0x01], r.unwrap());
}
