//! Cryptographic type definitions and primitives supported in Farcaster

/// This trait is defined for blockchains once per cryptographic engine wanted and allow a
/// blockchain to use different cryptographic types depending on the engine used.
///
/// E.g. ECDSA and Schnorr signature in Bitcoin are stored/parsed differently as Schnorr has been
/// optimized further than ECDSA at the begining of Bitcoin.
pub trait Crypto<C: CryptoEngine> {
    /// Private key type given the blockchain and the crypto engine
    type PrivateKey;

    /// Public key type given the blockchain and the crypto engine
    type PublicKey;

    /// Commitment type given the blockchain and the crypto engine
    type Commitment;

    /// Defines the signature format for the arbitrating blockchain
    type Signature;
}

/// Defines a type of cryptography used inside arbitrating transactions to validate the
/// transactions at the blockchain level and transfert the secrets.
pub trait CryptoEngine {}

/// Uses ECDSA signatures inside the scripting layer of the arbitrating blockchain.
pub struct ECDSAScripts;

impl CryptoEngine for ECDSAScripts {}

/// Uses Schnorr signatures inside the scripting layer of the arbitrating blockchain.
pub struct TrSchnorrScripts;

impl CryptoEngine for TrSchnorrScripts {}

/// Uses MuSig2 Schnorr off-chain multi-signature protocol to sign for a regular public key at the
/// blockchain transaction layer.
pub struct TrMuSig2;

impl CryptoEngine for TrMuSig2 {}
