use std::collections::HashMap;

struct Solution;

fn main() {
    let nums: Vec<i32> = vec![18, 43, 36, 13, 7];
    let result = Solution::maximum_sum(nums);
    println!("Result: {}", result);
}

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut digit_highest: HashMap<i32, i32> = HashMap::new();
        let mut res= -1;
        for num in nums {
            let d_sum = calc_digit_sum(num);
            if digit_highest.contains_key(&d_sum) {
                let value = digit_highest.get(&d_sum).unwrap();
                let sum = num + value;
                if sum > res {
                    res = sum;
                }
                if num > *value {
                    digit_highest.insert(d_sum, num);
                }
            } else {
                digit_highest.insert(d_sum, num);
            }
        }
        res
    }
}

pub fn calc_digit_sum(num: i32) -> i32 {
    let mut n = num;
    let mut sum = 0;
    while n != 0 {
        sum += n % 10;
        n = n / 10;
    }
    sum
}
