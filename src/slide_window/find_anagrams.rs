

pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut result: Vec<_> = Vec::new();
        let mut window = vec![0;26];
        let mut s_window = vec![0;26];

        let s_vec : Vec<char> = s.chars().collect();
        let p_vec : Vec<char> = p.chars().collect();

        let s_len = s_vec.len();
        let p_len = p_vec.len();

        if s_len < p_len {
            return result;
        }
        for i in 0..p_len {
            window[(p_vec[i] as u8 - b'a') as usize] += 1;
            s_window[(s_vec[i] as u8- b'a') as usize] += 1;
        }
        if s_window == window {
            result.push(0);
        }
        for i in p_len ..s_len {
            s_window[(s_vec[i] as u8- b'a') as usize] += 1;
            s_window[(s_vec[i - p_len] as u8- b'a') as usize] -= 1;
            if s_window == window {
                result.push((i - p_vec.len() + 1) as i32);
            }
        }
        result
}