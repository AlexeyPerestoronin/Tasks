use std::error::Error;

pub mod test_case_parameter;
pub use test_case_parameter::TestCaseParameter;

pub struct TestCase<'a> {
    scope: &'a mut serde_json::Map<String, serde_json::Value>,
}

impl TestCase<'_> {
    pub fn new<'a>(scope: &'a mut serde_json::Value) -> Result<TestCase<'a>, Box<dyn Error>> {
        Ok(TestCase {
            scope: scope
                .as_object_mut()
                .ok_or("Error: the a case of test should be presented like json-object!")?,
        })
    }

    pub fn get_input_param(&self, name: &str) -> Result<TestCaseParameter<'_>, Box<dyn Error>> {
        let param = self
            .scope
            .get("Input")
            .ok_or("Error: 'Input' key isn't existing in test-data!")?
            .as_object()
            .ok_or("Error: Input parameter isn't a json-object!")?
            .get(name)
            .ok_or(format!("Error: Input hasn't '{name}' parameter!"))?;
        Ok(TestCaseParameter { param: param })
    }

    pub fn set_result<T>(&mut self, result: T) -> Result<(), Box<dyn Error>>
    where
        T: Into<serde_json::Value>,
    {
        self.scope
            .get_mut("Output")
            .ok_or("Error: 'Output' key isn't existing in test-data!")?
            .as_object_mut()
            .ok_or("Error: Output parameter isn't a json-object!")?
            .insert("actual".to_string(), serde_json::Value::try_from(result)?);
        Ok(())
    }
}
