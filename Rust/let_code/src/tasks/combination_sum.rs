/// ! https://leetcode.com/problems/combination-sum/

use std::{collections::{self, HashSet}, result};

pub struct Solution1;
impl Solution1 {
    fn build_combinations(candidates: &Vec<i32>, target: &i32, mut combination: Vec<i32>, result: &mut Vec<Vec<i32>>) {
        let sum: i32 =  combination.iter().sum::<i32>();
        if sum == *target {
            combination.sort();
            for res in result.iter() {
                if *res == combination {
                    return;
                }
            }
            result.push(combination);
            return;
        }
        for combination in candidates.iter().filter(|candidate| **candidate <= (*target - sum)).map(|candidate| {
            let mut combination = combination.clone();
            combination.push(*candidate);
            combination

        }) {
            Self::build_combinations(candidates, target, combination, result);
        }
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result : Vec<Vec<i32>> = Vec::new();
        Self::build_combinations(&candidates, &target, Vec::new(), &mut result);
        return result;
    }
}

#[cfg(test)]
mod tests {
    use crate::tasks::combination_sum::Solution1;

    #[test]
    fn case_1() -> () {
        let real_output = Solution1::combination_sum(vec![2, 3, 6, 7], 7);
        let expected_output: Vec<Vec<i32>> = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(real_output, expected_output);
    }
}