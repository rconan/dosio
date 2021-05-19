//! Interface for the GMT Dynamic Optics Simulation
//!
//! All components of GMT Dynamic Optics Simulations must implement the [`inputs`](Dos::inputs) and [`outputs`](Dos::outputs) method of the [`Dos`] trait.
//! All inputs and outputs must be a variant of the enum type [`IO`].

pub mod error;
pub mod io;

#[doc(inline)]
pub use error::DOSIOSError;
#[doc(inline)]
pub use io::IO;

/// Dynamic Optics Simulation interface
pub trait Dos {
    /// `Self` inputs type
    type Input;
    /// `Self` outputs type
    type Output;

    /// Returns a [`IO`] output vector from `Self`
    fn outputs(&mut self) -> Option<Vec<IO<Self::Output>>>;

    /// Passe a [`IO`] input vector to `Self`
    fn inputs(&mut self, data: Option<Vec<IO<Self::Input>>>) -> Result<&mut Self, DOSIOSError>;

    /// Invokes the `next` method of `Self`
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
