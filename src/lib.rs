pub mod error;
pub mod io;
pub use error::DOSIOSError;
pub use io::IO;

pub type DosIoData = Option<Vec<IO<Vec<f64>>>>;
/// Used to glue together the different components of an end-to-end model
pub trait Dos {
    /// Computes and returns a vector outputs from a model component
    fn outputs(&mut self) -> DosIoData;
    /// Passes a vector of input data to a model component
    fn inputs(&mut self, data: DosIoData) -> Result<&mut Self, DOSIOSError>;
    /// Updates the state of a model component for one time step
    fn step(&mut self) -> Result<&mut Self, DOSIOSError>
    where
        Self: Sized + Iterator,
    {
        self.next()
            .and(Some(self))
            .ok_or_else(|| "DOS next step has issued None".into())
            .map_err(DOSIOSError::Step)
    }
    /// Combines `inputs`, `step` and `outputs` in a single method
    ///
    /// This is equivalent to `.inputs(...)?.step()?.outputs()?`
    fn in_step_out(&mut self, data: DosIoData) -> Result<DosIoData, DOSIOSError>
    where
        Self: Sized + Iterator,
    {
        Ok(self.inputs(data)?.step()?.outputs())
    }
}
