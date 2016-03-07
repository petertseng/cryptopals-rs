const SET_BITS: &'static [usize; 256] = &[
    0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8,
];

pub fn hamming(s1: &[u8], s2: &[u8]) -> usize {
    s1.iter().zip(s2.iter()).fold(0, |acc, (c1, c2)| {
        acc + SET_BITS[(c1 ^ c2) as usize]
    })
}

pub fn levenshtein(s1: &str, s2: &str) -> usize {
    let mut dist = Vec::new();
    for _ in 0..(s1.len() + 1) {
        dist.push(vec![0; s2.len() + 1]);
    }

    for i in 0..(s1.len() + 1) {
        dist[i][0] = i;
    }
    for j in 0..(s2.len() + 1) {
        dist[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            if c1 == c2 {
                dist[i + 1][j + 1] = dist[i][j];
            } else {
                let delete = dist[i][j + 1] + 1;
                let min = delete;

                let insert = dist[i + 1][j] + 1;
                let min = if insert < min { insert } else { min };

                let substitute = dist[i][j] + 1;
                let min = if substitute < min { substitute } else { min };

                dist[i + 1][j + 1] = min;
            }
        }
    }

    dist[s1.len()][s2.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming() {
        // Set 1 Challenge 6
        assert_eq!(hamming("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()), 37);
    }

    #[test]
    fn test_levenshtein() {
        assert_eq!(levenshtein("hi there", "hi there"), 0);
    }

    #[test]
    fn test_levenshtein_add() {
        assert_eq!(levenshtein("aaa", "aaabb"), 2);
    }

    #[test]
    fn test_levenshtein_del() {
        assert_eq!(levenshtein("aaabb", "aaa"), 2);
    }

    #[test]
    fn test_levenshtein_sub() {
        assert_eq!(levenshtein("aaaa", "bbbb"), 4);
    }

    #[test]
    fn test_levenshtein1() {
        assert_eq!(levenshtein("kittens", "sitting"), 3);
    }

    #[test]
    fn test_levenshtein2() {
        assert_eq!(levenshtein("Saturday", "Sunday"), 3);
    }
}
