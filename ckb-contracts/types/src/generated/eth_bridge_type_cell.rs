// Generated by Molecule 0.6.0

use super::basic::*;
use molecule::prelude::*;
#[derive(Clone)]
pub struct ETHBridgeTypeArgs(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for ETHBridgeTypeArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for ETHBridgeTypeArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for ETHBridgeTypeArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "bridge_lock_hash", self.bridge_lock_hash())?;
        write!(
            f,
            ", {}: {}",
            "recipient_lock_hash",
            self.recipient_lock_hash()
        )?;
        write!(f, ", {}: {}", "owner_lock_hash", self.owner_lock_hash())?;
        write!(f, ", {}: {}", "fee", self.fee())?;
        write!(f, " }}")
    }
}
impl ::core::default::Default for ETHBridgeTypeArgs {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        ETHBridgeTypeArgs::new_unchecked(v.into())
    }
}
impl ETHBridgeTypeArgs {
    pub const TOTAL_SIZE: usize = 112;
    pub const FIELD_SIZES: [usize; 4] = [32, 32, 32, 16];
    pub const FIELD_COUNT: usize = 4;
    pub fn bridge_lock_hash(&self) -> Byte32 {
        Byte32::new_unchecked(self.0.slice(0..32))
    }
    pub fn recipient_lock_hash(&self) -> Byte32 {
        Byte32::new_unchecked(self.0.slice(32..64))
    }
    pub fn owner_lock_hash(&self) -> Byte32 {
        Byte32::new_unchecked(self.0.slice(64..96))
    }
    pub fn fee(&self) -> Uint128 {
        Uint128::new_unchecked(self.0.slice(96..112))
    }
    pub fn as_reader<'r>(&'r self) -> ETHBridgeTypeArgsReader<'r> {
        ETHBridgeTypeArgsReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for ETHBridgeTypeArgs {
    type Builder = ETHBridgeTypeArgsBuilder;
    const NAME: &'static str = "ETHBridgeTypeArgs";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        ETHBridgeTypeArgs(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ETHBridgeTypeArgsReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ETHBridgeTypeArgsReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .bridge_lock_hash(self.bridge_lock_hash())
            .recipient_lock_hash(self.recipient_lock_hash())
            .owner_lock_hash(self.owner_lock_hash())
            .fee(self.fee())
    }
}
#[derive(Clone, Copy)]
pub struct ETHBridgeTypeArgsReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for ETHBridgeTypeArgsReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for ETHBridgeTypeArgsReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for ETHBridgeTypeArgsReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "bridge_lock_hash", self.bridge_lock_hash())?;
        write!(
            f,
            ", {}: {}",
            "recipient_lock_hash",
            self.recipient_lock_hash()
        )?;
        write!(f, ", {}: {}", "owner_lock_hash", self.owner_lock_hash())?;
        write!(f, ", {}: {}", "fee", self.fee())?;
        write!(f, " }}")
    }
}
impl<'r> ETHBridgeTypeArgsReader<'r> {
    pub const TOTAL_SIZE: usize = 112;
    pub const FIELD_SIZES: [usize; 4] = [32, 32, 32, 16];
    pub const FIELD_COUNT: usize = 4;
    pub fn bridge_lock_hash(&self) -> Byte32Reader<'r> {
        Byte32Reader::new_unchecked(&self.as_slice()[0..32])
    }
    pub fn recipient_lock_hash(&self) -> Byte32Reader<'r> {
        Byte32Reader::new_unchecked(&self.as_slice()[32..64])
    }
    pub fn owner_lock_hash(&self) -> Byte32Reader<'r> {
        Byte32Reader::new_unchecked(&self.as_slice()[64..96])
    }
    pub fn fee(&self) -> Uint128Reader<'r> {
        Uint128Reader::new_unchecked(&self.as_slice()[96..112])
    }
}
impl<'r> molecule::prelude::Reader<'r> for ETHBridgeTypeArgsReader<'r> {
    type Entity = ETHBridgeTypeArgs;
    const NAME: &'static str = "ETHBridgeTypeArgsReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        ETHBridgeTypeArgsReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], _compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len != Self::TOTAL_SIZE {
            return ve!(Self, TotalSizeNotMatch, Self::TOTAL_SIZE, slice_len);
        }
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct ETHBridgeTypeArgsBuilder {
    pub(crate) bridge_lock_hash: Byte32,
    pub(crate) recipient_lock_hash: Byte32,
    pub(crate) owner_lock_hash: Byte32,
    pub(crate) fee: Uint128,
}
impl ETHBridgeTypeArgsBuilder {
    pub const TOTAL_SIZE: usize = 112;
    pub const FIELD_SIZES: [usize; 4] = [32, 32, 32, 16];
    pub const FIELD_COUNT: usize = 4;
    pub fn bridge_lock_hash(mut self, v: Byte32) -> Self {
        self.bridge_lock_hash = v;
        self
    }
    pub fn recipient_lock_hash(mut self, v: Byte32) -> Self {
        self.recipient_lock_hash = v;
        self
    }
    pub fn owner_lock_hash(mut self, v: Byte32) -> Self {
        self.owner_lock_hash = v;
        self
    }
    pub fn fee(mut self, v: Uint128) -> Self {
        self.fee = v;
        self
    }
}
impl molecule::prelude::Builder for ETHBridgeTypeArgsBuilder {
    type Entity = ETHBridgeTypeArgs;
    const NAME: &'static str = "ETHBridgeTypeArgsBuilder";
    fn expected_length(&self) -> usize {
        Self::TOTAL_SIZE
    }
    fn write<W: ::molecule::io::Write>(&self, writer: &mut W) -> ::molecule::io::Result<()> {
        writer.write_all(self.bridge_lock_hash.as_slice())?;
        writer.write_all(self.recipient_lock_hash.as_slice())?;
        writer.write_all(self.owner_lock_hash.as_slice())?;
        writer.write_all(self.fee.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        ETHBridgeTypeArgs::new_unchecked(inner.into())
    }
}