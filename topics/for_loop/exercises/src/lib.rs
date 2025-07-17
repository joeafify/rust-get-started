pub fn sum(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in nums.iter() {
        sum += i
    }
    sum
}

pub fn fill(i: u32, n: usize) -> Vec<u32> {
    let mut v:Vec<u32> = Vec::new();
    for _ in 0..n  {
        v.push(i);
    }
    v
}
