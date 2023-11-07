// SPDX-License-Identifier: CC0-1.0

//! Bitcoin hash types.
//!
//! This module defines types for hashes used throughout the library. These
//! types are needed in order to avoid mixing data of the same hash format
//! (e.g. `SHA256d`) but of different meaning (such as transaction id, block
//! hash).
//!

#[rustfmt::skip]
macro_rules! impl_hashencode {
    ($hashtype:ident) => {
        impl $crate::consensus::Encodable for $hashtype {
            fn consensus_encode<W: $crate::io::Write + ?Sized>(&self, w: &mut W) -> Result<usize, $crate::io::Error> {
                self.0.consensus_encode(w)
            }
        }

        impl $crate::consensus::Decodable for $hashtype {
            fn consensus_decode<R: $crate::io::Read + ?Sized>(r: &mut R) -> Result<Self, $crate::consensus::encode::Error> {
                use $crate::hashes::Hash;
                Ok(Self::from_byte_array(<<$hashtype as $crate::hashes::Hash>::Bytes>::consensus_decode(r)?))
            }
        }
    };
}
pub(crate) use impl_hashencode;

#[rustfmt::skip]
macro_rules! impl_asref_push_bytes {
    ($($hashtype:ident),*) => {
        $(
            impl AsRef<$crate::blockdata::script::PushBytes> for $hashtype {
                fn as_ref(&self) -> &$crate::blockdata::script::PushBytes {
                    use $crate::hashes::Hash;
                    self.as_byte_array().into()
                }
            }

            impl From<$hashtype> for $crate::blockdata::script::PushBytesBuf {
                fn from(hash: $hashtype) -> Self {
                    use $crate::hashes::Hash;
                    hash.as_byte_array().into()
                }
            }
        )*
    };
}
pub(crate) use impl_asref_push_bytes;

// newtypes module is solely here so we can rustfmt::skip.
#[rustfmt::skip]
#[doc(inline)]
pub use newtypes::*;

#[rustfmt::skip]
mod newtypes {
    use hashes::{sha256d, hash_newtype};

    hash_newtype! {
        /// A bitcoin transaction hash/transaction ID.
        ///
        /// For compatibility with the existing Bitcoin infrastructure and historical
        /// and current versions of the Bitcoin Core software itself, this and
        /// other [`sha256d::Hash`] types, are serialized in reverse
        /// byte order when converted to a hex string via [`std::fmt::Display`] trait operations.
        /// See [`hashes::Hash::DISPLAY_BACKWARD`] for more details.
        pub struct Txid(sha256d::Hash);

        /// A bitcoin witness transaction ID.
        pub struct Wtxid(sha256d::Hash);

        /// Filter hash, as defined in BIP-157
        pub struct FilterHash(sha256d::Hash);
        /// Filter header, as defined in BIP-157
        pub struct FilterHeader(sha256d::Hash);
    }

    impl_hashencode!(Txid);
    impl_hashencode!(Wtxid);

    impl_hashencode!(FilterHash);
    impl_hashencode!(FilterHeader);
}

#[deprecated(since = "0.0.0-NEXT-RELEASE", note = "use crate::T instead")]
pub use crate::{BlockHash, TxMerkleNode, WitnessCommitment, WitnessMerkleNode};
