#[derive(Clone)]
pub struct JsonParsingProcess {
    json: String,
    index: usize,
    allow_trailing_commas: bool,
}

impl JsonParsingProcess {
    fn new(json: String, index: usize, allow_trailing_commas: bool) -> Self {
        Self {
            json,
            index,
            allow_trailing_commas,
        }
    }

    pub fn new_for_json(json: String) -> Self {
        Self::new(json, 0, false)
    }

    pub fn new_for_json_with_trailing_commas(json: String) -> Self {
        Self::new(json, 0, true)
    }

    pub fn are_trailing_commas_allowed(&self) -> bool {
        self.allow_trailing_commas
    }

    pub fn increment_index(&mut self) {
        self.index += 1;
    }

    pub fn get_char(&self) -> Option<char> {
        if self.is_index_in_json() {
            self.json.chars().nth(self.index)
        } else {
            None
        }
    }

    pub fn is_at_char(&self, c: char) -> bool {
        self.is_char_valid(|current_char| current_char == c)
    }

    pub fn is_char_valid(&self, validation_function: impl Fn(char) -> bool) -> bool {
        match self.get_char() {
            Some(c) => validation_function(c),
            None => false
        }
    }

    pub fn is_index_in_json(&self) -> bool {
        self.index < self.json.chars().count()
    }

    pub fn starts_with(&self, string: &str) -> bool {
        let mut json_chars = self.json.chars().skip(self.index);

        for expected in string.chars() {
            match json_chars.next() {
                Some(actual) if actual == expected => {}
                _ => return false
            }
        }

        true
    }

    pub fn is_finished(&self) -> bool {
        self.index == self.json.chars().count()
    }
}
