
fn main() {
    let mut table = DataTable::new();
    table.set(b"Mathias", b"aaaa");
    table.get(b"Mathias");
}


struct DataEntry {
    key: Vec<u8>,
    value: Option<Vec<u8>>
}

struct DataTable {
    entries: Vec<DataEntry>,
    size: usize
}

impl DataTable {
    pub fn new() -> DataTable {
        DataTable { entries: Vec::new(), size: 0 }
    }

    fn get_index(&self, key: &[u8]) -> Result<usize, usize> {
        self
        .entries
        .binary_search_by_key(&key, |entry| entry.key.as_slice())
    }

    pub fn set(&mut self, key: &[u8], value: &[u8]) {
        //Creating the DataEntry with value given
        let data = DataEntry {
            key: key.to_owned(),
            value: Some(value.to_owned())   
        };
        //Searching in the array for a position to insert the DataEntry
        match self.get_index(key) {
            Ok(idx) => {
                if let Some(v) = self.entries[idx].value.as_ref() {
                    if value.len() < v.len() {
                        self.size -= v.len() - value.len();
                    } else {
                        self.size += value.len() - v.len();
                    }
                }
                self.entries[idx] = data;
            }

            Err(idx) => {
                self.size += key.len() + value.len() + 16 + 1;
                self.entries.insert(idx, data)
            }
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<&DataEntry> {
        if let Ok(idx) = self.get_index(key) {
            println!("{:?}", &self.entries[idx].value);
            return Some(&self.entries[idx]);
        }
        None
    } 

}

