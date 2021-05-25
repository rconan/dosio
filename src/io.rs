//! DOS inputs/outputs
//!
//! Provides the definitions for all the inputs and outputs used by DOS

use core::fmt::Debug;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{AddAssign, Deref, DerefMut, Index, IndexMut, SubAssign};

/// IO Error type
#[derive(Debug)]
pub enum IOError<T> {
    Missing(IO<T>),
}
impl<T: Debug> fmt::Display for IOError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing(v) => write!(f, "{:?} is missing", v),
        }
    }
}
impl<T: Debug> std::error::Error for IOError<T> {}

macro_rules! build_io {
    ($($variant:ident),+) => {
        /// Inputs/Outputs definition
        #[derive(Debug,Clone,Serialize,Deserialize)]
        pub enum IO<T> {
            $($variant{data: Option<T>}),+
        }
        impl IO<usize> {
            /// Assign `n` to `IO` `data`
            pub fn assign(&mut self, n: usize) {
                match self {
                    $(IO::$variant{ data: values} => {*values=Some(n);}),+
                }
            }
        }
	impl<T> Deref for IO<T> {
	    type Target = Option<T>;
	    fn deref(&self) -> &Self::Target {
                match self {
                    $(IO::$variant{ data: values} => values),+
                }
	    }
	}
	impl<T> DerefMut for IO<T> {
	    fn deref_mut(&mut self) -> &mut Self::Target {
                match self {
                    $(IO::$variant{ data: values} => values),+
                }
	    }
	}
        impl<T,U> PartialEq<IO<T>> for IO<U> {
            fn eq(&self, other: &IO<T>) -> bool {
                match (self,other) {
                    $((IO::$variant{..},IO::$variant{..}) => true,)+
                    _ => false,
                }
            }
        }
        impl<T,U> From<&IO<U>> for IO<T> {
            /// Converts a `IO<T>` into an `Option<T>`
            fn from(io: &IO<U>) -> Self {
                match io {
                    $(IO::$variant{ ..} => IO::$variant{ data: Default::default()}),+
                }
            }
        }
        impl<T,U: Iterator<Item=T>> From<&mut IO<U>> for Option<IO<T>> {
            /// Converts a `IO<T>` into an `Option<T>`
            fn from(io: &mut IO<U>) -> Self {
                match io {
                    $(IO::$variant{ data: Some(data)} => match data.next() {
                        Some(data) => Some(IO::$variant{ data: Some(data)}),
                        None => None
                    },)+
                        $(IO::$variant{ data: None} => None,)+
                }
            }
        }
        impl<T> From<IO<T>> for Option<T> {
            /// Converts a `IO<T>` into an `Option<T>`
            fn from(io: IO<T>) -> Self {
                match io {
                    $(IO::$variant{ data: values} => values),+
                }
            }
        }
        impl<T> From<(&IO<()>,Option<T>)> for IO<T> {
            fn from((io,data): (&IO<()>,Option<T>)) -> Self {
                match io {
                    $(IO::$variant{ .. } => IO::$variant{ data: data}),+
                }
            }
        }
        impl<T: Debug> From<IO<T>> for Result<T,IOError<T>> {
            /// Converts a `IO<T>` into an `Option<T>`
            fn from(io: IO<T>) -> Self {
                match io {
                    $(IO::$variant{ data: values} =>
                      values.ok_or_else(||
                                        //format!("{:?} data missing",IO::<T>::$variant{data: None}).into()
                                        IOError::Missing(IO::<T>::$variant{data: None})
                    )),+
                }
            }
        }
        impl<T: Clone> From<&IO<T>> for Option<T> {
            /// Converts a `&IO<T>` into an `Option<T>`
            fn from(io: &IO<T>) -> Self {
                match io {
                    $(IO::$variant{ data: values} => values.as_ref().cloned()),+
                }
            }
        }
        impl From<(&IO<usize>,Vec<f64>)> for IO<Vec<f64>> {
            /// Converts a `(&IO<usize>,Vec<f64>)` into an `IO<Vec<f64>>`
            fn from((io,v): (&IO<usize>,Vec<f64>)) -> Self {
                match io {
                    $(IO::$variant{ data: _} => IO::$variant{ data: Some(v)}),+
                }
            }
        }
        impl From<(&IO<()>,Vec<f64>)> for IO<Vec<f64>> {
            /// Converts a `(&IO<()>,Vec<f64>)` into an `IO<Vec<f64>>`
            fn from((io,v): (&IO<()>,Vec<f64>)) -> Self {
                match io {
                    $(IO::$variant{ data: _} => IO::$variant{ data: Some(v)}),+
                }
            }
        }
        impl AddAssign<&IO<Vec<f64>>> for IO<Vec<f64>> {
            fn add_assign(&mut self, other: &IO<Vec<f64>>) {
                match self.clone() {
                    $(IO::$variant{ data: Some(x)} => {
                        Option::<Vec<f64>>::from(other).map(|y| {
                            let z: Vec<_> = x.iter().zip(y).map(|(x,y)| x+y).collect();
                            *self = IO::$variant{ data: Some(z)};
                        });
                    }),+
                        _ => println!("Failed adding IO")
                }
            }
        }
        impl SubAssign<&IO<Vec<f64>>> for IO<Vec<f64>> {
            fn sub_assign(&mut self, other: &IO<Vec<f64>>) {
                match self.clone() {
                    $(IO::$variant{ data: Some(x)} => {
                        Option::<Vec<f64>>::from(other).map(|y| {
                            let z: Vec<_> = x.iter().zip(y).map(|(x,y)| x-y).collect();
                            *self = IO::$variant{ data: Some(z)};
                        });
                    }),+
                        _ => println!("Failed substracting IO")
                }
            }
        }
        pub mod jar {
            //! A DOS Inputs/Outputs builder
            use super::IO;
            $(pub struct $variant {}
              impl $variant {
                  /// Creates a new `IO` type variant with `data` set to `None`
		  #[deprecated(
		      note = "Please use the io function instead"
		  )]
                  pub fn new<T>() -> IO<T> {
                      IO::$variant{ data: None}
                  }
                  /// Creates a new `IO` type variant with `data` set to `None`
                  pub fn io<T>() -> IO<T> {
                      IO::$variant{ data: None}
                  }
                  /// Creates a new `IO` type variant filled with `data`
		  #[deprecated(
		      note = "Please use the io_with function instead"
		  )]
                  pub fn with<T>(data: T) -> IO<T> {
                      IO::$variant{ data: Some(data)}
                  }
                  /// Creates a new `IO` type variant filled with `data`
                  pub fn io_with<T>(data: T) -> IO<T> {
                      IO::$variant{ data: Some(data)}
                  }
              }
            )+
        }
    };
}
/*impl<T> Index<IO<T>> for Vec<IO<T>> {
    type Output = IO<T>;
    fn index(&self, io: IO<T>) -> &Self::Output {
        self.iter()
            .position(|x| *x == io)
            .map(|i| &self[i])
            .unwrap()
    }
}*/
impl<T, U> Index<IO<U>> for Vec<IO<T>> {
    type Output = IO<T>;
    fn index(&self, io: IO<U>) -> &Self::Output {
        self.iter()
            .position(|x| *x == io)
            .map(|i| &self[i])
            .unwrap()
    }
}
impl<T, U> Index<&IO<U>> for Vec<IO<T>> {
    type Output = IO<T>;
    fn index(&self, io: &IO<U>) -> &Self::Output {
        self.iter().position(|x| x == io).map(|i| &self[i]).unwrap()
    }
}
impl<T> IndexMut<IO<T>> for Vec<IO<T>> {
    fn index_mut(&mut self, io: IO<T>) -> &mut Self::Output {
        self.iter()
            .position(|x| *x == io)
            .map(move |i| &mut self[i])
            .unwrap()
    }
}

