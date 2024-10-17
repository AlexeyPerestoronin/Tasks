/// ! https://leetcode.com/problems/multiply-strings/description/

type LongDigit = std::collections::LinkedList<u8>;

pub struct Solution1;
impl Solution1 {
    fn sum(mut num1: LongDigit, num2: LongDigit) -> LongDigit {
        if num1.len() < num2.len() {
            return Self::sum(num2, num1);
        }

        let mut reminder: u8 = 0;
        let mut it2 = num2.iter().rev();
        num1.iter_mut()
            .rev()
            .for_each(|d1| {
                *d1 = *d1 + *it2.next().unwrap_or(&0) + reminder;
                reminder = *d1 / 10;
                *d1 = *d1 % 10;
            });

        if reminder > 0 {
            num1.push_front(reminder);
        }

        return num1;
    }

    fn mul(mut num1: LongDigit, d2: u8) -> LongDigit {
        let mut reminder: u8 = 0;
        num1.iter_mut()
            .rev()
            .for_each(|d1| {
                *d1 = *d1 * d2;
                reminder = *d1 / 10;
                *d1 = *d1 % 10;
            });

        if reminder > 0 {
            num1.push_front(reminder);
        }

        return num1;
    }

    pub fn multiply(num1: String, num2: String) -> String {
        if let (Ok(num1), Ok(num2)) = (num1.parse::<u128>(), num2.parse::<u128>()) {
            if let Some(res) = num1.checked_mul(num2) {
                return res.to_string();
            }
        }

        let num1: LongDigit = num1
            .chars()
            .map(|ch| ch as u8)
            .collect();
        let num2: LongDigit = num2
            .chars()
            .map(|ch| ch as u8)
            .collect();

        let mut result: LongDigit = LongDigit::new();
        num2.into_iter()
            .rev()
            .map(|d2| Self::mul(num1.clone(), d2))
            .for_each(|num1| {
                result = Self::sum(num1, result.clone());
            });

        return num1
            .iter()
            .map(|ch| char::from(*ch))
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use crate::tasks::multiply_strings::{ LongDigit, Solution1 };

    #[test]
    fn sum_case_1() -> () {
        let d1 = LongDigit::from([1, 2, 3]);
        let d2 = LongDigit::from([1, 2, 3]);
        let res = LongDigit::from([2, 4, 6]);
        assert_eq!(Solution1::sum(d1, d2), res);
    }

    #[test]
    fn sum_case_2() -> () {
        let d1 = LongDigit::from([1, 9]);
        let d2 = LongDigit::from([1, 2, 3]);
        let res = LongDigit::from([1, 4, 2]);
        assert_eq!(Solution1::sum(d1, d2), res);
    }

    #[test]
    fn sum_case_3() -> () {
        let d1 = LongDigit::from([1]);
        let d2 = LongDigit::from([9, 9, 9, 9]);
        let res = LongDigit::from([1, 0, 0, 0, 0]);
        assert_eq!(Solution1::sum(d1, d2), res);
    }

    #[test]
    fn case_1() -> () {
        let real_output = Solution1::multiply("2".into(), "3".into());
        let expected_output: String = String::from("6");
        assert_eq!(real_output, expected_output);
    }

    #[test]
    fn case_2() -> () {
        let real_output = Solution1::multiply("498828660196".into(), "840477629533".into());
        let expected_output: String = String::from("419254329864656431168468");
        assert_eq!(real_output, expected_output);
    }

    #[test]
    fn case_3() -> () {
        let real_output = Solution1::multiply(
            "401716832807512840963".into(),
            "167141802233061013023557397451289113296441069".into()
        );
        let expected_output: String = String::from("419254329864656431168468");
        assert_eq!(real_output, expected_output);
    }
}
