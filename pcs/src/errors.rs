// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the Jellyfish library.

// You should have received a copy of the MIT License
// along with the Jellyfish library. If not, see <https://mit-license.org/>.

//! Error module.

use super::transcript::TranscriptError;
use ark_serialize::SerializationError;
use ark_std::string::String;
use displaydoc::Display;
#[cfg(feature = "icicle")]
use icicle_core::error::IcicleError;
#[cfg(feature = "icicle")]
use icicle_cuda_runtime::error::CudaError;

/// A `enum` specifying the possible failure modes of the PCS.
#[derive(Display, Debug)]
pub enum PCSError {
    /// Invalid Prover: {0}
    InvalidProver(String),
    /// Invalid Verifier: {0}
    InvalidVerifier(String),
    /// Invalid Proof: {0}
    InvalidProof(String),
    /// Invalid parameters: {0}
    InvalidParameters(String),
    /// An error during (de)serialization: {0}
    SerializationError(SerializationError),
    /// Transcript error {0}
    TranscriptError(TranscriptError),
    /// Error from upstream dependencies: {0}
    UpstreamError(String),
    #[cfg(feature = "icicle")]
    /// Error from ICICLE: {0}
    IcicleError(String),
}

impl ark_std::error::Error for PCSError {}

impl From<SerializationError> for PCSError {
    fn from(e: ark_serialize::SerializationError) -> Self {
        Self::SerializationError(e)
    }
}

impl From<TranscriptError> for PCSError {
    fn from(e: TranscriptError) -> Self {
        Self::TranscriptError(e)
    }
}

#[cfg(feature = "icicle")]
impl From<IcicleError> for PCSError {
    fn from(e: IcicleError) -> Self {
        Self::IcicleError(ark_std::format!("{:?}", e))
    }
}
#[cfg(feature = "icicle")]
impl From<CudaError> for PCSError {
    fn from(e: CudaError) -> Self {
        let icicle_err = IcicleError::from_cuda_error(e);
        icicle_err.into()
    }
}
