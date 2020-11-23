vardel
======
  A simple library to encode/decode integers to var-ints.

Usage
-----

```rust
let mut out: [u8; 16] = [0_u8; 16];
let n = vardel::encode_u32(300, &mut out).unwrap();
assert_eq!(2, n);
assert_eq!(&[0xac, 0x02], &out[0..2]);

let (y, rest) = vardel::decode_u32(&out).unwrap()
assert_eq!(y, 300);
assert!(rest.is_empty())
```
