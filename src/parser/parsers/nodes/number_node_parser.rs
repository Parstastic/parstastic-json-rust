use crate::{
    node::nodes::number_node::{
        DECIMAL_DELIMITER,
        EXPONENT_SYMBOL,
        EXPONENT_SYMBOL_CAPITALIZED,
        NEGATIVE_NUMBER_PREFIX,
        NumberNode,
        NumberNodeBase,
        NumberNodeExponent,
        NumberNodeExponentSignSymbol
    },
    parser::{
        json_parsing_process::JsonParsingProcess,
        parsers::{
            json_particle_parser::JsonParticleParser,
            nodes::json_node_parser::JsonNodeParser
        },
        steps::{
            block_step::BlockStep,
            export_step::ExportStep,
            json_parsing_step::JsonParsingStep,
            or_step::OrStep,
            parse_character_step::ParseCharacterStep,
            validate_character_step::ValidateCharacterStep,
            while_loop_step::WhileLoopStep
        }
    }
};

pub struct NumberNodeParser {
    base: Vec<char>,
    is_exponent_capitalized: Option<bool>,
    exponent_sign: Option<NumberNodeExponentSignSymbol>,
    exponent: Vec<char>,
}

impl NumberNodeParser {
    pub fn new() -> Self {
        Self {
            base: Vec::new(),
            is_exponent_capitalized: None,
            exponent_sign: None,
            exponent: Vec::new(),
        }
    }

    fn is_digit_zero(c: char) -> bool {
        c == '0'
    }

    fn is_digit_one_to_nine(c: char) -> bool {
        c >= '1' && c <= '9'
    }

    fn is_digit(c: char) -> bool {
        Self::is_digit_zero(c) || Self::is_digit_one_to_nine(c)
    }

    fn create_base_add_step(&self) -> ParseCharacterStep<NumberNode, Self> {
        ParseCharacterStep::new(|p: &mut NumberNodeParser, c| {
            p.base.push(c);
            true
        })
    }

    fn create_base_sign_parser(&self) -> OrStep<NumberNode, Self> {
        OrStep::else_success(vec![
            (
                Box::new(|_, p| p.is_at_char(NEGATIVE_NUMBER_PREFIX)),
                Box::new(self.create_base_add_step())
            )
        ])
    }

    fn create_base_parser(&self) -> OrStep<NumberNode, Self> {
        OrStep::else_error(vec![
            (
                Box::new(|_, p| p.is_char_valid(|c| Self::is_digit_zero(c))),
                Box::new(self.create_base_add_step())
            ),
            (
                Box::new(|_, p| p.is_char_valid(|c| Self::is_digit_one_to_nine(c))),
                Box::new(BlockStep::new([
                    Box::new(self.create_base_add_step()),
                    Box::new(WhileLoopStep::new(
                        self.create_base_add_step(),
                        |p| p.is_char_valid(|c| Self::is_digit(c))
                    ))
                ]))
            )
        ])
    }

    fn create_fraction_parser(&mut self) -> OrStep<NumberNode, Self> {
        OrStep::else_success(vec![
            (
                Box::new(|_, p| p.is_at_char(DECIMAL_DELIMITER)),
                Box::new(BlockStep::new([
                    Box::new(self.create_base_add_step()),
                    Box::new(self.create_while_loop_with_at_least_one_iteration_parser(false))
                ]))
            )
        ])
    }

    fn create_while_loop_with_at_least_one_iteration_parser(&self, use_exponent: bool) -> BlockStep<3, NumberNode, Self> {
        BlockStep::new([
            Box::new(ValidateCharacterStep::new(Self::is_digit)),
            Box::new(ParseCharacterStep::new(move |p: &mut NumberNodeParser, c| {
                if use_exponent {
                    p.exponent.push(c);
                } else {
                    p.base.push(c);
                }
                true
            })),
            Box::new(WhileLoopStep::new(
                ParseCharacterStep::new(move |p: &mut NumberNodeParser, c| {
                    if use_exponent {
                        p.exponent.push(c);
                    } else {
                        p.base.push(c);
                    }
                    true
                }),
                |p| p.is_char_valid(Self::is_digit)
            ))
        ])
    }

