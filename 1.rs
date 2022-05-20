use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        
        for index in 0..nums.len() {
            let sub = target - nums[index];
            
            if map.contains_key(&sub) {                
                return vec![*map.get(&sub).unwrap(), index as i32];
            }
            
            map.insert(nums[index], index as i32);
        }
        
        vec![]
    }
}
