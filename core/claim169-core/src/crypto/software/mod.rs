#[cfg(feature = "software-crypto")]
mod aes_gcm;
#[cfg(feature = "software-crypto")]
mod ecdsa;
#[cfg(feature = "software-crypto")]
mod ed25519;

#[cfg(feature = "software-crypto")]
pub use aes_gcm::{AesGcmDecryptor, AesGcmEncryptor};
#[cfg(feature = "software-crypto")]
pub use ecdsa::{EcdsaP256Signer, EcdsaP256Verifier};
#[cfg(feature = "software-crypto")]
pub use ed25519::{Ed25519Signer, Ed25519Verifier};
