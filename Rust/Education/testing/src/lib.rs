pub fn sum(left: u64, right: u64) -> u64 {
    left + right
}

pub fn mul(left: u64, right: u64) -> Result<u64, String> {
    if left < right {
        Ok(left * right)
    } else {
        Err(format!("left is greater then right: {left} > {right}"))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works1() {
        let result = super::sum(2, 2);
        assert_eq!(result, 4);
    }
}
