impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut counts = [0;26];
        for (s_char, t_char) in s.bytes().zip(t.bytes()){
            counts[(s_char - b'a') as usize] += 1;
            counts[(t_char - b'a') as usize] -= 1;
        }
        counts.iter().all(|&x| x==0)
    }
}
