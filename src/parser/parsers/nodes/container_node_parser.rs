use crate::{
    node::{
        json_particle::JsonParticle,
        nodes::container_node::{
            ContainerNode,
            ContainerNodeValue
        },
        stringify_options::StringifyOptionsContainer,
        whitespace::Whitespace
    },
    parser::{
        json_parsing_process::JsonParsingProcess,
        parsers::{
            json_particle_parser::JsonParticleParser,
            nodes::json_node_parser::JsonNodeParser,
            whitespace_parser::WhitespaceParser
        },
        steps::{
            block_step::BlockStep,
            export_step::ExportStep,
            json_parsing_step::JsonParsingStep,
            or_step::OrStep,
            parse_character_step::ParseCharacterStep,
            parse_step::ParseStep,
            while_loop_step::WhileLoopStep
        }
    }
};

pub struct ContainerNodeParser<P: JsonParticle, JPP: JsonParticleParser<P>> {
    whitespace: Option<Whitespace>,
    elements: Vec<P>,
    delimiter_start: char,
    delimiter_end: char,
    delimiter_elements: char,
    stringify_options_type: StringifyOptionsContainer,
    get_element_parser_with_whitespace: Box<dyn Fn(Whitespace) -> JPP>,
}

impl<P: JsonParticle + 'static, JPP: JsonParticleParser<P> + 'static> ContainerNodeParser<P, JPP> {
    pub fn new<F>(
        delimiter_start: char,
        delimiter_end: char,
        delimiter_elements: char,
        stringify_options_type: StringifyOptionsContainer,
        get_element_parser_with_whitespace: F,
    ) -> Self
    where
        F: Fn(Whitespace) -> JPP + 'static,
    {
        Self {
            whitespace: None,
            elements: Vec::new(),
            delimiter_start,
            delimiter_end,
            delimiter_elements,
            stringify_options_type,
            get_element_parser_with_whitespace: Box::new(get_element_parser_with_whitespace),
        }
    }

    fn create_content_parser(&self) -> ParseStep<Whitespace, WhitespaceParser, ContainerNode<P>, Self> {
        ParseStep::new(
            |_| WhitespaceParser::new(),
            |w, parser: &mut Self, parsing_process| OrStep::new(
                [(
                    Box::new(|parser: &Self, p| !p.is_at_char(parser.delimiter_end)),
                    Box::new(parser.create_elements_parser(w.clone())),
                )],
                Box::new(ExportStep::new(move |p: &mut Self, _| {
                    p.whitespace = Some(w.clone());
                    true
                }))
            ).execute(parser, parsing_process)
        )
    }
    
    fn create_elements_parser(&self, whitespace: Whitespace) -> BlockStep<2, ContainerNode<P>, Self> {
        BlockStep::new([
            Box::new(ParseStep::new(
                move |p: &Self| (p.get_element_parser_with_whitespace)(whitespace.clone()),
                |element, p: &mut Self, _| {
                    p.elements.push(element);
                    None
                }
            )),
            Box::new(WhileLoopStep::new(
                BlockStep::new([
                    Box::new(ParseCharacterStep::new(|_, _| true)),
                    Box::new(ParseStep::new(
                        |_| WhitespaceParser::new(),
                        |w, parser: &mut Self, parsing_process| OrStep::new(
                            [
                                (
                                    Box::new(|parser: &Self, p| p.is_at_char(parser.delimiter_end) && p.are_trailing_commas_allowed()),
                                    Box::new(ExportStep::new(|_, _| {
                                        true
                                    }))
                                )
                            ],
                            Box::new(ParseStep::new(
                                move |p: &Self| (p.get_element_parser_with_whitespace)(w.clone()),
                                |element, parser: &mut Self, _| {
                                    parser.elements.push(element);
                                    None
                                }
                            ))
                        ).execute(parser, parsing_process)
                    )),
                ]),
                |parser, p| p.is_at_char(parser.delimiter_elements)
            )),
        ])
    }
}

impl<P: JsonParticle + 'static, JPP: JsonParticleParser<P> + 'static> JsonNodeParser<ContainerNode<P>> for ContainerNodeParser<P, JPP> {

}

impl<P: JsonParticle + 'static, JPP: JsonParticleParser<P> + 'static> JsonParticleParser<ContainerNode<P>> for ContainerNodeParser<P, JPP> {
    type Step = BlockStep<3, ContainerNode<P>, Self>;

    fn can_parse(&self, parsing_process: &JsonParsingProcess) -> bool {
        parsing_process.is_at_char(self.delimiter_start)
    }

    fn get_step(&mut self) -> Self::Step {
        BlockStep::new([
            Box::new(ParseCharacterStep::new_with_expected_character(self.delimiter_start)),
            Box::new(self.create_content_parser()),
            Box::new(ParseCharacterStep::new_with_expected_character(self.delimiter_end)),
        ])
    }

    fn create(self) -> Option<ContainerNode<P>> {
        Some(ContainerNode::new(
            match self.whitespace {
                Some(whitespace) => ContainerNodeValue::Whitespace(whitespace),
                None => ContainerNodeValue::Elements(self.elements),
            },
            self.delimiter_start,
            self.delimiter_end,
            self.delimiter_elements,
            self.stringify_options_type,
        ))
    }
}
