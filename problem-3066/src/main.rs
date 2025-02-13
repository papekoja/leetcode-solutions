use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let nums: Vec<i32> = vec![999999999, 999999999, 999999999];
    let k: i32 = 1000000000;
    let result = Solution::min_operations(nums, k);
    println!("Result: {}", result);
}

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut min_heap = nums
            .iter()
            .map(|n| Reverse(*n as u64))
            .collect::<BinaryHeap<_>>();

        let mut result = 0;
        let k = k as u64;

        loop {
            let min = min_heap.pop().expect("").0;
            if min >= k {
                break;
            }

            let second_min = min_heap.pop().expect("").0;
            min_heap.push(Reverse(min * 2 + second_min));
            result += 1;
        }

        result
    }
}
