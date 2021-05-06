use std::fs;
use std::path;

struct SimpleLog {
    dir: String
}

impl SimpleLog {
    fn load(&mut self) {
        let dirs = match fs::read_dir(&self.dir) {
            Ok(d) => {
                d
            },
            Err(err) => {
                unreachable!("{:?}", err);
            }
        };
        let mut names = Vec::new();
        for dir in dirs {
            let dir = dir.unwrap();
            names.push(dir.file_name());
        }
        if names.is_empty() {
            return
        }
        names.sort();
        let lastest_name = names.last().unwrap();
        let path = path::Path::new(&self.dir);
        path.join(lastest_name);
        /*
         * open file
         * */
    }

    pub fn new(dir: String) -> Self {
        Self {
            dir: dir
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_test() {
        let mut log = SimpleLog::new("tmp".to_string());
        log.load();
    }
}

