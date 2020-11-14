use vardel;
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_encode_decode_u16(x in any::<u16>()) {
        let mut out: [u8; 16] = [0_u8; 16];
        let r = vardel::encode_u16(x, &mut out);
        assert!(r.is_ok());
        let n = r.unwrap();
        let r2 = vardel::decode_u16(&out[..n]);
        assert!(r2.is_ok());
        let (y, rest) = r2.unwrap();
        assert_eq!(y, x);
        assert!(rest.is_empty());
    }
}

proptest! {
    #[test]
    fn prop_encode_decode_u32(x in any::<u32>()) {
        let mut out: [u8; 16] = [0_u8; 16];
        let r = vardel::encode_u32(x, &mut out);
        assert!(r.is_ok());
        let n = r.unwrap();
        let r2 = vardel::decode_u32(&out[..n]);
        assert!(r2.is_ok());
        let (y, rest) = r2.unwrap();
        assert_eq!(y, x);
        assert!(rest.is_empty());
    }
}

proptest! {
    #[test]
    fn prop_encode_decode_u64(x in any::<u64>()) {
        let mut out: [u8; 16] = [0_u8; 16];
        let r = vardel::encode_u64(x, &mut out);
        assert!(r.is_ok());
        let n = r.unwrap();
        let r2 = vardel::decode_u64(&out[..n]);
        assert!(r2.is_ok());
        let (y, rest) = r2.unwrap();
        assert_eq!(y, x);
        assert!(rest.is_empty());
    }
}

proptest! {
    #[test]
    fn prop_encode_decode_u128(x in any::<u128>()) {
        let mut out: [u8; 32] = [0_u8; 32];
        let r = vardel::encode_u128(x, &mut out);
        assert!(r.is_ok());
        let n = r.unwrap();
        let r2 = vardel::decode_u128(&out[..n]);
        assert!(r2.is_ok());
        let (y, rest) = r2.unwrap();
        assert_eq!(y, x);
        assert!(rest.is_empty());
    }
}
