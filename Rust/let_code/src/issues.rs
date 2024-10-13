pub mod context;
pub mod tasks;

#[cfg(test)]
mod tests {
    use crate::context;
    use std::{error::Error, path::Path};

    #[test]
    fn two_sum() -> Result<(), Box<dyn Error>> {
        let test_file = Path::new("./src/tasks/two_sum.json");
        context::TestData::load_from_file(test_file)?
            .execute(|test_case| {
                test_case.set_result(crate::tasks::two_sum::impl1(
                    test_case.get_input_param("nums")?.try_into()?,
                    test_case.get_input_param("target")?.try_into()?,
                ))
            })?
            .save_to_file(test_file)?;
        Ok(())
    }
}
