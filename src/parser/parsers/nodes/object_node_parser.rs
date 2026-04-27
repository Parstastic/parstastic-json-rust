use crate::{
    node::{
        json_particle::JsonParticle,
        json_value::JsonValue,
        nodes::{
            container_node::{
                ContainerNode,
                ContainerNodeValue
            },
            object_node::{
                DELIMITER_ELEMENTS,
                DELIMITER_END,
                DELIMITER_START,
                KEY_VALUE_DELIMITER,
                ObjectNode,
                ObjectNodeProperty
            },
            string_node::StringNode
        },
        stringify_options::StringifyOptionsContainer,
        whitespace::Whitespace
    },
    parser::{
        json_parsing_process::JsonParsingProcess,
        parsers::{
            json_particle_parser::JsonParticleParser,
            json_value_parser::JsonValueParser,
            nodes::{
                container_node_parser::ContainerNodeParser,
                json_node_parser::JsonNodeParser,
                string_node_parser::StringNodeParser
            },
            whitespace_parser::WhitespaceParser
        },
        steps::{
            block_step::BlockStep,
            parse_character_step::ParseCharacterStep,
            parse_step::ParseStep
        }
    }
};

pub struct ObjectNodePropertyParser {
    leading_whitespace: Option<Whitespace>,
    key: Option<StringNode>,
    trailing_whitespace: Option<Whitespace>,
    value: Option<JsonValue>,
}

impl ObjectNodePropertyParser {
    pub fn new() -> Self {
        Self {
            leading_whitespace: None,
            key: None,
            trailing_whitespace: None,
            value: None,
        }
    }

    pub fn new_with_leading_whitespace(leading_whitespace: Whitespace) -> Self {
        Self {
            leading_whitespace: Some(leading_whitespace),
            key: None,
            trailing_whitespace: None,
            value: None,
        }
    }
}

impl JsonParticleParser<ObjectNodeProperty> for ObjectNodePropertyParser {
    type Step = BlockStep<5, ObjectNodeProperty, Self>;

    fn can_parse(&self, parsing_process: &JsonParsingProcess) -> bool {
        WhitespaceParser::new().can_parse(parsing_process)
    }

    fn get_step(&mut self) -> Self::Step {
        BlockStep::new([
            Box::new(ParseStep::new(
                |p: &Self| {
                    match &p.leading_whitespace {
                        Some(whitespace) => WhitespaceParser::new_with_whitespace(whitespace.clone()),
                        None => WhitespaceParser::new(),
                    }
                },
                |w, p, _| {
                    p.leading_whitespace = Some(w);
                    None
                }
            )),
            Box::new(ParseStep::new(
                |_: &Self| StringNodeParser::new(),
                |s, p, _| {
                    p.key = Some(s);
                    None
                }
            )),
            Box::new(ParseStep::new(
                |_: &Self| WhitespaceParser::new(),
                |w, p, _| {
                    p.trailing_whitespace = Some(w);
                    None
                }
            )),
            Box::new(ParseCharacterStep::new_with_expected_character(KEY_VALUE_DELIMITER)),
            Box::new(ParseStep::new(
                |_: &Self| JsonValueParser::new(),
                |v, p, _| {
                    p.value = Some(v);
                    None
                }
            )),
        ])
    }

    fn create(self) -> Option<ObjectNodeProperty> {
        Some(ObjectNodeProperty::new(
            self.leading_whitespace?,
            self.key?,
            self.trailing_whitespace?,
            self.value?
        ))
    }
}


pub struct ObjectNodeParser {
    container_node: Option<ContainerNode<ObjectNodeProperty>>
}

impl ObjectNodeParser {
    pub fn new() -> Self {
        Self {
            container_node: None
        }
    }

    fn create_container_node_parser(&self) -> ContainerNodeParser<ObjectNodeProperty, ObjectNodePropertyParser> {
        ContainerNodeParser::new(
            DELIMITER_START,
            DELIMITER_END,
            DELIMITER_ELEMENTS,
            StringifyOptionsContainer::ArrayNode,
            |w| ObjectNodePropertyParser::new_with_leading_whitespace(w),
        )
    }
}

impl JsonNodeParser<ObjectNode> for ObjectNodeParser {

}

impl JsonParticleParser<ObjectNode> for ObjectNodeParser {
    type Step = ParseStep<ContainerNode<ObjectNodeProperty>, ContainerNodeParser<ObjectNodeProperty, ObjectNodePropertyParser>, ObjectNode, Self>;

    fn can_parse(&self, parsing_process: &JsonParsingProcess) -> bool {
        self.create_container_node_parser().can_parse(parsing_process)
    }

    fn get_step(&mut self) -> Self::Step {
        ParseStep::new(
            |p: &Self| p.create_container_node_parser(),
            |container_node, p, _| {
                p.container_node = Some(container_node);
                None
            }
        )
    }

    fn create(self) -> Option<ObjectNode> {
        let value = self.container_node?.extract_value();
        Some(match value {
            ContainerNodeValue::Whitespace(whitespace) => ObjectNode::new_with_whitespace(whitespace),
            ContainerNodeValue::Elements(elements) => ObjectNode::new_with_elements(elements),
        })
    }
}
