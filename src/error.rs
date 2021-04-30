use std::error::Error;

pub type BoxError = Box<dyn Error>;
/// DOS trait methods error
pub enum DOSIOSError {
    Inputs(BoxError),
    Outputs(BoxError),
    Step(BoxError),
}
impl std::fmt::Display for DOSIOSError {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inputs(_) => write!(f,"An error occured with the Inputs method from DOS trait"),
            Self::Outputs(_) => write!(f,"An error occured with the Outputs method from DOS trait"),
            Self::Step(_) => write!(f,"An error occured with the Step method from DOS trait"),
        }?;
			  if let Some(error) = self.source() {
				    write!(f, "\nCaused by: {}", error)?;
			  }
			  Ok(())
		}
}
impl std::fmt::Debug for DOSIOSError {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			  <DOSIOSError as std::fmt::Display>::fmt(self, f)
		}
}
impl Error for DOSIOSError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Inputs(error) => Some(error.as_ref()),
            Self::Outputs(error) => Some(error.as_ref()),
            Self::Step(error) => Some(error.as_ref()),
        }
    }
}
