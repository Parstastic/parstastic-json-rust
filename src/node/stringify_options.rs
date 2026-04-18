use std::sync::Arc;

use crate::node::whitespace::{
    Whitespace,
    WhitespaceCharacter
};

trait Pretty {
    fn pretty() -> Self;
}

trait Minimal {
    fn minimal() -> Self;
}


pub const DEFAULT_INDENTATION: &str = "    ";


struct WhitespaceStringifyOptions {
    whitespace_creator: Box<dyn Fn(&StringifyOptionsCore) -> Option<Whitespace>>,
}

impl WhitespaceStringifyOptions {
    fn line_break_and_indentation() -> Self {
        Self {
            whitespace_creator: Box::new(|options| {
                let mut s = String::new();
                s.push('\n');
                s.push_str(&DEFAULT_INDENTATION.repeat(options.indentation_level as usize));
                Whitespace::from_string(s)
            })
        }
    }

    fn line_break_and_indentation_reduced() -> Self {
        Self {
            whitespace_creator: Box::new(|options| {
                let mut s = String::new();
                s.push('\n');
                s.push_str(&DEFAULT_INDENTATION.repeat((options.indentation_level as i64 - 1).max(0) as usize));
                Whitespace::from_string(s)
            })
        }
    }

    fn one_space() -> Self {
        Self {
            whitespace_creator: Box::new(|options| Some(Whitespace::new(vec![WhitespaceCharacter::Space])))
        }
    }

    fn to_whitespace(&self, options: &StringifyOptionsCore) -> Option<Whitespace> {
        (self.whitespace_creator)(options)
    }
}

impl Default for WhitespaceStringifyOptions {
    fn default() -> Self {
        Self {
            whitespace_creator: Box::new(|_options| None)
        }
    }
}

impl Minimal for WhitespaceStringifyOptions {
    fn minimal() -> Self {
        Self {
            whitespace_creator: Box::new(|_options| Some(Whitespace::new(Vec::new())))
        }
    }
}


struct JsonValueStringifyOptions<W> {
    leading_whitespace_stringify_options: W,
    trailing_whitespace_stringify_options: W,
}

impl Default for JsonValueStringifyOptions<WhitespaceStringifyOptions> {
    fn default() -> Self {
        Self {
            leading_whitespace_stringify_options: WhitespaceStringifyOptions::default(),
            trailing_whitespace_stringify_options: WhitespaceStringifyOptions::default(),
        }
    }
}

impl Pretty for JsonValueStringifyOptions<WhitespaceStringifyOptions> {
    fn pretty() -> Self {
        Self {
            leading_whitespace_stringify_options: WhitespaceStringifyOptions::minimal(),
            trailing_whitespace_stringify_options: WhitespaceStringifyOptions::minimal(),
        }
    }
}

impl Minimal for JsonValueStringifyOptions<WhitespaceStringifyOptions> {
    fn minimal() -> Self {
        Self {
            leading_whitespace_stringify_options: WhitespaceStringifyOptions::minimal(),
            trailing_whitespace_stringify_options: WhitespaceStringifyOptions::minimal(),
        }
    }
}


struct ContainerNodeStringifyOptions<W> {
    whitespace_stringify_options: W,
    json_value_stringify_options: JsonValueStringifyOptions<W>,
    json_value_last_element_trailing_whitespace_stringify_options: W,
}

impl Default for ContainerNodeStringifyOptions<WhitespaceStringifyOptions> {
    fn default() -> Self {
        Self {
            whitespace_stringify_options: Default::default(),
            json_value_stringify_options: Default::default(),
            json_value_last_element_trailing_whitespace_stringify_options: Default::default()
        }
    }
}

impl Minimal for ContainerNodeStringifyOptions<WhitespaceStringifyOptions> {
    fn minimal() -> Self {
        Self {
            whitespace_stringify_options: Minimal::minimal(),
            json_value_stringify_options: Minimal::minimal(),
            json_value_last_element_trailing_whitespace_stringify_options: Minimal::minimal()
        }
    }
}


struct ArrayNodeStringifyOptions<W> {
    container_node_stringify_options: ContainerNodeStringifyOptions<W>
}

