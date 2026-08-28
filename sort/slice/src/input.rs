use orx_criterion::Factors;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElementType {
    U64,
    String,
}

impl ElementType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::U64 => "u64",
            Self::String => "String",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InputVariant {
    pub n: usize,
    pub element_type: ElementType,
}

impl InputVariant {
    pub fn len(&self) -> usize {
        1 << self.n
    }
}

impl Factors for InputVariant {
    fn factor_names() -> Vec<&'static str> {
        vec!["n", "type"]
    }

    fn factor_levels(&self) -> Vec<String> {
        vec![
            format!("2e{}", self.n),
            self.element_type.as_str().to_string(),
        ]
    }
}
