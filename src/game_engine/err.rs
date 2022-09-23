pub trait EngineErrorTrait {
    fn get_error_message(&self) -> &str;
}

impl std::fmt::Debug for dyn EngineErrorTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_error_message())
    }
}

pub type EngineError = Box<dyn EngineErrorTrait>;

pub struct StaticEngineError {
    s: &'static str
}

impl EngineErrorTrait for StaticEngineError {
    fn get_error_message(&self) -> &str {
        self.s
    }
}

impl From<&'static str> for EngineError {
    fn from(s : &'static str) -> Self {
        Box::new(StaticEngineError { s: s })
    }
}

pub struct DynamicEngineError {
    s: String
}

impl EngineErrorTrait for DynamicEngineError {
    fn get_error_message(&self) -> &str {
        self.s.as_str()
    }
}

impl From<String> for Box<dyn EngineErrorTrait> {
    fn from(s : String) -> Self {
        Box::new(DynamicEngineError { s: s })
    }
}