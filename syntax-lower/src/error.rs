pub type SyntaxResult<T> = Result<T, SyntaxError>;

#[derive(Debug)]
pub struct SyntaxError {
    pub message: String,
}

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Syntax error: {}", self.message)
    }
}

impl std::error::Error for SyntaxError {
    //
}

#[macro_export]
macro_rules! invalidate {
    ($($arg:tt)*) => {
        return Err($crate::SyntaxError {
            message: format!($($arg)*),
        })
    };
}
