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

///  Create IO enum
///
/// Return a single IO enum or a vector of IO enums, either empty or with values
/// ios!(variant)
/// ios!(variant(value))
/// ios!(variant1, variant2, ...)
/// ios!(variant1(value1), variant2(value2), ...)
#[macro_export]
macro_rules! ios {
    // ios!(variant)
    ($name:ident) => {
        $crate::io::jar::$name::io::<()>()
    };
    // ios!(variant(value))
    ($name:ident($value:expr)) => {
        $crate::io::jar::$name::io_with($value)
    };
    // ios!(variant1, variant2, ...)
   ($($name:ident),+) => {
        vec![$($crate::io::jar::$name::io::<()>()),+]
    };
    // ios!(variant1(value1), variant2(value2), ...)
    ($($name:ident($value:expr)),+) => {
        vec![$($crate::io::jar::$name::io_with($value)),+]
    };
}

/// Specialization of [`Vec`] of [`IO`]
pub trait IOVec {
    type Output: std::cmp::PartialEq<IO<()>>
        + std::cmp::PartialEq<IO<Vec<f64>>>
        + std::cmp::PartialEq;
    /// Removes and returns all the elements of a [`Vec`] that are equal to the elements in `vals`
    fn pop_these(&mut self, vals: Vec<IO<()>>) -> Option<Vec<Self::Output>>;
    /// Removes and returns the element of [`Vec`] that is equal to `val`
    fn pop_this(&mut self, val: IO<()>) -> Option<Self::Output> {
        self.pop_these(vec![val]).and_then(|mut x| x.pop())
    }
    /// Replaces all the elements of a [`Vec`] that are equal to the elements in `vals` by the corresponding value
    fn swap_these(&mut self, vals: Vec<Self::Output>);
    /// Replaces the element of [`Vec`] that is equal to `val` by `val`
    fn swap_this(&mut self, val: Self::Output) {
        self.swap_these(vec![val])
    }
}
impl<T: std::cmp::PartialEq<IO<()>> + std::cmp::PartialEq<IO<Vec<f64>>> + std::cmp::PartialEq> IOVec
    for Vec<T>
{
    type Output = T;
    fn pop_these(&mut self, vals: Vec<IO<()>>) -> Option<Vec<Self::Output>> {
        vals.into_iter()
            .map(|val| {
                self.iter()
                    .position(|io| *io == val)
                    .map(|idx| self.remove(idx))
            })
            .collect()
    }
    fn swap_these(&mut self, vals: Vec<T>) {
        vals.into_iter().for_each(|val| {
            if let Some(idx) = self.iter().position(|io| *io == val) {
                self[idx] = val;
            }
        });
    }
}

/// Used to get the list of inputs or outputs
pub trait IOTags {
    /// Return the list of outputs
    fn outputs_tags(&self) -> Vec<IO<()>>;
    /// Return the list of inputs
    fn inputs_tags(&self) -> Vec<IO<()>>;
}

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
