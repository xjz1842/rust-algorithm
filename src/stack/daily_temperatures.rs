
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let len = temperatures.len();
        let mut result : Vec<i32> = vec![0;len];
        let mut stack : Vec<i32> = Vec::new();

        for (i, _) in temperatures.iter().enumerate() {
             while !stack.is_empty() && 
             temperatures[stack[stack.len()-1] as usize] < temperatures[i] {
                println!(" {:?} {:?} {:?} ", stack[stack.len()-1] ,
                stack[stack.len()-1], 
                i);
                result[stack[stack.len() - 1] as usize] = 
                    i as i32 - stack[stack.len() - 1] ; 
                stack.pop();
             } 
             stack.push(i as i32);
        }
        result
}