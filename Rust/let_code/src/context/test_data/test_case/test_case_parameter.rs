use std::error::Error;

pub struct TestCaseParameter<'a> {
    pub param: &'a serde_json::Value,
}

impl TryInto<String> for TestCaseParameter<'_> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<String, Self::Error> {
        Ok(self
            .param
            .as_str()
            .ok_or("Error: test-parameter isn't a json-string!")?
            .to_string())
    }
}

impl TryInto<i32> for TestCaseParameter<'_> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<i32, Self::Error> {
        Ok(self
            .param
            .as_i64()
            .ok_or("Error: test-parameter isn't a json-number!")?
            .try_into()?)
    }
}

impl TryInto<i64> for TestCaseParameter<'_> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<i64, Self::Error> {
        Ok(self
            .param
            .as_i64()
            .ok_or("Error: test-parameter isn't a json-number!")?)
    }
}

impl TryInto<Vec<i32>> for TestCaseParameter<'_> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<Vec<i32>, Self::Error> {
        let json_array = self
            .param
            .as_array()
            .ok_or("Error: test-parameter isn't a json-array!")?;

        let mut result: Vec<i32> = Vec::new();
        for (i, value) in json_array.iter().enumerate() {
            result.push(
                value
                    .as_i64()
                    .ok_or(format!(
                        "Error: {i}th value of json-array isn't a json-number!"
                    ))?
                    .try_into()?,
            );
        }
        Ok(result)
    }
}

impl TryInto<Vec<i64>> for TestCaseParameter<'_> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<Vec<i64>, Self::Error> {
        let json_array = self
            .param
            .as_array()
            .ok_or("Error: test-parameter isn't a json-array!")?;

        let mut result: Vec<i64> = Vec::new();
        for (i, value) in json_array.iter().enumerate() {
            result.push(value.as_i64().ok_or(format!(
                "Error: {i}th value of json-array isn't a json-number!"
            ))?);
        }
        Ok(result)
    }
}
