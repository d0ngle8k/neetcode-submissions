impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for left in 0..nums.len() {
            for right in left + 1..nums.len() {
                if nums[left] + nums[right] == target {
                    return vec![left as i32, right as i32];
                }
            }

        }
        vec![]
    }
}
