use std::collections::HashMap;
use std::time::SystemTime;
pub struct Source {
    pub path: Option<String>,
    pub last_modified: SystemTime,
    pub data: Vec<Vec<String>>,
    pub access_hash: HashMap<String, usize>,
}

impl Default for Source {
    fn default() -> Self {
        Self {
            path: None,
            last_modified: std::time::UNIX_EPOCH,
            data: Vec::new(),
            access_hash: HashMap::new(),
        }
    }
}

impl Source {
    pub fn load_data(&mut self, path: std::path::PathBuf) {
        match std::fs::read_to_string(path) {
            Ok(string) => {
                let mut lines = string.lines();

                if lines.clone().count() == 0 {
                    // TODO: propagate the error, show it in the UI
                    panic!("Error: source file doesn't have any content.");
                }
                // TODO: how to check whether or not the file doesn't have a header?
                // Get the first line
                let line = lines.next().unwrap();
                for (idx, key) in line.split(',').enumerate() {
                    self.access_hash.insert(key.to_string(), idx);
                }

                // Populate the data vec
                for line in lines {
                    let mut row: Vec<String> = Vec::new();
                    for datum in line.split(',') {
                        row.push(datum.to_string());
                    }
                    self.data.push(row);
                }
            }
            Err(e) => panic!("Error while reading source file: {e}"),
        }
    }
}
