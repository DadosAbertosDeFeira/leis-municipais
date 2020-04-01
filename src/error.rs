use failure::Fail;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Pattern not found")]
    PatternNotFound,
    // #[fail(display = "Unexpected: {}", name)]
    // PatternNotFound { name: String },
    // #[fail(display = "Template not found")]
    // TemplateNotFound(String, Backtrace),
    // #[fail(display = "Handlebars template error")]
    // HandlebarsTemplate(#[cause] handlebars::TemplateError, Backtrace),
    // #[fail(display = "Handlebars render error")]
    // HandlebarsRender(#[cause] handlebars::RenderError, Backtrace),
    // #[fail(display = "Sendgrid error")]
    // Sendgrid(#[cause] sendgrid::errors::SendgridError, Backtrace),
    // #[fail(display = "Error sending mail: {}", _0)]
    // SendgridResponse(http::status::StatusCode),
}

pub trait CapturedOkOrUnexpected<T> {
    fn ok_or_unexpected(self, msg: &str) -> Result<T, Error>;
}

impl<T> CapturedOkOrUnexpected<T> for Option<T> {
    fn ok_or_unexpected(self, _msg: &str) -> Result<T, Error> {
        self.ok_or_else(|| Error::PatternNotFound)
    }
}
