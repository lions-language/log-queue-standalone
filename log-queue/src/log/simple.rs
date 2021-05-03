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
        for dir in dirs.iter() {
        }
    }

    pub fn new(dir: String) -> Self {
        Self {
            dir: dir
        }
    }
}

