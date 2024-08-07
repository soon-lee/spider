use getset::Getters;
use serde_json::Value;

#[derive(Debug)]
pub(crate) enum SelectorType {
    CSS,
    XPath,
}
#[derive(Debug)]
pub(crate) enum SelectorAction {
    Text,
    Attribute(String),
}
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct Selector {}
pub(crate) trait Extract {
    fn output(&self) -> Value;
}
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct Mapping {
    expression: String,
    operation: String,
    field: String,
}
impl Mapping {
    pub(crate) fn new(expression: String, operation: String, field: String) -> Self {
        Mapping {
            expression,
            operation,
            field,
        }
    }
}
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct Mapper {
    expression: String,
    operation: String,
    field: String,
    sequential: bool,
    children: Vec<Mapper>,
}
impl Mapper {
    pub(crate) fn new(expression: String, operation: String, field: String, sequential: bool, children: Vec<Mapper>) -> Self {
        Mapper {
            expression,
            operation,
            field,
            sequential,
            children,
        }
    }
}
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct Extractor {
    path: String,
    parser: String,
    mapper: Vec<Mapper>,
    field: String,
}
impl Extractor {
    pub(crate) fn new(path: String, parser: String, mapper: Vec<Mapper>, field: String) -> Self {
        Extractor {
            path,
            parser,
            mapper,
            field,
        }
    }
}
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub(crate) struct Spider {
    origin: String,
    extractor: Vec<Extractor>,
    field: String,
}
impl Spider {
    pub(crate) fn new(origin: String, extractor: Vec<Extractor>, field: String) -> Self {
        Spider {
            origin,
            extractor,
            field,
        }
    }
}
