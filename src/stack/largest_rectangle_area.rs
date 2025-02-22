

pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    if heights.is_empty() {
        return 0;
    }
    if heights.len() == 1 {
        return heights[0];
    }
    let mut max_area = 0;
    // 单调递增栈
    let mut stack: Vec<usize> = vec![];

    for idx in 0..heights.len() {
        if stack.is_empty() {
            stack.push(idx);
        } else {
            if let Some(last_index) = stack.last() {
                // 当前元素等于栈顶元素 出战
               if  heights[*last_index] <=  heights[idx] {
                    stack.push(idx);
                } else {
                    let mut last_index = *last_index;
                    while heights[last_index] > heights[idx] {
                        // 当前元素大于等于栈顶元素
                        let height_idx = stack.pop().unwrap();
                        let witdh = if let Some(i) = stack.last() {
                             idx - *i as usize - 1
                        } else {
                             idx
                        };
                        max_area = max_area.max(witdh * heights[height_idx] as usize);

                        if let Some(v) = stack.last() {
                            last_index = *v;
                        } else {
                           break;
                        }
                    }
                   stack.push(idx);
                }
            }
        }
    }
    // 如果栈中还有元素
    while !stack.is_empty() {
       let last_idx =  stack.pop().unwrap();
       let width =  if let Some(idx) = stack.last() {
           heights.len() - * idx - 1
       } else {
          heights.len()
       };
       let height = heights[last_idx];
       max_area = max_area.max(width * height as usize);
    }
    max_area as i32
}

#[test]
fn largest_rectangle_area_test() {
    let heights = vec![2,1,5,6,2,3];
    assert_eq!(10,largest_rectangle_area(heights));
}
