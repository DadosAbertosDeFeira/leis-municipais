use failure::Fail;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{} não encontrado no arquivo {}", _0, _1)]
    PatternNotFound(String, String),
    #[fail(display = "async error")]
    Async(#[cause] std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(inner: std::io::Error) -> Self {
        Error::Async(inner)
    }
}

pub trait CapturedOkOrUnexpected<T> {
    fn ok_or_unexpected(self, pattern: &str, file: &str) -> Result<T, Error>;
}

impl<T> CapturedOkOrUnexpected<T> for Option<T> {
    fn ok_or_unexpected(self, pattern: &str, file: &str) -> Result<T, Error> {
        self.ok_or_else(|| Error::PatternNotFound(pattern.to_string(), file.to_string()))
    }
}
