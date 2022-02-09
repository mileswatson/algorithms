pub mod decoding;
pub mod encoding;

#[cfg(test)]
mod test {
    use super::decoding::Encoded::Huffman;
    use super::encoding::encode;
    use rand::random;
    use std::iter::repeat_with;

    #[test]
    fn bytes() {
        for _ in 0..50 {
            let bytes: Vec<u8> = repeat_with(random).take(1000).collect();
            let compressed = encode(&bytes);
            let decompressed = compressed.decode().unwrap();
            assert_eq!(bytes, decompressed);
            if let Huffman(v, ..) = compressed {
                assert!(v.len() <= bytes.len());
            }
        }
    }
}
