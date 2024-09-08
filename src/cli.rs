use std::env;

pub const VERSION: &str = "0.1.0";

#[derive(Debug, Clone)]
pub struct Args {
    list: Vec<String>,
}

impl Args {
    pub fn parse(limit: usize) -> Self {
        let mut as_used: Vec<String> = Vec::new();

        for (index, string) in env::args().enumerate() {
            if index == 0 {
                continue;
            }

            as_used.push(string);

            if index == limit {
                break;
            }
        }

        Self { list: as_used }
    }

    pub fn get(&self, index: usize) -> Option<&String> {
        self.list.get(index)
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn compare<'a, F>(&self, index: usize, with: &'a str, f: F)
    where
        F: FnOnce(),
    {
        if let Some(args) = self.list.get(index) {
            if &args[..] == with {
                f();
            }

            return ();
        }
    }
}