/// A type for empty `IO`
pub type Tags = IO<()>;

build_io!(
    // Uniform Wind Pressure
    OSSTopEnd6F,  // Top-End
    MCM2Lcl6F,    // M2 segments
    OSSTruss6F,   //Truss
    OSSM1Lcl6F,   // M1 segments
    OSSCellLcl6F, // M1 segment cells
    OSSGIR6F,     // GIR
    OSSCRING6F,   // C-rings
    // Axial wind forces on M1 mirror segments
    M1DistributedWindf,
    // Axial displacement of M1 segment surface nodes
    M1Segment1AxialD,
    M1Segment2AxialD,
    M1Segment3AxialD,
    M1Segment4AxialD,
    M1Segment5AxialD,
    M1Segment6AxialD,
    M1Segment7AxialD,
    // M1 hardpoints
    OSSHarpointDeltaF, // forces
    OSSHardpointD,     // displacements
    // M1 Actuators forces applied to back-side of M1 segments
    M1ActuatorsSegment1,
    M1ActuatorsSegment2,
    M1ActuatorsSegment3,
    M1ActuatorsSegment4,
    M1actuatorsSegment5,
    M1actuatorsSegment6,
    M1ActuatorsSegment7,
    // M1 fans
    OSSM1FansLcl6F,
    OSSM1FansLcl6D,
    // Payloads
    OSSPayloads6F,
    OSSPayloads6D,
    // Mount Drives
    OSSAzDriveF,  // azimuth drive
    OSSElDriveF,  // elevation drive
    OSSGIRDriveF, // GIR drive
    OSSAzDriveD,
    OSSElDriveD,
    OSSGIRDriveD,
    // Mount Drives
    OSSAzDriveTorque,
    OSSElDriveTorque,
    OSSRotDriveTorque,
    OSSAzEncoderAngle,
    OSSElEncoderAngle,
    OSSRotEncoderAngle,
    // Azimuth, elevation, rotation drive torques
    SlewTorques,
    // Line of sight
    OSSM1LOS,  // M1
    MCM2LOS6D, // M2
    // Base of Pier
    OSSBASE6F,
    // Inertial Measurement Units
    OSSIMUs6d,
    // M2 Positioners
    MCM2SmHexF,
    // ASM Proof Mass Actuators
    MCM2PMA1F,
    // ASM
    MCM2CP6F,  // Cold plates
    MCM2RB6F,  // Reference bodies
    MCM2CP6D,  // Cold plates
    MCM2RB6D,  // Reference bodies
    MCM2Lcl6D, // Face sheets
    MCASMCOG6F,
    MCASMCOG6D,
    //
    MCM2TE6F,
    MCM2TEIF6F,
    OSSTrussTEIF6f,
    MCM2GravCS0,
    MCM2PZTS1F,
    MCM2PZTS2F,
    MCM2PZTS3F,
    MCM2PZTS4F,
    MCM2PZTS5F,
    MCM2PZTS6F,
    MCM2PZTS7F,
    MCM2SmallS16F,
    MCM2SmallS26F,
    MCM2SmallS36F,
    MCM2SmallS46F,
    MCM2SmallS56F,
    MCM2SmallS66F,
    MCM2SmallS76F,
    OSSGravCS0,
    OSSTrussIF6D,
    OSSGIR6D,
    OSSCRING6D,
    OSSBASE6D,
    OSSM1Lcl,
    OSSTruss6d,
    OSSCellLcl,
    MCM2SmallS16D,
    MCM2PZTS1D,
    MCM2SmallS26D,
    MCM2PZTS2D,
    MCM2SmallS36D,
    MCM2PZTS3D,
    MCM2SmallS46D,
    MCM2PZTS4D,
    MCM2SmallS56D,
    MCM2PZTS5D,
    MCM2SmallS66D,
    MCM2PZTS6D,
    MCM2SmallS76D,
    MCM2PZTS7D,
    M1SurfacesD,
    M1EdgeSensors,
    MCM2CP1D,
    MCM2SmHexD,
    M2edgesensors,
    MCM2TEIF6D,
    MCM2TE6D,
    M2ReferenceBody1AxialD,
    M2ReferenceBody2AxialD,
    M2ReferenceBody3AxialD,
    M2ReferenceBody4AxialD,
    M2ReferenceBody5AxialD,
    M2ReferenceBody6AxialD,
    M2ReferenceBody7AxialD,
    MountCmd,
    // M1 control
    M1HPCmd,
    M1HPLC,
    M1CGFM,
    SensorData,
    SrcWfeRms,
    Pssn
);
