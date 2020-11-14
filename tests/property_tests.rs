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
    fn prop_encode_decode_u16_slice(x in any::<Vec<u16>>()) {
        let mut out: Vec<u8> = Vec::new();
        let r = vardel::encode_u16_slice(&x[..], &mut out);
        assert!(r.is_ok());
        let r2 = vardel::decode_u16_slice(&out);
        assert!(r2.is_ok());
        let y = r2.unwrap();
        assert_eq!(y, x);
    }
}

proptest! {
    #[test]
    fn prop_delta_encode_decode_u16_slice(mut x in any::<Vec<u16>>()) {
        x.sort_unstable();
        let mut out: Vec<u8> = Vec::new();
        let r = vardel::delta_encode_u16_slice(&x[..], &mut out);
        assert!(r.is_ok());
        let r2 = vardel::delta_decode_u16_slice(&out);
        assert!(r2.is_ok());
        let y = r2.unwrap();
        assert_eq!(y, x);
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
    fn prop_encode_decode_u32_slice(x in any::<Vec<u32>>()) {
        let mut out: Vec<u8> = Vec::new();
        let r = vardel::encode_u32_slice(&x[..], &mut out);
        assert!(r.is_ok());
        let r2 = vardel::decode_u32_slice(&out);
        assert!(r2.is_ok());
        let y = r2.unwrap();
        assert_eq!(y, x);
    }
}

proptest! {
    #[test]
    fn prop_delta_encode_decode_u32_slice(mut x in any::<Vec<u32>>()) {
        x.sort_unstable();
        let mut out: Vec<u8> = Vec::new();
        let r = vardel::delta_encode_u32_slice(&x[..], &mut out);
        assert!(r.is_ok());
        let r2 = vardel::delta_decode_u32_slice(&out);
        assert!(r2.is_ok());
        let y = r2.unwrap();
        assert_eq!(y, x);
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
    fn prop_encode_decode_u64_slice(x in any::<Vec<u64>>()) {
        let mut out: Vec<u8> = Vec::new();
        let r = vardel::encode_u64_slice(&x[..], &mut out);
        assert!(r.is_ok());
        let r2 = vardel::decode_u64_slice(&out);
        assert!(r2.is_ok());
        let y = r2.unwrap();
        assert_eq!(y, x);
    }
}

proptest! {
    #[test]
    fn prop_delta_encode_decode_u64_slice(mut x in any::<Vec<u64>>()) {
        x.sort_unstable();
        let mut out: Vec<u8> = Vec::new();
        let r = vardel::delta_encode_u64_slice(&x[..], &mut out);
        assert!(r.is_ok());
        let r2 = vardel::delta_decode_u64_slice(&out);
        assert!(r2.is_ok());
        let y = r2.unwrap();
        assert_eq!(y, x);
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

proptest! {
    #[test]
    fn prop_encode_decode_u128_slice(x in any::<Vec<u128>>()) {
        let mut out: Vec<u8> = Vec::new();
        let r = vardel::encode_u128_slice(&x[..], &mut out);
        assert!(r.is_ok());
        let r2 = vardel::decode_u128_slice(&out);
        assert!(r2.is_ok());
        let y = r2.unwrap();
        assert_eq!(y, x);
    }
}

proptest! {
    #[test]
    fn prop_delta_encode_decode_u128_slice(mut x in any::<Vec<u128>>()) {
        x.sort_unstable();
        let mut out: Vec<u8> = Vec::new();
        let r = vardel::delta_encode_u128_slice(&x[..], &mut out);
        assert!(r.is_ok());
        let r2 = vardel::delta_decode_u128_slice(&out);
        assert!(r2.is_ok());
        let y = r2.unwrap();
        assert_eq!(y, x);
    }
}
