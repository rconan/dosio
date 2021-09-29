//! A macro to build the dos inputs and outputs enum variants
//!
//! For the FEM, the macro get the variant identifiers from the field names of the structures `fem_inputs` and `fem_outputs` in the file `modal_state_space_model_2ndOrder.rs.mat`.
//! The location of the file is given by the environment variable `FEM_REPO`

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::env;
use std::path::Path;

/// Ad-hoc `dosio` crate builder
#[proc_macro]
pub fn ad_hoc(_item: TokenStream) -> TokenStream {
    let mut variants = if let Ok(fem_repo) = env::var("FEM_REPO") {
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
        Vec::<Ident>::new()
    };

    variants.extend(
        [
            // wind loads
            "OSSTopEnd6F",
            "OSSTruss6F",
            "OSSGIR6F",
            "OSSCRING6F",
            "OSSCellLcl6F",
            "OSSM1Lcl6F",
            "MCM2Lcl6F",
            "MCM2TE6F",
            "MCM2RB6F",
            "OSSMirrorCovers6F",
            // mount controller
            "OSSAzDriveTorque",
            "OSSElDriveTorque",
            "OSSRotDriveTorque",
            "MountCmd",
            "OSSAzEncoderAngle",
            "OSSElEncoderAngle",
            "OSSRotEncoderAngle",
            // m1 controller:
            //  - hardpoints load cells
            "M1HPLC",
            "OSSHardpointD",
            "M1HPCmd",
            // - hardpoints dynamics
            "HPFcmd",
            "M1RBMcmd",
            // - CG
            "M1CGFM",
            "M1HPLC",
            // - actuators
            "M1S1HPLC",
            "M1S1BMcmd",
            "M1S1ACTF",
            "M1ActuatorsSegment1",
            "M1S2HPLC",
            "M1S2BMcmd",
            "M1S2ACTF",
            "M1ActuatorsSegment2",
            "M1S3HPLC",
            "M1S3BMcmd",
            "M1S3ACTF",
            "M1ActuatorsSegment3",
            "M1S4HPLC",
            "M1S4BMcmd",
            "M1S4ACTF",
            "M1ActuatorsSegment4",
            "M1S5HPLC",
            "M1S5BMcmd",
            "M1S5ACTF",
            "M1ActuatorsSegment5",
            "M1S6HPLC",
            "M1S6BMcmd",
            "M1S6ACTF",
            "M1ActuatorsSegment6",
            "M1S7HPLC",
            "M1S7BMcmd",
            "M1S7ACTF",
            "M1ActuatorsSegment7",
            // fsm controller
            //  - positionner
            "M2poscmd",
            "M2posFB",
            "M2posactF",
            //  - piezostack
            "TTcmd",
            "PZTFB",
            "PZTF",
            //  - tiptilt
            "TTSP",
            "TTFB",
            "TTcmd",
            // CEO
            "SrcWfeRms",
            "Pssn",
            "SensorData",
            "M1modes",
        ]
        .iter()
        .map(|&v| Ident::new(v, Span::call_site())),
    );

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

// Build the enum
fn build_io(variant: Vec<Ident>) -> proc_macro2::TokenStream {
    quote!(
        /// Inputs/Outputs definition
        #[derive(Debug,Clone,Serialize,Deserialize)]
        pub enum IO<T> {
            #(#variant{data: Option<T>}),*
        }
        impl IO<usize> {
            /// Assign `n` to `IO` `data`
            pub fn assign(&mut self, n: usize) {
                match self {
                    #(IO::#variant{ data: values} => {*values=Some(n);}),*
                }
            }
        }
        impl IO<Vec<f64>> {
            /// Compute `io` sum squared
            pub fn sum_sqred(&self) -> f64 {
                match self {
            #(IO::#variant{ data: None} => f64::NAN,)*
                    #(IO::#variant{ data: Some(values)} => values.iter().map(|x: &f64| x * x).sum::<f64>()),*
                }
            }
            /// Compute `io` mean sum squared
            pub fn mean_sum_sqred(&self) -> f64 {
                match self {
            #(IO::#variant{ data: None} => f64::NAN,)*
                    #(IO::#variant{ data: Some(values)} => (values.iter().map(|x: &f64| x * x).sum::<f64>()*(values.len() as f64).recip())),*
                }
            }
            /// Compute the mean
            pub fn mean(&self) -> f64 {
                match self {
            #(IO::#variant{ data: None} => f64::NAN,)*
                    #(IO::#variant{ data: Some(values)} => (values.iter().sum::<f64>()*(values.len() as f64).recip())),*
                }
            }
            /// Compute the variance
            pub fn var(&self) -> f64 {
                match self {
            #(IO::#variant{ data: None} => f64::NAN,)*
                    #(IO::#variant{ data: Some(values)} => {
            let n_recip = (values.len() as f64).recip();
            let mean = values.iter().sum::<f64>()*n_recip;
            values.iter().map(|x| x -mean).map(|x| x*x).sum::<f64>()*n_recip
            }),*
                }
            }
            /// Compute the standard deviation
            pub fn std(&self) -> f64 { self.var().sqrt() }
        }
    impl<T> std::ops::Deref for IO<T> {
        type Target = Option<T>;
        fn deref(&self) -> &Self::Target {
            match self {
                #(IO::#variant{ data: values} => values),*
            }
        }
    }
    impl<T> std::ops::DerefMut for IO<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            match self {
                #(IO::#variant{ data: values} => values),*
            }
        }
    }
        impl<T,U> PartialEq<IO<T>> for IO<U> {
            fn eq(&self, other: &IO<T>) -> bool {
                match (self,other) {
                    #((IO::#variant{..},IO::#variant{..}) => true,)*
                    _ => false,
                }
            }
        }
        impl<T,U> From<&IO<U>> for IO<T> {
            /// Converts a `IO<T>` into an `Option<T>`
            fn from(io: &IO<U>) -> Self {
                match io {
                    #(IO::#variant{ ..} => IO::#variant{ data: Default::default()}),*
                }
            }
        }
        impl<T,U: Iterator<Item=T>> From<&mut IO<U>> for Option<IO<T>> {
            /// Converts a `IO<T>` into an `Option<T>`
            fn from(io: &mut IO<U>) -> Self {
                match io {
                    #(IO::#variant{ data: Some(data)} => data.next().map(|data| IO::#variant{ data: Some(data)}),)*
                    #(IO::#variant{ data: None} => None,)*
                }
            }
        }
        impl<T> From<IO<T>> for Option<T> {
            /// Converts a `IO<T>` into an `Option<T>`
            fn from(io: IO<T>) -> Self {
                match io {
                    #(IO::#variant{ data: values} => values),*
                }
            }
        }
        impl<T> From<(&IO<()>,Option<T>)> for IO<T> {
            fn from((io,data): (&IO<()>,Option<T>)) -> Self {
                match io {
                    #(IO::#variant{ .. } => IO::#variant{ data: data}),*
                }
            }
        }
        impl<T: Debug> From<IO<T>> for Result<T,IOError<T>> {
            /// Converts a `IO<T>` into an `Result<T,IOError<T>>`
            fn from(io: IO<T>) -> Self {
                match io {
                    #(IO::#variant{ data: values} =>
                      values.ok_or_else(||
                                        //format!("{:?} data missing",IO::<T>::#variant{data: None}).into()
                                        IOError::Missing(IO::<T>::#variant{data: None})
                    )),*
                }
            }
        }
        impl<T: Clone> From<&IO<T>> for Option<T> {
            /// Converts a `&IO<T>` into an `Option<T>`
            fn from(io: &IO<T>) -> Self {
                match io {
                    #(IO::#variant{ data: values} => values.as_ref().cloned()),*
                }
            }
        }
        impl From<(&IO<usize>,Vec<f64>)> for IO<Vec<f64>> {
            /// Converts a `(&IO<usize>,Vec<f64>)` into an `IO<Vec<f64>>`
            fn from((io,v): (&IO<usize>,Vec<f64>)) -> Self {
                match io {
                    #(IO::#variant{ data: _} => IO::#variant{ data: Some(v)}),*
                }
            }
        }
        impl From<(&IO<()>,Vec<f64>)> for IO<Vec<f64>> {
            /// Converts a `(&IO<()>,Vec<f64>)` into an `IO<Vec<f64>>`
            fn from((io,v): (&IO<()>,Vec<f64>)) -> Self {
                match io {
                    #(IO::#variant{ data: _} => IO::#variant{ data: Some(v)}),*
                }
            }
        }
        impl std::ops::AddAssign<&IO<Vec<f64>>> for IO<Vec<f64>> {
            fn add_assign(&mut self, other: &IO<Vec<f64>>) {
                match self.clone() {
                    #(IO::#variant{ data: Some(x)} => {
                        Option::<Vec<f64>>::from(other).map(|y| {
                            let z: Vec<_> = x.iter().zip(y).map(|(x,y)| x+y).collect();
                            *self = IO::#variant{ data: Some(z)};
                        });
                    }),*
                        _ => println!("Failed adding IO")
                }
            }
        }
        impl std::ops::SubAssign<&IO<Vec<f64>>> for IO<Vec<f64>> {
            fn sub_assign(&mut self, other: &IO<Vec<f64>>) {
                match self.clone() {
                    #(IO::#variant{ data: Some(x)} => {
                        Option::<Vec<f64>>::from(other).map(|y| {
                            let z: Vec<_> = x.iter().zip(y).map(|(x,y)| x-y).collect();
                            *self = IO::#variant{ data: Some(z)};
                        });
                    }),*
                        _ => println!("Failed substracting IO")
                }
            }
        }
        impl std::ops::MulAssign<f64> for IO<Vec<f64>> {
            fn mul_assign(&mut self, rhs: f64) {
                match self.clone() {
                    #(IO::#variant{ data: Some(x)} => {
            let z: Vec<_> = x.iter().map(|x| x*rhs).collect();
                        *self = IO::#variant{ data: Some(z)};
                    }),*
                        _ => println!("Failed substracting IO")
                }
            }
        }
    impl std::ops::Mul<f64> for &mut IO<Vec<f64>> {
        type Output = ();
        fn mul(self, rhs: f64) -> Self::Output {
                match self {
                    #(IO::#variant{ data: Some(values)} => {values.iter_mut().for_each(|v| {*v*=rhs;})}),*
                        _ => println!("Failed scaling IO")
                };
        }
    }
    impl std::fmt::Display for IO<()> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    #(IO::#variant{ ..} => write!(f,"{}",stringify!(#variant))),*
                }
        }
    }
    impl<T> IO<T> {
        pub fn kind(&self) -> String
        {
            match self {
                    #(IO::#variant{ ..} => stringify!(#variant).to_string()),*
            }
        }
    }
        pub mod jar {
            //! A DOS Inputs/Outputs builder
            use super::IO;
            #(pub struct #variant {}
              impl #variant {
                  /// Creates a new `IO` type variant with `data` set to `None`
          #[deprecated(
              note = "Please use the io function instead"
          )]
                  pub fn new<T>() -> IO<T> {
                      IO::#variant{ data: None}
                  }
                  /// Creates a new `IO` type variant with `data` set to `None`
                  pub fn io<T>() -> IO<T> {
                      IO::#variant{ data: None}
                  }
                  /// Creates a new `IO` type variant filled with `data`
          #[deprecated(
              note = "Please use the io_with function instead"
          )]
                  pub fn with<T>(data: T) -> IO<T> {
                      IO::#variant{ data: Some(data)}
                  }
                  /// Creates a new `IO` type variant filled with `data`
                  pub fn io_with<T>(data: T) -> IO<T> {
                      IO::#variant{ data: Some(data)}
                  }
              }
            )*
        }
    )
}
