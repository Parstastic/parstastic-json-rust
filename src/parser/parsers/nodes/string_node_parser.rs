use crate::{
    node::nodes::string_node::{
        DELIMITER, 
        StringNode
    },
    parser::{
        json_parsing_process::JsonParsingProcess,
        parsers::{
            json_particle_parser::JsonParticleParser,
            nodes::json_node_parser::JsonNodeParser
        },
        steps::{
            block_step::BlockStep,
            for_loop_step::ForLoopStep,
            json_parsing_step::JsonParsingStep,
            or_step::OrStep,
            parse_character_step::ParseCharacterStep,
            validate_character_step::ValidateCharacterStep,
            while_loop_step::WhileLoopStep
        }
    }
};

pub struct StringNodeParser {
    characters: Vec<char>
}

impl StringNodeParser {
    pub fn new() -> Self {
        Self {
            characters: Vec::new()
        }
    }

    fn create_add_character_step(&self) -> ParseCharacterStep<StringNode, Self> {
        ParseCharacterStep::new(|p: &mut Self, c| {
            p.characters.push(c);
            true
        })
    }

    fn create_delimiter_parsing_step(&self) -> ParseCharacterStep<StringNode, Self> {
        ParseCharacterStep::new_with_expected_character(DELIMITER)
    }

    fn create_characters_parsing_step(&self) -> WhileLoopStep<OrStep<1, StringNode, Self>, StringNode, Self> {
        WhileLoopStep::new(
            self.create_character_parsing_step(),
            |_, p| !p.is_at_char(DELIMITER)
        )
    }

    fn create_character_parsing_step(&self) -> OrStep<1, StringNode, Self> {
        OrStep::new(
            [(
                Box::new(|_, p| p.is_at_char('\\')),
                Box::new(BlockStep::new([
                    Box::new(self.create_add_character_step()),
                    Box::new(self.create_escape_targets_parsing_step()),
                ]))
            )],
            Box::new(self.create_add_character_step())
        )
    }

    fn create_escape_targets_parsing_step(&self) -> OrStep<8, StringNode, Self> {
        OrStep::new(
            self.create_escape_targets_parser_map(),
            self.create_unicode_parser()
        )
    }

    fn create_escape_targets_parser_map(&self) -> [(Box<dyn Fn(&Self, &JsonParsingProcess) -> bool>, Box<dyn JsonParsingStep<StringNode, Self>>); 8] {
        [
            '"',
            '\\',
            '/',
            'b',
            'f',
            'n',
            'r',
            't'
        ]
            .map(|c| {
                let condition: Box<dyn Fn(&Self, &JsonParsingProcess) -> bool> = Box::new(move |_, p: &JsonParsingProcess| p.is_at_char(c));
                let effect: Box<dyn JsonParsingStep<StringNode, Self>> = Box::new(self.create_add_character_step());

                (condition, effect)
            })
    }

    fn create_unicode_parser(&self) -> Box<dyn JsonParsingStep<StringNode, Self>> {
        Box::new(BlockStep::new([
            Box::new(ValidateCharacterStep::new_with_expected_character('u')),
            Box::new(self.create_add_character_step()),
            Box::new(ForLoopStep::new(
                BlockStep::new([
                    Box::new(ValidateCharacterStep::new(|c| c.is_ascii_hexdigit())),
                    Box::new(self.create_add_character_step()),
                ]),
                4
            ))
        ]))
    }
}

impl JsonNodeParser<StringNode> for StringNodeParser {

}

impl JsonParticleParser<StringNode> for StringNodeParser {
    type Step = BlockStep<3, StringNode, Self>;

    fn can_parse(&self, parsing_process: &JsonParsingProcess) -> bool {
        parsing_process.is_at_char(DELIMITER)
    }

    fn get_step(&mut self) -> Self::Step {
        BlockStep::new([
            Box::new(self.create_delimiter_parsing_step()),
            Box::new(self.create_characters_parsing_step()),
            Box::new(self.create_delimiter_parsing_step()),
        ])
    }

    fn create(self) -> Option<StringNode> {
        Some(StringNode::new(self.characters.iter().collect()))
    }
}
