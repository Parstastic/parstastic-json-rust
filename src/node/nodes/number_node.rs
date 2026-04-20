use crate::node::{
    json_particle::JsonParticle,
    nodes::json_node::JsonNode,
    stringify_options::StringifyOptions
};


pub enum NumberNodeBase {
    UInt(u64),
    Int(i64),
    Float(f64),
}

impl NumberNodeBase {
    pub fn to_float(&self) -> f64 {
        match *self {
            NumberNodeBase::UInt(value) => value as f64,
            NumberNodeBase::Int(value) => value as f64,
            NumberNodeBase::Float(value) => value,
        }
    }
}

impl ToString for NumberNodeBase {
    fn to_string(&self) -> String {
        match self {
            NumberNodeBase::UInt(value) => value.to_string(),
            NumberNodeBase::Int(value) => value.to_string(),
            NumberNodeBase::Float(value) => value.to_string(),
        }
    }
}


pub struct NumberNodeExponent {
    is_capitalized: bool,
    sign: NumberNodeExponentSignSymbol,
    value: u64
}

impl NumberNodeExponent {
    pub fn new(is_capitalized: bool, sign: NumberNodeExponentSignSymbol, value: u64) -> Self {
        Self {
            is_capitalized,
            sign,
            value
        }
    }
}


#[derive(Clone, Copy)]
pub enum NumberNodeExponentSignSymbol {
    Blank,
    Plus,
    Minus
}

impl NumberNodeExponentSignSymbol {
    pub fn get_symbol(&self) -> String {
        match self {
            Self::Blank => "",
            Self::Plus => "+",
            Self::Minus => "-"
        }.to_string()
    }

    pub fn get_value(&self) -> i8 {
        match self {
            Self::Blank => 1,
            Self::Plus => 1,
            Self::Minus => -1
        }
    }
}


pub const NEGATIVE_NUMBER_PREFIX: char = '-';

pub const DECIMAL_DELIMITER: char = '.';

pub const EXPONENT_SYMBOL: char = 'e';
pub const EXPONENT_SYMBOL_CAPITALIZED: char = 'E';


pub struct NumberNode {
    base: NumberNodeBase,
    exponent: Option<NumberNodeExponent>
}

impl NumberNode {
    pub fn new_without_exponent(value: NumberNodeBase) -> Self {
        Self {
            base: value,
            exponent: None
        }
    }

    pub fn new_with_exponent(base: NumberNodeBase, exponent: NumberNodeExponent) -> Self {
        Self {
            base,
            exponent: Some(exponent)
        }
    }

    pub fn get_numeric_value(&self) -> f64 {
        match &self.exponent {
            Some(exponent) => {
                let exponent_value = exponent.sign.get_value() as i32
                    * exponent.value.min(i32::MIN as u64) as i32;

                self.base.to_float() * 10f64.powi(exponent_value)
            },
            None => self.base.to_float(),
        }
    }
}

impl JsonNode for NumberNode {

}

impl JsonParticle for NumberNode {
    fn stringify_with_options(&self, _options: &StringifyOptions) -> String {
        let mut s = String::new();

        s.push_str(&self.base.to_string());

        match &self.exponent {
            Some(exponent) => {
                if exponent.is_capitalized {
                    s.push(EXPONENT_SYMBOL_CAPITALIZED);
                } else {
                    s.push(EXPONENT_SYMBOL);
                }

                s.push_str(&exponent.sign.get_symbol());

                s.push_str(&exponent.value.to_string());
            },
            None => (),
        }

        return s;
    }
}
