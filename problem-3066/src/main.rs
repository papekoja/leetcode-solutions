use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let nums: Vec<i32> = vec![999999999,999999999,999999999];
    let k: i32 = 1000000000;
    let result = Solution::min_operations(nums, k);
    println!("Result: {}", result);
}

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut h: BinaryHeap<Reverse<i64>> = nums.into_iter().map(|x| Reverse(x as i64)).collect();
        let mut res = 0;
        while h.peek().unwrap().0 < k.into() {
            let x = h.pop().unwrap().0;
            let y = h.pop().unwrap().0;
            let z = x.min(y) * 2 + x.max(y);
            h.push(Reverse(z));
            res += 1;
        }
        res
    }
}

