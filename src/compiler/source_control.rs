#[derive(Debug)]
pub struct SourceControl {
    source: String,
    raw_instructions: Vec<String>,
}

impl SourceControl {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            raw_instructions: Vec::new(),
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
            self.source += el;
            self.source += "\n\t"
        }
        self.insert_footer();
    }

    pub fn get_source(&self) -> String {
        self.source.clone()
    }
}
