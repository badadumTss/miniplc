#[derive(Debug)]
pub struct SourceControl {
    source: String,
    raw_instructions: Vec<String>,
    indent: usize,
}

impl SourceControl {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            raw_instructions: Vec::new(),
            indent: 1,
        }
    }

    pub fn incr_indent(&mut self) {
        self.indent += 1
    }

    pub fn decr_indent(&mut self) {
        if self.indent > 1 {
            self.indent -= 1
        }
    }

    pub fn insert_header(&mut self) {
        self.source = "#include <stdio.h>\nint main(){\n".to_string() + &self.source;
    }

    pub fn insert_footer(&mut self) {
        self.source += "\n}";
    }

    pub fn push_instruction(&mut self, instr: String) {
        self.raw_instructions.push(instr);
    }

    pub fn gen_source(&mut self) {
        self.source = String::new();
        self.insert_header();
        for el in self.raw_instructions.iter() {
            for _ in 0..self.indent {
                self.source += "\t";
            }
            self.source += el;
            self.source += "\n";
        }
        self.insert_footer();
    }

    pub fn get_source(&self) -> String {
        self.source.clone()
    }
}