impl Default for ArrayNodeStringifyOptions<WhitespaceStringifyOptions> {
    fn default() -> Self {
        Self {
            container_node_stringify_options: Default::default()
        }
    }
}

impl Pretty for ArrayNodeStringifyOptions<WhitespaceStringifyOptions> {
    fn pretty() -> Self {
        Self {
            container_node_stringify_options: ContainerNodeStringifyOptions {
                whitespace_stringify_options: Minimal::minimal(),
                json_value_stringify_options: JsonValueStringifyOptions {
                    leading_whitespace_stringify_options: WhitespaceStringifyOptions::line_break_and_indentation(),
                    trailing_whitespace_stringify_options: Minimal::minimal()
                },
                json_value_last_element_trailing_whitespace_stringify_options: WhitespaceStringifyOptions::line_break_and_indentation_reduced()
            }
        }
    }
}

impl Minimal for ArrayNodeStringifyOptions<WhitespaceStringifyOptions> {
    fn minimal() -> Self {
        Self {
            container_node_stringify_options: Minimal::minimal()
        }
    }
}


struct ObjectNodeStringifyOptions<W> {
    container_node_stringify_options: ContainerNodeStringifyOptions<W>,
    object_node_property_leading_whitespace_stringify_options: W,
    object_node_property_trailing_whitespace_stringify_options: W
}

impl Default for ObjectNodeStringifyOptions<WhitespaceStringifyOptions> {
    fn default() -> Self {
        Self {
            container_node_stringify_options: Default::default(),
            object_node_property_leading_whitespace_stringify_options: Default::default(),
            object_node_property_trailing_whitespace_stringify_options: Default::default()
        }
    }
}

impl Pretty for ObjectNodeStringifyOptions<WhitespaceStringifyOptions> {
    fn pretty() -> Self {
        Self {
            container_node_stringify_options: ContainerNodeStringifyOptions {
                whitespace_stringify_options: Minimal::minimal(),
                json_value_stringify_options: JsonValueStringifyOptions {
                    leading_whitespace_stringify_options: WhitespaceStringifyOptions::one_space(),
                    trailing_whitespace_stringify_options: Minimal::minimal()
                },
                json_value_last_element_trailing_whitespace_stringify_options: WhitespaceStringifyOptions::line_break_and_indentation_reduced()
            },
            object_node_property_leading_whitespace_stringify_options: WhitespaceStringifyOptions::line_break_and_indentation(),
            object_node_property_trailing_whitespace_stringify_options: Minimal::minimal()
        }
    }
}

impl Minimal for ObjectNodeStringifyOptions<WhitespaceStringifyOptions> {
    fn minimal() -> Self {
        Self {
            container_node_stringify_options: Minimal::minimal(),
            object_node_property_leading_whitespace_stringify_options: Minimal::minimal(),
            object_node_property_trailing_whitespace_stringify_options: Minimal::minimal()
        }
    }
}


#[derive(Clone)]
pub enum StringifyOptionsContainer {
    None,
    ArrayNode,
    ObjectNode,
}


struct StringifyOptionsCore {
    container: StringifyOptionsContainer,
    is_last_element: bool,
    indentation_level: u32,
}


pub struct StringifyOptions {
    core: StringifyOptionsCore,
    
    json_value_stringify_options: Arc<JsonValueStringifyOptions<WhitespaceStringifyOptions>>,
    array_node_stringify_options: Arc<ArrayNodeStringifyOptions<WhitespaceStringifyOptions>>,
    object_node_stringify_options: Arc<ObjectNodeStringifyOptions<WhitespaceStringifyOptions>>,

    json_value_stringify_options_configured: JsonValueStringifyOptions<Option<Whitespace>>,
    array_node_stringify_options_configured: ArrayNodeStringifyOptions<Option<Whitespace>>,
    object_node_stringify_options_configured: ObjectNodeStringifyOptions<Option<Whitespace>>,
}

impl StringifyOptions {
    fn new(
        json_value_stringify_options: JsonValueStringifyOptions<WhitespaceStringifyOptions>,
        array_node_stringify_options: ArrayNodeStringifyOptions<WhitespaceStringifyOptions>,
        object_node_stringify_options: ObjectNodeStringifyOptions<WhitespaceStringifyOptions>,
    ) -> Self {
        Self::next_level(
            StringifyOptionsContainer::None,
            false,
            0,
            Arc::new(json_value_stringify_options),
            Arc::new(array_node_stringify_options),
            Arc::new(object_node_stringify_options),
        )
    }

