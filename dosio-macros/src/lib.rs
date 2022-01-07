//! A macro to build the dos inputs and outputs enum variants
//!
//! For the FEM, the macro get the variant identifiers from the field names of the structures `fem_inputs` and `fem_outputs` in the file `modal_state_space_model_2ndOrder.rs.mat`.
//! The location of the file is given by the environment variable `FEM_REPO`

use proc_macro::TokenStream;

mod io;

#[cfg(feature = "hdf5")]
mod hdf5_io;
#[cfg(feature = "hdf5")]
use hdf5_io::ad_hoc_macro;

#[cfg(feature = "prqt")]
mod parquet_io;
#[cfg(feature = "prqt")]
use parquet_io::ad_hoc_macro;

/// Ad-hoc `dosio` crate builder
#[proc_macro]
pub fn ad_hoc(_item: TokenStream) -> TokenStream {
    ad_hoc_macro(_item)
}
