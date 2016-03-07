use std::collections::HashMap;

pub const ENGLISH_LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyz";
pub const ENGLISH_FREQS: &'static [f64; 26] = &[
    0.08167,
    0.01492,
    0.02782,
    0.04253,
    0.12702,
    0.02228,
    0.02015,
    0.06094,
    0.06966,
    0.00153,
    0.00772,
    0.04025,
    0.02406,
    0.06749,
    0.07507,
    0.01929,
    0.00095,
    0.05987,
    0.06327,
    0.09056,
    0.02758,
    0.00978,
    0.02361,
    0.00150,
    0.01974,
    0.00074,
];

pub fn lower_english_only(s: &str) -> String {
    let s_lower = s.to_lowercase();
    s_lower.chars().filter(|&c| c >= 'a' && c <= 'z').collect()
}

pub fn letter_frequencies(s: &str) -> (usize, HashMap<char, usize>) {
    s.chars().fold((0, HashMap::new()), |(count, mut freqs), c| {
        *freqs.entry(c).or_insert(0) += 1;
        (count + 1, freqs)
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_lower_english_only() {
        assert_eq!(lower_english_only("HELLO world!"), "helloworld");
    }

    #[test]
    fn test_letter_frequencies() {
        let input = "aba";
        let mut expected = HashMap::new();
        expected.insert('a', 2);
        expected.insert('b', 1);
        assert_eq!(letter_frequencies(&input), (3, expected));
    }
}
