/// ! https://leetcode.com/problems/two-sum/description/

pub fn impl1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut map: HashMap<i32, usize> = HashMap::new();
    for (index1, num) in nums.into_iter().enumerate() {
        if let Some(index2) = map.get(&(target - num)) {
            return vec![index1 as i32, *index2 as i32];
        }

        if let Some(index2) = map.get(&num) {
            if num * 2 == target {
                return vec![index1 as i32, *index2 as i32];
            }
        }

        map.insert(num, index1);
    }

    panic!("Right answer wan't detected!")
}

#[cfg(test)]
mod tests {
    use crate::tasks::two_sum::impl1;

    #[test]
    fn two_sum() -> () {
        // assert_eq!(impl1(vec![-3, 4, 3, 90], 0), vec![0, 2]);
        assert_eq!(
            impl1(vec![1, 1, 1, 1, 1, 4, 1, 1, 1, 1, 1, 7, 1, 1, 1, 1, 1], 11),
            vec![5, 11]
        );
    }
}
