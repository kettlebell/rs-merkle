use core::convert::TryFrom;

//
// sha384.rs
// Author imotai <codego.me@gmail.com>
//
use crate::{prelude::*, Hasher};
use serde_with::serde_as;
use serde_with::Bytes;
use sha2::{digest::FixedOutput, Digest, Sha384};

/// Sha384 implementation of the [`Hasher`] trait.
///
/// # Examples
///
/// ```
/// # use rs_merkle::{MerkleTree, MerkleProof, algorithms::Sha384, Hasher, Error, utils};
/// # use std::convert::TryFrom;
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///  let tree = MerkleTree::<Sha384>::new();
///  let other_tree: MerkleTree<Sha384> = MerkleTree::new();
///
/// let proof_bytes: Vec<u8> = vec![
///     46, 125, 44, 3, 169, 80, 122, 226, 101, 236, 245, 181, 53, 104, 133, 165, 51, 147, 162,
///     2, 157, 36, 19, 148, 153, 114, 101, 161, 162, 90, 239, 198, 37, 47, 16, 200, 54, 16,
///     235, 202, 26, 5, 156, 11, 174, 130, 85, 235, 162, 249, 91, 228, 209, 215, 188, 250,
///     137, 215, 36, 138, 130, 217, 241, 17, 229, 160, 31, 238, 20, 224, 237, 92, 72, 113, 79,
///     34, 24, 15, 37, 173, 131, 101, 181, 63, 151, 121, 247, 157, 196, 163, 215, 233, 57, 99,
///     249, 74,
/// ];
///
/// let proof_result = MerkleProof::<Sha384>::from_bytes(&proof_bytes);
/// # Ok(())
/// # }
/// ```
///
/// [`Hasher`]: crate::Hasher
///

#[serde_as]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hash(#[serde_as(as = "Bytes")] pub [u8; 48]);

impl From<Hash> for Vec<u8> {
    fn from(val: Hash) -> Self {
        val.0.to_vec()
    }
}

impl TryFrom<Vec<u8>> for Hash {
    type Error = Vec<u8>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Clone)]
pub struct Sha384Algorithm {}

impl Hasher for Sha384Algorithm {
    type Hash = Hash;

    fn hash(data: &[u8]) -> Hash {
        let mut hasher = Sha384::new();
        hasher.update(data);
        let inner = <[u8; 48]>::from(hasher.finalize_fixed());
        Hash(inner)
    }
}
