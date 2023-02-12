fn prefix_function(pattern: &str) -> Vec<usize> {
    let mut prefix = vec![0; pattern.len()];
    let mut k = 0;
    for i in 1..pattern.len() {
        while k > 0 && pattern.chars().nth(k) != pattern.chars().nth(i) {
            k = prefix[k as usize - 1];
        }
        if pattern.chars().nth(k) == pattern.chars().nth(i) {
            k += 1;
        }
        prefix[i] = k;
    }
    prefix
}

fn kmp(text: &str, pattern: &str) -> Vec<usize> {
    let mut matches = vec![];
    let prefix = prefix_function(pattern);
    let mut k = 0;
    for i in 0..text.len() {
        while k > 0 && pattern.chars().nth(k) != text.chars().nth(i) {
            k = prefix[k as usize - 1];
        }
        if pattern.chars().nth(k) == text.chars().nth(i) {
            k += 1;
        }
        if k == pattern.len() {
            matches.push(i - pattern.len() + 1);
            k = prefix[k as usize - 1];
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_kmp() {
        let s = "Hello World World";
        let p = "World";
        let matches = super::kmp(s, p);
        assert_eq!(matches, vec![6, 12]);

        let s = "abcdeaabaaabecfg";
        let p = "aabaaab";
        let matches = super::kmp(s, p);
        assert_eq!(matches, vec![5]);
    }
}
