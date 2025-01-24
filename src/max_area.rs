


pub fn max_area(height: Vec<i32>) -> i32 {
     let mut left = 0 ;
     let mut right = height.len() - 1;

     let mut max_area: i32 = 0;

     while left < right {
        
        let h = height[left].min(height[right]);

        max_area = max_area.max((right - left)as i32 * h); 

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
     }   
     
     max_area
}

