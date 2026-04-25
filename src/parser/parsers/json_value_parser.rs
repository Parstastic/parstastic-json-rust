use crate::{
    node::{
        json_particle::JsonParticle,
        json_value::JsonValue,
        nodes::json_node::JsonNode,
        whitespace::Whitespace
    },
    parser::{
        json_parsing_process::JsonParsingProcess,
        parsers::{
            json_particle_parser::JsonParticleParser,
            nodes::{
                array_node_parser::ArrayNodeParser,
                boolean_node_parser::BooleanNodeParser,
                json_node_parser::JsonNodeParser,
                null_node_parser::NullNodeParser,
                number_node_parser::NumberNodeParser,
                object_node_parser::ObjectNodeParser,
                string_node_parser::StringNodeParser
            },
            whitespace_parser::WhitespaceParser
        },
        steps::{
            block_step::BlockStep,
            json_parsing_step::JsonParsingStep,
            or_step::OrStep,
            parse_step::ParseStep
        }
    }
};

pub struct JsonValueParser {
    leading_whitespace: Option<Whitespace>,
    json_node: Option<Box<dyn JsonNode>>,
    trailing_whitespace: Option<Whitespace>,
}

impl JsonValueParser {
    pub fn new() -> Self {
        Self {
            leading_whitespace: None,
            json_node: None,
            trailing_whitespace: None,
        }
    }

    pub fn new_with_leading_whitespace(leading_whitespace: Whitespace) -> Self {
        Self {
            leading_whitespace: Some(leading_whitespace),
            json_node: None,
            trailing_whitespace: None,
        }
    }

    fn create_parsers_map(&self) -> Vec<(Box<dyn Fn(&Self, &JsonParsingProcess) -> bool>, Box<dyn JsonParsingStep<JsonValue, Self>>)> {
        vec![
            self.create_parser_entry(|| StringNodeParser::new()),
            self.create_parser_entry(|| NumberNodeParser::new()),
            self.create_parser_entry(|| ObjectNodeParser::new()),
            self.create_parser_entry(|| ArrayNodeParser::new()),
            self.create_parser_entry(|| BooleanNodeParser::new()),
            self.create_parser_entry(|| NullNodeParser::new()),
        ]
    }

    fn create_parser_entry<J: JsonNode + 'static, P: JsonNodeParser<J> + 'static, F>(&self, parser_creator: F) -> (Box<dyn Fn(&Self, &JsonParsingProcess) -> bool>, Box<dyn JsonParsingStep<JsonValue, Self>>)
        where F: Fn() -> P + 'static,
    {
        let parser = parser_creator();
        (
            Box::new(move |_, p| parser.can_parse(p)),
            Box::new(self.create_parse_step(
                move |_| parser_creator(),
                |v, p| p.json_node = Some(Box::new(v))
            ))
        )
    }

    fn create_parse_step<J: JsonParticle, P: JsonParticleParser<J>, F1, F2>(&self, parser_creator: F1, next: F2) -> ParseStep<J, P, JsonValue, Self>
        where
            F1: Fn(&mut Self) -> P + 'static,
            F2: Fn(J, &mut Self) + 'static
    {
        ParseStep::new(
            parser_creator,
            move |v, p, _| {
                next(v, p);
                None
            }
        )
    }
}

impl JsonParticleParser<JsonValue> for JsonValueParser {
    type Step = BlockStep<3, JsonValue, Self>;

    fn can_parse(&self, parsing_process: &JsonParsingProcess) -> bool {
        WhitespaceParser::new().can_parse(parsing_process)
    }

    fn get_step(&mut self) -> Self::Step {
        BlockStep::new([
            Box::new(self.create_parse_step(
                |p| {
                    match &p.leading_whitespace {
                        Some(w) => WhitespaceParser::new_with_whitespace(w.clone()),
                        None => WhitespaceParser::new(),
                    }
                },
                |w, p| p.leading_whitespace = Some(w)
            )),
            Box::new(OrStep::else_error(
                self.create_parsers_map()
            )),
            Box::new(self.create_parse_step(
                |_| WhitespaceParser::new(),
                |w, p| p.trailing_whitespace = Some(w)
            )),
        ])
    }

    fn create(self) -> Option<JsonValue> {
        Some(JsonValue::new(
            self.leading_whitespace?,
            self.json_node?,
            self.trailing_whitespace?
        ))
    }
}
