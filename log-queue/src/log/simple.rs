use std::fs;

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
        names.sort();
        for name in names {
            let path = Path::new(self.dir);
            path.join(name);
        }
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