    fn next_level(
        container: StringifyOptionsContainer,
        is_last_element: bool,
        indentation_level: u32,
        json_value_stringify_options: Arc<JsonValueStringifyOptions<WhitespaceStringifyOptions>>,
        array_node_stringify_options: Arc<ArrayNodeStringifyOptions<WhitespaceStringifyOptions>>,
        object_node_stringify_options: Arc<ObjectNodeStringifyOptions<WhitespaceStringifyOptions>>,
    ) -> Self {
        let core = StringifyOptionsCore {
            container,
            is_last_element,
            indentation_level,
        };

        let json_value_stringify_options_configured = JsonValueStringifyOptions {
            leading_whitespace_stringify_options: json_value_stringify_options.leading_whitespace_stringify_options.to_whitespace(&core),
            trailing_whitespace_stringify_options: json_value_stringify_options.trailing_whitespace_stringify_options.to_whitespace(&core),
        };
        let array_node_stringify_options_configured = ArrayNodeStringifyOptions {
            container_node_stringify_options: ContainerNodeStringifyOptions {
                whitespace_stringify_options: array_node_stringify_options.container_node_stringify_options.whitespace_stringify_options.to_whitespace(&core),
                json_value_stringify_options: JsonValueStringifyOptions {
                    leading_whitespace_stringify_options: array_node_stringify_options.container_node_stringify_options.json_value_stringify_options.leading_whitespace_stringify_options.to_whitespace(&core),
                    trailing_whitespace_stringify_options: array_node_stringify_options.container_node_stringify_options.json_value_stringify_options.trailing_whitespace_stringify_options.to_whitespace(&core),
                },
                json_value_last_element_trailing_whitespace_stringify_options: array_node_stringify_options.container_node_stringify_options.json_value_last_element_trailing_whitespace_stringify_options.to_whitespace(&core),
            },
        };
        let object_node_stringify_options_configured = ObjectNodeStringifyOptions {
            container_node_stringify_options: ContainerNodeStringifyOptions {
                whitespace_stringify_options: object_node_stringify_options.container_node_stringify_options.whitespace_stringify_options.to_whitespace(&core),
                json_value_stringify_options: JsonValueStringifyOptions {
                    leading_whitespace_stringify_options: object_node_stringify_options.container_node_stringify_options.json_value_stringify_options.leading_whitespace_stringify_options.to_whitespace(&core),
                    trailing_whitespace_stringify_options: object_node_stringify_options.container_node_stringify_options.json_value_stringify_options.trailing_whitespace_stringify_options.to_whitespace(&core),
                },
                json_value_last_element_trailing_whitespace_stringify_options: object_node_stringify_options.container_node_stringify_options.json_value_last_element_trailing_whitespace_stringify_options.to_whitespace(&core),
            },
            object_node_property_leading_whitespace_stringify_options: object_node_stringify_options.object_node_property_leading_whitespace_stringify_options.to_whitespace(&core),
            object_node_property_trailing_whitespace_stringify_options: object_node_stringify_options.object_node_property_trailing_whitespace_stringify_options.to_whitespace(&core),
        };

        Self {
            core,

            json_value_stringify_options,
            array_node_stringify_options,
            object_node_stringify_options,
            
            json_value_stringify_options_configured,
            array_node_stringify_options_configured,
            object_node_stringify_options_configured,
        }
    }

    pub fn for_container_node(&self, container: StringifyOptionsContainer) -> Self {
        self.for_container(container, false)
    }

    pub fn for_container_node_last_element(&self, container: StringifyOptionsContainer) -> Self {
        self.for_container(container, true)
    }

    fn for_container(&self, container: StringifyOptionsContainer, is_last_element: bool) -> Self {
        Self::next_level(
            container,
            is_last_element,
            self.core.indentation_level + 1,
            self.json_value_stringify_options.clone(),
            self.array_node_stringify_options.clone(),
            self.object_node_stringify_options.clone(),
        )
    }

