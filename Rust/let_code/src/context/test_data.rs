use std::{error::Error, fs::OpenOptions, path::Path};

pub mod test_case;
use test_case::TestCase;

/// Structure to store test-data which is need to convenient solution of letcode's tasks.
pub struct TestData {
    data: serde_json::Value,
}

impl TestData {
    /// Creates new instance by direct loading of test-data as json-type.  
    /// Test-data is verifying on satisfying the next json-template:
    /// ```json
    /// {
    ///     "Data" : [ "// some data ..." ]
    /// }
    /// ```
    /// # Example
    /// ```rs
    /// let test_json_data = serde_json::json!(
    /// {
    ///     "Data": []
    /// }
    /// let test_data = TestData::load(test_json_data)?;
    /// ```
    #[must_use]
    pub fn load(data: serde_json::Value) -> Result<TestData, Box<dyn Error>> {
        data.as_object()
            .ok_or("Error: test data isn't a json-object!")?
            .get("Data")
            .ok_or("Error: test data hasn't 'Data' key!")?
            .as_array()
            .ok_or("Error: Data isn't a json-array!")?;

        Ok(TestData { data })
    }

    /// Creates new instance by loading of test-data from json-file.  
    /// Test-data is verifying on satisfying the next json-template:
    /// ```json
    /// {
    ///     "Data" : [ "// some data ..." ]
    /// }
    /// ```
    /// # Example
    /// ```rs
    /// let test_json_data = serde_json::json!(
    /// {
    ///     "Data": []
    /// });
    /// let test_data = TestData::load_from_file(Path::new("test-data.json"))?;
    /// ```
    #[must_use]
    pub fn load_from_file(load_file: &Path) -> Result<TestData, Box<dyn Error>> {
        let file = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(load_file)?;

        Self::load(serde_json::from_reader(file)?)
    }

    /// Saves test-data to json-file.
    /// # Example
    /// ```rs
    /// TestData::load_from_file(Path::new("from.json"))?.save_to_file(Path::new("to.json"))?;
    /// ```
    pub fn save_to_file(&self, save_file: &Path) -> Result<&Self, Box<dyn Error>> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(save_file)?;

        serde_json::to_writer_pretty(file, &self.data)?;
        Ok(self)
    }

    /// Executes function on every element of test-data array which could be changed.
    /// # Example
    /// ```rs
    /// TestData::load_from_file(Path::new("from.json"))?
    /// .execute(|input| {
    ///     input
    ///         .as_object_mut()
    ///         .ok_or("Error: input value isn't a json-object!")?
    ///         .insert("R".to_string(), serde_json::json!("result data"));
    ///     Ok(())
    /// })?
    /// .save_to_file(Path::new("to.json"))?;
    /// ```
    pub fn execute(
        &mut self,
        f: impl Fn(&mut TestCase) -> Result<(), Box<dyn Error>>,
    ) -> Result<&mut Self, Box<dyn Error>> {
        let data = self
            .data
            .as_object_mut()
            .unwrap()
            .get_mut("Data")
            .unwrap()
            .as_array_mut()
            .unwrap();

        for (i, value) in data.iter_mut().enumerate() {
            match f(&mut TestCase::new(value)?) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error on index {}: {}", i, e);
                    return Err(e);
                }
            }
        }
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::TestData;
    use std::{error::Error, path::Path};

    #[test]
    fn working_test() -> Result<(), Box<dyn Error>> {
        let file_name = "test_data_file.json";
        let test_json_data = serde_json::json!(
            {
                "Legend" : {
                    "Input": "I",
                    "Expected": "E",
                    "Result": "R"
                },
                "Data": [
                    {
                        "I": "input data 1",
                        "E": "expected data 1"
                    },
                    {
                        "I": "input data 2",
                        "E": "expected data 2"
                    }
                ]
            }
        );

        TestData::load(test_json_data)?.save_to_file(Path::new(file_name))?;

        TestData::load_from_file(Path::new(file_name))?
            .execute(|_| Ok(()))?
            .save_to_file(Path::new(file_name))?;

        std::fs::remove_file(file_name).expect("Error: test data file wasn't removed!");

        Ok(())
    }
}
