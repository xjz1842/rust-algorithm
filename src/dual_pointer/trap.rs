pub fn trap(height: Vec<i32>) -> i32 {
    let mut result = 0;
    let len: usize = height.len();

    let mut _left_max = vec![];
    let mut _right_max = vec![];

    for (i, v) in height.iter().enumerate() {
        if i == 0 {
              _left_max.push(v);
        } else {
            _left_max.push(v.max(_left_max[i - 1]));
        }
    }
    for (i, v) in height.iter().rev().enumerate() {
        if i == 0 {
                _right_max.push(v);
        } else {
                _right_max.push(v.max(_right_max[i-1]));
        }
    }
    _right_max.reverse();
    
    for i in 0..len - 1 {
        let min_height = _left_max[i].min(_right_max[i]);
        if *min_height > height[i] {
                println!( "{} {:?}",i, *min_height - height[i]);
            result += *min_height - height[i];
        }
    }
    result
}
