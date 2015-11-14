pub fn xor(v1: &Vec<u8>, v2: &Vec<u8>) -> Vec<u8> {
    v1.iter().zip(v2.iter()).map(|(a, b)| a ^ b).collect()
}

#[cfg(test)]
mod tests {
    extern crate rustc_serialize;

    use self::rustc_serialize::base64::{self, ToBase64};
    use self::rustc_serialize::hex::{FromHex, ToHex};

    use super::*;

    #[test]
    fn test_hex_to_base64() {
        // Set 1 Challenge 1
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(input.from_hex().unwrap().to_base64(base64::STANDARD), expected);
    }

    #[test]
    fn test_fixed_xor() {
        // Set 1 Challenge 2
        let input = "1c0111001f010100061a024b53535009181c";
        let key = "686974207468652062756c6c277320657965";
        let observed = xor(&input.from_hex().unwrap(), &key.from_hex().unwrap()).to_hex();
        let expected = "746865206b696420646f6e277420706c6179";
        assert_eq!(observed, expected);
    }
}
