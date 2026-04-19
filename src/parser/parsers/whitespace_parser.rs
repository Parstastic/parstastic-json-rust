use crate::{
    node::whitespace::{
        Whitespace, 
        WhitespaceCharacter
    },
    parser::{
        json_parsing_process::JsonParsingProcess, 
        parsers::json_particle_parser::JsonParticleParser, 
        steps::{
            parse_character_step::ParseCharacterStep,
            while_loop_step::WhileLoopStep
        }
    }
};

pub struct WhitespaceParser {
    characters: Vec<WhitespaceCharacter>
}

impl WhitespaceParser {
    pub fn new() -> Self {
        Self {
            characters: Vec::new()
        }
    }

    pub fn new_with_whitespace(whitespace: Whitespace) -> Self {
        Self {
            characters: whitespace.get_characters()
        }
    }
}

impl JsonParticleParser<Whitespace> for WhitespaceParser {
    type Step = WhileLoopStep<ParseCharacterStep<Whitespace, WhitespaceParser>, Whitespace, WhitespaceParser>;

    fn can_parse(&self, _parsing_process: &JsonParsingProcess) -> bool {
        true
    }

    fn get_step(&mut self) -> Self::Step {
        WhileLoopStep::new(
            ParseCharacterStep::new(|p: &mut WhitespaceParser, c| {
                match WhitespaceCharacter::from_character(c) {
                    Some(w) => {
                        p.characters.push(w);
                        true
                    },
                    None => false,
                }
            }),
            |p| p.is_char_valid(WhitespaceCharacter::is_whitespace_character)
        )
    }

    fn create(&self) -> Whitespace {
        Whitespace::new(self.characters.clone())
    }
}
