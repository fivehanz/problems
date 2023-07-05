use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // stack of type char
        let mut stack: Vec<char> = Vec::new();
        // hashmap to store brackets 
        let mut closeBrackets: HashMap<char, char> = HashMap::from([
            (')', '('),
            (']', '['),
            ('}', '{'),
        ]);    
        
        // traverse through the string and match
        for c in s.chars() {
            
            // if char exists in closeBrackets
            if closeBrackets.contains_key(&c) {
            
                if !stack.is_empty() && stack[stack.len()-1] == closeBrackets[&c] {
                    stack.pop();
                } else {
                    return false;
                }
            
            } else {
                stack.push(c);
            }
            
        }
        
        // return bool 
        stack.is_empty()
    }
}