    pub fn get_container_node_whitespace<'a>(&'a self, whitespace: &'a Whitespace) -> &'a Whitespace {
        match self.determine_container_node_whitespace() {
            Some(w) => w,
            None => whitespace,
        }
    }

    fn determine_container_node_whitespace(&self) -> &Option<Whitespace> {
        match self.core.container {
            StringifyOptionsContainer::None => &None,
            StringifyOptionsContainer::ArrayNode => &self.array_node_stringify_options_configured.container_node_stringify_options.whitespace_stringify_options,
            StringifyOptionsContainer::ObjectNode => &self.object_node_stringify_options_configured.container_node_stringify_options.whitespace_stringify_options,
        }
    }

    pub fn get_json_value_leading_whitespace<'a>(&'a self, whitespace: &'a Whitespace) -> &'a Whitespace {
        match self.determine_json_value_leading_whitespace() {
            Some(w) => w,
            None => whitespace,
        }
    }

    fn determine_json_value_leading_whitespace(&self) -> &Option<Whitespace> {
        match self.core.container {
            StringifyOptionsContainer::None => &self.json_value_stringify_options_configured.leading_whitespace_stringify_options,
            StringifyOptionsContainer::ArrayNode => &self.array_node_stringify_options_configured.container_node_stringify_options.json_value_stringify_options.leading_whitespace_stringify_options,
            StringifyOptionsContainer::ObjectNode => &self.object_node_stringify_options_configured.container_node_stringify_options.json_value_stringify_options.leading_whitespace_stringify_options,
        }
    }

    pub fn get_json_value_trailing_whitespace<'a>(&'a self, whitespace: &'a Whitespace) -> &'a Whitespace {
        match self.determine_json_value_trailing_whitespace() {
            Some(w) => w,
            None => whitespace,
        }
    }

    fn determine_json_value_trailing_whitespace(&self) -> &Option<Whitespace> {
        match self.core.container {
            StringifyOptionsContainer::None => &self.json_value_stringify_options_configured.trailing_whitespace_stringify_options,
            StringifyOptionsContainer::ArrayNode => {
                if self.core.is_last_element {
                    &self.array_node_stringify_options_configured.container_node_stringify_options.json_value_last_element_trailing_whitespace_stringify_options
                } else {
                    &self.array_node_stringify_options_configured.container_node_stringify_options.json_value_stringify_options.trailing_whitespace_stringify_options
                }
            },
            StringifyOptionsContainer::ObjectNode => {
                if self.core.is_last_element {
                    &self.object_node_stringify_options_configured.container_node_stringify_options.json_value_last_element_trailing_whitespace_stringify_options
                } else {
                    &self.object_node_stringify_options_configured.container_node_stringify_options.json_value_stringify_options.trailing_whitespace_stringify_options
                }
            },
        }
    }

    pub fn get_object_node_property_leading_whitespace<'a>(&'a self, whitespace: &'a Whitespace) -> &'a Whitespace {
        match &self.object_node_stringify_options_configured.object_node_property_leading_whitespace_stringify_options {
            Some(w) => w,
            None => whitespace,
        }
    }

    pub fn get_object_node_property_trailing_whitespace<'a>(&'a self, whitespace: &'a Whitespace) -> &'a Whitespace {
        match &self.object_node_stringify_options_configured.object_node_property_trailing_whitespace_stringify_options {
            Some(w) => w,
            None => whitespace,
        }
    }
}

impl Default for StringifyOptions {
    fn default() -> Self {
        Self::new(
            JsonValueStringifyOptions::default(),
            ArrayNodeStringifyOptions::default(),
            ObjectNodeStringifyOptions::default(),
        )
    }
}

impl Pretty for StringifyOptions {
    fn pretty() -> Self {
        Self::new(
            JsonValueStringifyOptions::pretty(),
            ArrayNodeStringifyOptions::pretty(),
            ObjectNodeStringifyOptions::pretty(),
        )
    }
}

impl Minimal for StringifyOptions {
    fn minimal() -> Self {
        Self::new(
            JsonValueStringifyOptions::minimal(),
            ArrayNodeStringifyOptions::minimal(),
            ObjectNodeStringifyOptions::minimal(),
        )
    }
}
