use std::{fs::File, io::Write};
pub struct Emitter {
    code: String,
    header: String,
    full_path: String,
}

impl Emitter {
    pub fn new(full_path: String) -> Emitter {
        Emitter {
            code: String::new(),
            header: String::new(),
            full_path,
        }
    }

    pub fn emit(&mut self, code: String) {
        self.code.push_str(&code);
    }

    pub fn emit_line(&mut self, code: String) {
        self.code.push_str(&code);
        self.code.push('\n');
    }

    pub fn header_line(&mut self, code: String) {
        self.header.push_str(&code);
        self.header.push('\n');
    }

    pub fn write_file(&mut self) {
        let mut file = File::create(self.full_path.clone()).unwrap();
        let value = self.header.clone() + self.code.clone().as_str();

        file.write_all(value.as_bytes()).unwrap();
    }
}
