

pub fn min_window(s: String, t: String) -> String {
    let s_bytes = s.as_bytes();
    let m = s.len();
    let mut ans_left = 0;
    let mut ans_right = m;
    let mut cnt = [0; 128];
    let mut less = 0;

    for c in t.bytes() {
        let c = c as usize;
        if cnt[c] == 0 {
            less += 1; // 有 less 种字母的出现次数 < t 中的字母出现次数
        }
        cnt[c] += 1;
    }

    let mut left = 0;

    for (right, &c) in s_bytes.iter().enumerate() { // 移动子串右端点
        let c = c as usize;
        cnt[c] -= 1; // 右端点字母移入子串
        if cnt[c] == 0 {
            // 原来窗口内 c 的出现次数比 t 的少，现在一样多
            less -= 1;
        }
        while less == 0 { // 涵盖：所有字母的出现次数都是 >=
            if right - left < ans_right - ans_left { // 找到更短的子串
                ans_left = left; // 记录此时的左右端点
                ans_right = right;
            }
            let x = s_bytes[left] as usize; // 左端点字母
            if cnt[x] == 0 {
                // x 移出窗口之前，检查出现次数，
                // 如果窗口内 x 的出现次数和 t 一样，
                // 那么 x 移出窗口后，窗口内 x 的出现次数比 t 的少
                less += 1;
            }
            cnt[x] += 1; // 左端点字母移出子串
            left += 1;
        }
    }
    if ans_right < m {
        s[ans_left..=ans_right].to_string()
    } else {
        String::new()
    }
}

