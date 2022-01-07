use arrow::{array::StringArray, record_batch::RecordBatch};
use parquet::{
    arrow::{ArrowReader, ParquetFileArrowReader},
    file::reader::SerializedFileReader,
    util::cursor::SliceableCursor,
};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::env;
use std::{fs::File, io::Read, path::Path, sync::Arc};
use zip::ZipArchive;

use crate::io::{build_io, io_list};

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("No suitable record in file")]
    NoRecord,
    #[error("No suitable data in file")]
    NoData,
    #[error("Cannot read arrow table")]
    ReadArrow(#[from] arrow::error::ArrowError),
    #[error("Cannot read parquet file")]
    ReadParquet(#[from] parquet::errors::ParquetError),
    #[error("Cannot find archive in zip file")]
    Zip(#[from] zip::result::ZipError),
    #[error("Cannot read zip file content")]
    ReadZip(#[from] std::io::Error),
}

pub fn ad_hoc_macro(_item: TokenStream) -> TokenStream {
    let mut variants: Vec<Ident> = if let Ok(fem_repo) = env::var("FEM_REPO") {
        // Gets the FEM repository
        println!(
            "Building `dosio::IO` enum to match inputs/outputs of FEM in {}",
            fem_repo
        );
        // Opens the mat file
        let path = Path::new(&fem_repo);
        let file = if let Ok(val) = File::open(path.join("modal_state_space_model_2ndOrder.zip")) {
            val
        } else {
            return quote!(compile_error!("Cannot find `modal_state_space_model_2ndOrder.zip` in `FEM_REPO`");).into();
        };
        let mut zip_file = if let Ok(val) = zip::ZipArchive::new(file) {
            val
        } else {
            return quote!(compile_error!("`modal_state_space_model_2ndOrder.zip` in `FEM_REPO` is not a valid zip archive");).into();
        };

        get_fem_io(&mut zip_file, "in")
            .into_iter()
            .chain(get_fem_io(&mut zip_file, "out").into_iter())
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
fn get_fem_io(zip_file: &mut ZipArchive<File>, fem_io: &str) -> Result<Vec<Ident>, Error> {
    let mut input_file = zip_file.by_name(&format!(
        "modal_state_space_model_2ndOrder_{}.parquet",
        fem_io
    ))?;
    let mut contents: Vec<u8> = Vec::new();
    input_file.read_to_end(&mut contents)?;

    let mut arrow_reader = ParquetFileArrowReader::new(Arc::new(SerializedFileReader::new(
        SliceableCursor::new(Arc::new(contents)),
    )?));
    if let Ok(input_records) = arrow_reader
        .get_record_reader(2048)?
        .collect::<Result<Vec<RecordBatch>, arrow::error::ArrowError>>()
    {
        let schema = input_records.get(0).unwrap().schema();
        let table = RecordBatch::concat(&schema, &input_records)?;
        let (idx, _) = schema.column_with_name("group").unwrap();
        let data: Option<Vec<&str>> = table
            .column(idx)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap()
            .iter()
            .collect();
        if let Some(mut data) = data {
            data.dedup();
            Ok(data
                .into_iter()
                .map(|fem_io| {
                    let fem_io_rsed = fem_io
                        .split("_")
                        .map(|s| {
                            let (first, last) = s.split_at(1);
                            first.to_uppercase() + last
                        })
                        .collect::<String>();
                    Ident::new(&fem_io_rsed, Span::call_site())
                })
                .collect())
        } else {
            Err(Error::NoData)
        }
    } else {
        Err(Error::NoRecord)
    }
}
