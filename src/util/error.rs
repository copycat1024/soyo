pub type Result<T = ()> = std::result::Result<T, Error>;

pub struct Error(String);

#[allow(dead_code)]
pub fn error<T>(msg: &str) -> Result<T> {
    Err(Error(msg.to_owned()))
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl<T> std::convert::From<T> for Error
where
    T: std::fmt::Display,
{
    fn from(error: T) -> Self {
        Self(format!("{}", error))
    }
}
