use crate::node::{
    json_particle::JsonParticle,
    stringify_options::StringifyOptions
};

pub enum WhitespaceCharacter {
    Space,
    HorizontalTab,
    LineFeed,
    CarriageReturn
}

impl WhitespaceCharacter {
    pub fn is_whitespace_character(character: char) -> bool {
        Self::from_character(character).is_some()
    }

    pub fn from_character(character: char) -> Option<Self> {
        match character {
            ' ' => Some(Self::Space),
            '\t' => Some(Self::HorizontalTab),
            '\n' => Some(Self::LineFeed),
            '\r' => Some(Self::CarriageReturn),
            _ => None
        }
    }

    pub fn vec_to_string(vec: &Vec<WhitespaceCharacter>) -> String {
        vec
            .iter()
            .map(|c| c.get_character())
            .collect()
    }

    pub fn get_character(&self) -> char {
        match self {
            Self::Space => ' ',
            Self::HorizontalTab => '\t',
            Self::LineFeed => '\n',
            Self::CarriageReturn => '\r'
        }
    }
}


pub struct Whitespace {
    characters: Vec<WhitespaceCharacter>
}

impl Whitespace {
    pub fn new(characters: Vec<WhitespaceCharacter>) -> Whitespace {
        Self {
            characters
        }
    }

    pub fn from_string(value: String) -> Option<Whitespace> {
        let mut characters = Vec::new();
        for c in value.chars() {
            match WhitespaceCharacter::from_character(c) {
                Some(w) => characters.push(w),
                None => {
                    return None;
                }
            }
        }
        Some(Self {
            characters
        })
    }
}

impl JsonParticle for Whitespace {
    fn stringify_with_options(&self, _options: StringifyOptions) -> String {
        WhitespaceCharacter::vec_to_string(&self.characters)
    }
}
