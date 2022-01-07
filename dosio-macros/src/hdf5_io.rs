use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::env;
use std::path::Path;

use crate::io::{build_io, io_list};

pub fn ad_hoc_macro(_item: TokenStream) -> TokenStream {
    let mut variants: Vec<Ident> = if let Ok(fem_repo) = env::var("FEM_REPO") {
        // Gets the FEM repository
        println!(
            "Building `dosio::IO` enum to match inputs/outputs of FEM in {}",
            fem_repo
        );
        // Opens the mat file
        let file = Path::new(&fem_repo).join("modal_state_space_model_2ndOrder.rs.mat");
        let h5 = if let Ok(val) = hdf5::File::open(file) {
            val
        } else {
            return quote!(compile_error!("Cannot find `modal_state_space_model_2ndOrder.rs.mat` in `FEM_REPO`");).into();
        };

        get_fem_io(&h5, "fem_inputs")
            .into_iter()
            .chain(get_fem_io(&h5, "fem_outputs").into_iter())
            .flatten()
            .collect()
    } else {
        println!("`FEM_REPO` environment variable is not set, using dummies instead.");
        [
            "Rodolphe",
            "Rodrigo",
            "Christoph",
            "Henry",
            "Conan",
            "Romano",
            "Dribusch",
            "Fitzpatrick",
        ]
        .iter()
        .map(|&v| Ident::new(v, Span::call_site()))
        .collect()
    };

    variants.extend(io_list().map(|&v| Ident::new(v, Span::call_site())));

    variants.sort();
    variants.dedup();
    let io = build_io(variants);

    quote!(
    #io
    )
    .into()
}

// Read the fields
fn get_fem_io(h5: &hdf5::File, fem_io: &str) -> Result<Vec<Ident>, hdf5::Error> {
    h5.group(fem_io)?.attr("MATLAB_fields")?.read_raw().map(
        |data: Vec<hdf5::types::VarLenArray<hdf5::types::FixedAscii<1>>>| {
            data.into_iter()
                .map(|v| {
                    let fem_io = v.iter().map(|x| x.as_str()).collect::<String>();
                    let fem_io_rsed = fem_io
                        .split("_")
                        .map(|s| {
                            let (first, last) = s.split_at(1);
                            first.to_uppercase() + last
                        })
                        .collect::<String>();
                    Ident::new(&fem_io_rsed, Span::call_site())
                })
                .collect()
        },
    )
}
