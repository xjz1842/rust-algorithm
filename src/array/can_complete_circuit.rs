pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut min_gas =  0;

    let mut sum = 0;

    for (i, (g, c)) in (gas.iter().zip(cost.iter())).enumerate() {
        // 在 i 处加油，然后从 i 到 i+1
        sum += g - c;
       // 更新最小油量
        if sum < min_gas {
            min_gas = sum;
            result = i + 1;
        }
    }

    if sum < 0 {
        -1
    } else {
        result as i32
    }
}

#[test]
fn can_complete_circuit_test() {
    let gas = vec![1,2,3,4,5];
    let cost = vec![3,4,5,1,2];
    let result = can_complete_circuit(gas, cost);
    assert_eq!(result, 3);
}