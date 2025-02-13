#!/bin/bash

# Check if Cargo is installed
if ! command -v cargo &> /dev/null
then
    echo "Cargo is not installed. Please install Rust and Cargo first."
    exit 1
fi

# Prompt the user for the project name
read -p "Enter the Rust project name: " PROJECT_NAME

# Create the Rust project using Cargo
cargo new "$PROJECT_NAME"

# Navigate into the project directory
cd "$PROJECT_NAME" || exit

# Define the boilerplate Rust code
BOILERPLATE_CODE='
fn main() {
    let nums: Vec<i32> = vec![1];
    let result = Solution::solution(nums);
    println!("Result: {}", result);
}

struct Solution;

impl Solution {
    pub fn solution(nums: Vec<i32>) -> i32 {
        0
    }
}
'

# Overwrite the main.rs file with the boilerplate code
echo "$BOILERPLATE_CODE" > src/main.rs

# Create the input.in file inside src
touch link.txt
touch src/input.in

echo "Rust project '$PROJECT_NAME' has been created with boilerplate code and an input.in file."
