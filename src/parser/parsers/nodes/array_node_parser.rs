use crate::{
    node::{
        json_value::JsonValue,
        nodes::{
            array_node::{
                ArrayNode,
                DELIMITER_ELEMENTS,
                DELIMITER_END,
                DELIMITER_START
            },
            container_node::{
                ContainerNode,
                ContainerNodeValue
            }
        },
        stringify_options::StringifyOptionsContainer
    },
    parser::{
        json_parsing_process::JsonParsingProcess,
        parsers::{
            json_particle_parser::JsonParticleParser,
            json_value_parser::JsonValueParser,
            nodes::{
                container_node_parser::ContainerNodeParser,
                json_node_parser::JsonNodeParser
            }
        },
        steps::parse_step::ParseStep
    }
};

pub struct ArrayNodeParser {
    container_node: Option<ContainerNode<JsonValue>>
}

impl ArrayNodeParser {
    pub fn new() -> Self {
        Self {
            container_node: None
        }
    }

    fn create_container_node_parser(&self) -> ContainerNodeParser<JsonValue, JsonValueParser> {
        ContainerNodeParser::new(
            DELIMITER_START,
            DELIMITER_END,
            DELIMITER_ELEMENTS,
            StringifyOptionsContainer::ArrayNode,
            |w| JsonValueParser::new_with_leading_whitespace(w),
            || JsonValueParser::new()
        )
    }
}

impl JsonNodeParser<ArrayNode> for ArrayNodeParser {

}

impl JsonParticleParser<ArrayNode> for ArrayNodeParser {
    type Step = ParseStep<ContainerNode<JsonValue>, ContainerNodeParser<JsonValue, JsonValueParser>, ArrayNode, Self>;

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

    fn create(self) -> Option<ArrayNode> {
        let value = self.container_node?.get_value();
        Some(match value {
            ContainerNodeValue::Whitespace(whitespace) => ArrayNode::new_with_whitespace(whitespace),
            ContainerNodeValue::Elements(elements) => ArrayNode::new_with_elements(elements),
        })
    }
}
