pub type SemanticResult<T = ()> = Result<T, SemanticError>;

#[derive(Debug)]
pub struct SemanticError {
    pub message: String,
}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Semantic error: {}", self.message)
    }
}

impl std::error::Error for SemanticError {
    //
}

#[macro_export]
macro_rules! invalidate {
    ($($arg:tt)*) => {
        return Err($crate::SemanticError {
            message: format!($($arg)*),
        })
    };
}
