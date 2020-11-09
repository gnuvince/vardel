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

make_encoder!(encode_u8, u8);
make_encoder!(encode_u16, u16);
make_encoder!(encode_u32, u32);
make_encoder!(encode_u64, u64);

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