    fn create_exponent_parser(&self) -> OrStep<NumberNode, Self> {
        OrStep::else_success(vec![
            (
                Box::new(|_, p| p.is_char_valid(
                    |c| c == EXPONENT_SYMBOL || c == EXPONENT_SYMBOL_CAPITALIZED
                )),
                Box::new(BlockStep::new([
                    Box::new(ParseCharacterStep::new(|p: &mut NumberNodeParser, c| {
                        p.is_exponent_capitalized = Some(c.is_uppercase());
                        true
                    })),
                    Box::new(self.create_exponent_sign_parser()),
                    Box::new(self.create_while_loop_with_at_least_one_iteration_parser(true))
                ]))
            )
        ])
    }

    fn create_exponent_sign_parser(&self) -> OrStep<NumberNode, Self> {
        OrStep::new(
            vec![
                self.create_exponent_sign_symbol_parser_entry(NumberNodeExponentSignSymbol::Plus),
                self.create_exponent_sign_symbol_parser_entry(NumberNodeExponentSignSymbol::Minus)
            ],
            Box::new(ExportStep::new(|p: &mut NumberNodeParser, _| {
                p.exponent_sign = Some(NumberNodeExponentSignSymbol::Blank);
                true
            }))
        )
    }

    fn create_exponent_sign_symbol_parser_entry(&self, exponent_sign_symbol: NumberNodeExponentSignSymbol) -> (Box<dyn Fn(&Self, &JsonParsingProcess) -> bool>, Box<dyn JsonParsingStep<NumberNode, Self>>) {
        (
            Box::new(move |_, p| p.starts_with(&exponent_sign_symbol.get_symbol())),
            Box::new(ExportStep::new(move |p: &mut NumberNodeParser, _| {
                p.exponent_sign = Some(exponent_sign_symbol);
                true
            }))
        )
    }

    fn parse_base(&self) -> Option<NumberNodeBase> {
        let s: String = self.base.iter().collect();

        if let Ok(v) = s.parse::<u64>() {
            return Some(NumberNodeBase::UInt(v));
        }

        if let Ok(v) = s.parse::<i64>() {
            return Some(NumberNodeBase::Int(v));
        }

        if let Ok(v) = s.parse::<f64>() {
            return Some(NumberNodeBase::Float(v));
        }

        None
    }

    fn parse_exponent(&self) -> Option<NumberNodeExponent> {
        let s: String = self.exponent.iter().collect();
        let value = s.parse::<u64>().ok()?;

        Some(NumberNodeExponent::new(
            self.is_exponent_capitalized?,
            self.exponent_sign?,
            value
        ))
    }
}

impl JsonNodeParser<NumberNode> for NumberNodeParser {

}

impl JsonParticleParser<NumberNode> for NumberNodeParser {
    type Step = BlockStep<4, NumberNode, Self>;

    fn can_parse(&self, parsing_process: &JsonParsingProcess) -> bool {
        parsing_process.is_char_valid(|c| c == NEGATIVE_NUMBER_PREFIX || Self::is_digit(c))
    }

    fn get_step(&mut self) -> Self::Step {
        BlockStep::new([
            Box::new(self.create_base_sign_parser()),
            Box::new(self.create_base_parser()),
            Box::new(self.create_fraction_parser()),
            Box::new(self.create_exponent_parser()),
        ])
    }

    fn create(self) -> Option<NumberNode> {
        let base = self.parse_base()?;
        Some(match self.parse_exponent() {
            Some(exponent) => NumberNode::new_with_exponent(base, exponent),
            None => NumberNode::new_without_exponent(base),
        })
    }
}
