fn find(nums: &[u32], target: u32) -> [i32; 2] {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left < right {
        let m = (left + right)/2;
        if nums[m] < target {
            left = m + 1;
        } else if nums[m] > target {
            right = m;
        } else {
            return [left as i32, (right - 1) as i32];
        }
    }

    [-1, -1]
}

fn main() {
    let nums = [5,7,7,8,8,10];
    let output = find(&nums, 6);
    println!("{:?}", output);
}
