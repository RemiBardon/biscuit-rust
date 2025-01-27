//! error types
//!

use std::convert::{From, Infallible};
use thiserror::Error;

/// the global error type for Biscuit
#[derive(Error, Clone, Debug, PartialEq)]
pub enum Token {
    #[error("internal error")]
    InternalError,
    #[error("error deserializing or verifying the token")]
    Format(Format),
    #[error("the authority block must have the index 0")]
    InvalidAuthorityIndex(u32),
    #[error("the block index does not match its position")]
    InvalidBlockIndex(InvalidBlockIndex),
    #[error("multiple blocks declare the same symbols")]
    SymbolTableOverlap,
    #[error(r#"the symbol table is missing either "authority" or "ambient""#)]
    MissingSymbols,
    #[error("tried to append a block to a sealed token")]
    Sealed,
    #[error("check validation failed")]
    FailedLogic(Logic),
    #[error("Datalog parsing error")]
    ParseError,
    #[error("Reached Datalog execution limits")]
    RunLimit(RunLimit),
    #[error("Cannot convert from Term: %s")]
    ConversionError(String),
    #[error("Cannot decode base64 token: %s")]
    Base64(base64::DecodeError),
}

impl From<Infallible> for Token {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl From<Format> for Token {
    fn from(e: Format) -> Self {
        Token::Format(e)
    }
}

impl From<Logic> for Token {
    fn from(e: Logic) -> Self {
        Token::FailedLogic(e)
    }
}

impl From<base64::DecodeError> for Token {
    fn from(e: base64::DecodeError) -> Self {
        Token::Base64(e)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InvalidBlockIndex {
    pub expected: u32,
    pub found: u32,
}

/// Errors related to the token's serialization format or cryptographic
/// signature
#[derive(Error, Clone, Debug, PartialEq)]
pub enum Format {
    #[error("failed verifying the signature")]
    Signature(Signature),
    #[error("failed verifying the signature of a sealed token")]
    SealedSignature,
    #[error("the token does not provide intermediate public keys")]
    EmptyKeys,
    #[error("the root public key was not recognized")]
    UnknownPublicKey,
    #[error("could not deserialize the wrapper object")]
    DeserializationError(String),
    #[error("could not serialize the wrapper object")]
    SerializationError(String),
    #[error("could not deserialize the block")]
    BlockDeserializationError(String),
    #[error("could not serialize the block")]
    BlockSerializationError(String),
    #[error("Block format version is higher than supported")]
    Version { maximum: u32, actual: u32 },
}

/// Signature errors
#[derive(Error, Clone, Debug, PartialEq)]
pub enum Signature {
    #[error("could not parse the signature elements")]
    InvalidFormat,
    #[error("the signature did not match")]
    InvalidSignature,
}

/// errors in the Datalog evaluation
#[derive(Error, Clone, Debug, PartialEq)]
pub enum Logic {
    #[error("a fact of the authority block did not have the authority tag")]
    InvalidAuthorityFact(String),
    #[error("a fact provided or generated by the verifier did not have the ambient tag")]
    InvalidAmbientFact(String),
    #[error("a fact provided or generated by a block had the authority or ambient tag")]
    InvalidBlockFact(u32, String),
    #[error("a rule provided by a block is generating facts with the authority or ambient tag, or has head variables not used in its body")]
    InvalidBlockRule(u32, String),
    #[error("list of checks that failed validation")]
    FailedChecks(Vec<FailedCheck>),
    #[error("the verifier already contains a token")]
    VerifierNotEmpty,
    #[error("denied by policy")]
    Deny(usize),
    #[error("no matching policy was found")]
    NoMatchingPolicy,
}

/// check check errors
#[derive(Error, Clone, Debug, PartialEq)]
pub enum FailedCheck {
    #[error("a check failed in a block")]
    Block(FailedBlockCheck),
    #[error("a check provided by the verifier failed")]
    Verifier(FailedVerifierCheck),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FailedBlockCheck {
    pub block_id: u32,
    pub check_id: u32,
    /// pretty print of the rule that failed
    pub rule: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FailedVerifierCheck {
    pub check_id: u32,
    /// pretty print of the rule that failed
    pub rule: String,
}

/// runtime limits errors
#[derive(Error, Clone, Debug, PartialEq)]
pub enum RunLimit {
    #[error("too many facts generated")]
    TooManyFacts,
    #[error("too many engine iterations")]
    TooManyIterations,
    #[error("spent too much time verifying")]
    Timeout,
}
