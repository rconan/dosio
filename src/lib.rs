pub mod error;
pub mod io;
pub use error::DOSIOSError;
pub use io::IO;

/// Used to glue together the different components of an end-to-end model
pub trait Dos {
    type Input;
    type Output;

    /// Computes and returns a vector outputs from a model component
    fn outputs(&mut self) -> Option<Vec<IO<Self::Output>>>;

    /// Passes a vector of input data to a model component
    fn inputs(&mut self, data: Option<Vec<IO<Self::Input>>>) -> Result<&mut Self, DOSIOSError>;

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
    fn in_step_out(
        &mut self,
        data: Option<Vec<IO<Self::Input>>>,
    ) -> Result<Option<Vec<IO<Self::Output>>>, DOSIOSError>
    where
        Self: Sized + Iterator,
    {
        Ok(self.inputs(data)?.step()?.outputs())
    }
}
