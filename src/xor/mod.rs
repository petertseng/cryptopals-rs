mod test_data;

use edit_distance::hamming;
use frequency::{ENGLISH_LETTERS, ENGLISH_FREQS, lower_english_only, letter_frequencies};
use std::cmp;
use std::f64;

pub fn crack_vigenere(s: &[u8]) -> Option<Vec<u8>> {
    let max_try = cmp::min(s.len() / 2, 40);

    let mut length_scores: Vec<_> = (2..(max_try + 1)).map(|i| {
        let first = &s[0..i];
        let second = &s[i..(2 * i)];
        let dist = hamming(first, second);
        let normalized = (dist as f64) / (i as f64);
        (i, normalized)
    }).collect();

    length_scores.sort_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap());

    for &(length, _) in length_scores.iter() {
        let blocks = chunks(s, length);
        let mut key = Vec::new();
        for b in blocks {
            let (k, v, _, _) = crack(&b, true);
            if v.is_empty() {
                break;
            }
            key.push(k);
        }

        if key.len() == length {
            return Some(key);
        }
    }

    None
}

fn chunks(s: &[u8], n: usize) -> Vec<Vec<u8>> {
    let mut chunks = vec![Vec::new(); n];
    for (i, &c) in s.iter().enumerate() {
        chunks[i % n].push(c);
    }
    chunks
}

pub fn detect(ss: &Vec<Vec<u8>>, must_be_printable: bool) -> (usize, &Vec<u8>, u8, Vec<u8>, String, f64) {
    let mut best_index = 0;
    let mut best_input = &ss[0];
    let mut best_byte = 0;
    let mut best_vec = Vec::new();
    let mut best_string = String::new();
    let mut best_score = f64::INFINITY;

    for (i, s) in ss.iter().enumerate() {
        let (byte, vec, string, score) = crack(s, must_be_printable);
        if score < best_score {
            best_index = i;
            best_input = s;
            best_byte = byte;
            best_vec = vec;
            best_string = string;
            best_score = score;
        }
    }

    (best_index, best_input, best_byte, best_vec, best_string, best_score)
}

pub fn crack(s: &Vec<u8>, must_be_printable: bool) -> (u8, Vec<u8>, String, f64) {
    let mut best_byte = 0;
    let mut best_vec = Vec::new();
    let mut best_string = String::new();
    let mut best_score = f64::INFINITY;
    let max: u16 = 256;

    for i in 0..max {
        let byte = i as u8;
        let candidate = xor_single(s, byte);
        let candidate_string = match String::from_utf8(candidate.to_vec()) {
            Ok(s) => s,
            Err(_) => continue,
        };

        if must_be_printable && !is_printable(&candidate_string) {
            continue;
        }

        let normal = lower_english_only(&candidate_string);
        let (count, freqs) = letter_frequencies(&normal);
        let score = ENGLISH_LETTERS.chars().zip(ENGLISH_FREQS).fold(0.0, |score, (letter, eng_freq)| {
            let letter_score = match freqs.get(&letter) {
                Some(freq) => (*freq as f64) / (count as f64),
                None => 0.0,
            };
            score + (eng_freq - letter_score).abs()
        });

        if score < best_score {
            best_byte = byte;
            best_vec = candidate;
            best_string = candidate_string;
            best_score = score;
        }
    }

    (best_byte, best_vec, best_string, best_score)
}

fn is_printable(s: &String) -> bool {
    s.as_bytes().iter().all(|&c| c == 0x0a || 0x20 <= c && c <= 0x7f)
}

// TODO: Is it so strange that it takes different types?
pub fn xor_repeating(v: &[u8], k: &[u8]) -> Vec<u8> {
    let klen = k.len();
    v.iter().enumerate().map(|(i, c)| c ^ k[i % klen]).collect()
}

pub fn xor(v1: &Vec<u8>, v2: &Vec<u8>) -> Vec<u8> {
    v1.iter().zip(v2.iter()).map(|(a, b)| a ^ b).collect()
}

fn xor_single(v: &Vec<u8>, k: u8) -> Vec<u8> {
    v.iter().map(|c| c ^ k).collect()
}

#[cfg(test)]
mod tests {
    extern crate rustc_serialize;

    use self::rustc_serialize::base64::{self, ToBase64};
    use self::rustc_serialize::hex::{FromHex, ToHex};

    use xor::test_data::DETECT_STRINGS;
    use xor::test_data::VIGENERE_STRING;

    use super::*;
    use super::chunks;

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

    #[test]
    fn test_crack() {
        // Set 1 Challenge 3
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let (byte, _, string, _) = crack(&input.from_hex().unwrap(), true);
        assert_eq!(byte, 88);
        assert_eq!(string, "Cooking MC's like a pound of bacon");
    }

    #[test]
    fn test_detect() {
        // Set 1 Challenge 4
        let inputs = DETECT_STRINGS.iter().map(|s| s.from_hex().unwrap()).collect();
        let (i, _, byte, _, string, _) = detect(&inputs, true);
        assert_eq!(i, 170);
        assert_eq!(byte, 53);
        assert_eq!(string, "Now that the party is jumping\n");
    }

    #[test]
    fn test_repeating_key_xor() {
        // Set 1 Challenge 5
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let output = xor_repeating(input.as_bytes(), "ICE".as_bytes()).to_hex();
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_chunks() {
        // Set 1 Challenge 6
        let input = &[1, 2, 3, 4, 5, 6];
        let expected = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        assert_eq!(chunks(input, 3), expected);
    }

    #[test]
    fn test_crack_vigenere() {
        // Set 1 Challenge 6
        let key = crack_vigenere(VIGENERE_STRING).unwrap();
        // Terminator X: Bring the noise
        let expected = &[84, 101, 114, 109, 105, 110, 97, 116, 111, 114, 32, 88, 58, 32, 66, 114, 105, 110, 103, 32, 116, 104, 101, 32, 110, 111, 105, 115, 101];
        assert_eq!(key, expected);
        // It's "Play That Funky Music"
        //println!("{}", String::from_utf8(xor_repeating(VIGENERE_STRING, &key)).unwrap());
    }
}
