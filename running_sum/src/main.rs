struct Solution {}

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut running_sum = Vec::with_capacity(nums.len());
        for n in nums {
            running_sum.push(n + running_sum.last().unwrap_or(&0));
        }
        running_sum
    }
}

// Transform main into tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = [1,2,3,4];
        assert_eq!(Solution::running_sum(nums.to_vec()), vec![1,3,6,10]);
    }

    #[test]
    fn test_2() {
        let nums = [1,1,1,1, 1];
        assert_eq!(Solution::running_sum(nums.to_vec()), vec![1,2,3,4,5]);
    }

    #[test]
    fn test_3() {
        let nums = [3,1,2,10,1];
        assert_eq!(Solution::running_sum(nums.to_vec()), vec![3,4,6,16,17]);
    }
}
