// Generated by Molecule 0.6.0

use super::basic::*;
use molecule::prelude::*;
#[derive(Clone)]
pub struct MintTokenWitness(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for MintTokenWitness {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for MintTokenWitness {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for MintTokenWitness {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "mode", self.mode())?;
        write!(f, ", {}: {}", "spv_proof", self.spv_proof())?;
        write!(
            f,
            ", {}: {}",
            "cell_dep_index_list",
            self.cell_dep_index_list()
        )?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl ::core::default::Default for MintTokenWitness {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            25, 0, 0, 0, 16, 0, 0, 0, 17, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        MintTokenWitness::new_unchecked(v.into())
    }
}
impl MintTokenWitness {
    pub const FIELD_COUNT: usize = 3;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn mode(&self) -> Byte {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        Byte::new_unchecked(self.0.slice(start..end))
    }
    pub fn spv_proof(&self) -> Bytes {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        Bytes::new_unchecked(self.0.slice(start..end))
    }
    pub fn cell_dep_index_list(&self) -> Bytes {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[16..]) as usize;
            Bytes::new_unchecked(self.0.slice(start..end))
        } else {
            Bytes::new_unchecked(self.0.slice(start..))
        }
    }
    pub fn as_reader<'r>(&'r self) -> MintTokenWitnessReader<'r> {
        MintTokenWitnessReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for MintTokenWitness {
    type Builder = MintTokenWitnessBuilder;
    const NAME: &'static str = "MintTokenWitness";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        MintTokenWitness(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        MintTokenWitnessReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        MintTokenWitnessReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .mode(self.mode())
            .spv_proof(self.spv_proof())
            .cell_dep_index_list(self.cell_dep_index_list())
    }
}
#[derive(Clone, Copy)]
pub struct MintTokenWitnessReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for MintTokenWitnessReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for MintTokenWitnessReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for MintTokenWitnessReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "mode", self.mode())?;
        write!(f, ", {}: {}", "spv_proof", self.spv_proof())?;
        write!(
            f,
            ", {}: {}",
            "cell_dep_index_list",
            self.cell_dep_index_list()
        )?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl<'r> MintTokenWitnessReader<'r> {
    pub const FIELD_COUNT: usize = 3;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn mode(&self) -> ByteReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        ByteReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn spv_proof(&self) -> BytesReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        BytesReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn cell_dep_index_list(&self) -> BytesReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[16..]) as usize;
            BytesReader::new_unchecked(&self.as_slice()[start..end])
        } else {
            BytesReader::new_unchecked(&self.as_slice()[start..])
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for MintTokenWitnessReader<'r> {
    type Entity = MintTokenWitness;
    const NAME: &'static str = "MintTokenWitnessReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        MintTokenWitnessReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let total_size = molecule::unpack_number(slice) as usize;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        if slice_len == molecule::NUMBER_SIZE && Self::FIELD_COUNT == 0 {
            return Ok(());
        }
        if slice_len < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE * 2, slice_len);
        }
        let offset_first = molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]) as usize;
        if offset_first % 4 != 0 || offset_first < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, OffsetsNotMatch);
        }
        let field_count = offset_first / 4 - 1;
        if field_count < Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        } else if !compatible && field_count > Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        };
        let header_size = molecule::NUMBER_SIZE * (field_count + 1);
        if slice_len < header_size {
            return ve!(Self, HeaderIsBroken, header_size, slice_len);
        }
        let mut offsets: Vec<usize> = slice[molecule::NUMBER_SIZE..]
            .chunks(molecule::NUMBER_SIZE)
            .take(field_count)
            .map(|x| molecule::unpack_number(x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            return ve!(Self, OffsetsNotMatch);
        }
        ByteReader::verify(&slice[offsets[0]..offsets[1]], compatible)?;
        BytesReader::verify(&slice[offsets[1]..offsets[2]], compatible)?;
        BytesReader::verify(&slice[offsets[2]..offsets[3]], compatible)?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct MintTokenWitnessBuilder {
    pub(crate) mode: Byte,
    pub(crate) spv_proof: Bytes,
    pub(crate) cell_dep_index_list: Bytes,
}
impl MintTokenWitnessBuilder {
    pub const FIELD_COUNT: usize = 3;
    pub fn mode(mut self, v: Byte) -> Self {
        self.mode = v;
        self
    }
    pub fn spv_proof(mut self, v: Bytes) -> Self {
        self.spv_proof = v;
        self
    }
    pub fn cell_dep_index_list(mut self, v: Bytes) -> Self {
        self.cell_dep_index_list = v;
        self
    }
}
impl molecule::prelude::Builder for MintTokenWitnessBuilder {
    type Entity = MintTokenWitness;
    const NAME: &'static str = "MintTokenWitnessBuilder";
    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1)
            + self.mode.as_slice().len()
            + self.spv_proof.as_slice().len()
            + self.cell_dep_index_list.as_slice().len()
    }
    fn write<W: ::molecule::io::Write>(&self, writer: &mut W) -> ::molecule::io::Result<()> {
        let mut total_size = molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1);
        let mut offsets = Vec::with_capacity(Self::FIELD_COUNT);
        offsets.push(total_size);
        total_size += self.mode.as_slice().len();
        offsets.push(total_size);
        total_size += self.spv_proof.as_slice().len();
        offsets.push(total_size);
        total_size += self.cell_dep_index_list.as_slice().len();
        writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
        for offset in offsets.into_iter() {
            writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
        }
        writer.write_all(self.mode.as_slice())?;
        writer.write_all(self.spv_proof.as_slice())?;
        writer.write_all(self.cell_dep_index_list.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        MintTokenWitness::new_unchecked(inner.into())
    }
}
#[derive(Clone)]
pub struct ETHSPVProof(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for ETHSPVProof {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for ETHSPVProof {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for ETHSPVProof {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "log_index", self.log_index())?;
        write!(f, ", {}: {}", "log_entry_data", self.log_entry_data())?;
        write!(f, ", {}: {}", "receipt_index", self.receipt_index())?;
        write!(f, ", {}: {}", "receipt_data", self.receipt_data())?;
        write!(f, ", {}: {}", "header_data", self.header_data())?;
        write!(f, ", {}: {}", "proof", self.proof())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl ::core::default::Default for ETHSPVProof {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            60, 0, 0, 0, 28, 0, 0, 0, 36, 0, 0, 0, 40, 0, 0, 0, 48, 0, 0, 0, 52, 0, 0, 0, 56, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            4, 0, 0, 0,
        ];
        ETHSPVProof::new_unchecked(v.into())
    }
}
impl ETHSPVProof {
    pub const FIELD_COUNT: usize = 6;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn log_index(&self) -> Uint64 {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        Uint64::new_unchecked(self.0.slice(start..end))
    }
    pub fn log_entry_data(&self) -> Bytes {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        Bytes::new_unchecked(self.0.slice(start..end))
    }
    pub fn receipt_index(&self) -> Uint64 {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        let end = molecule::unpack_number(&slice[16..]) as usize;
        Uint64::new_unchecked(self.0.slice(start..end))
    }
    pub fn receipt_data(&self) -> Bytes {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[16..]) as usize;
        let end = molecule::unpack_number(&slice[20..]) as usize;
        Bytes::new_unchecked(self.0.slice(start..end))
    }
    pub fn header_data(&self) -> Bytes {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[20..]) as usize;
        let end = molecule::unpack_number(&slice[24..]) as usize;
        Bytes::new_unchecked(self.0.slice(start..end))
    }
    pub fn proof(&self) -> BytesVec {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[24..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[28..]) as usize;
            BytesVec::new_unchecked(self.0.slice(start..end))
        } else {
            BytesVec::new_unchecked(self.0.slice(start..))
        }
    }
    pub fn as_reader<'r>(&'r self) -> ETHSPVProofReader<'r> {
        ETHSPVProofReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for ETHSPVProof {
    type Builder = ETHSPVProofBuilder;
    const NAME: &'static str = "ETHSPVProof";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        ETHSPVProof(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ETHSPVProofReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ETHSPVProofReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .log_index(self.log_index())
            .log_entry_data(self.log_entry_data())
            .receipt_index(self.receipt_index())
            .receipt_data(self.receipt_data())
            .header_data(self.header_data())
            .proof(self.proof())
    }
}
#[derive(Clone, Copy)]
pub struct ETHSPVProofReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for ETHSPVProofReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for ETHSPVProofReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for ETHSPVProofReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "log_index", self.log_index())?;
        write!(f, ", {}: {}", "log_entry_data", self.log_entry_data())?;
        write!(f, ", {}: {}", "receipt_index", self.receipt_index())?;
        write!(f, ", {}: {}", "receipt_data", self.receipt_data())?;
        write!(f, ", {}: {}", "header_data", self.header_data())?;
        write!(f, ", {}: {}", "proof", self.proof())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl<'r> ETHSPVProofReader<'r> {
    pub const FIELD_COUNT: usize = 6;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn log_index(&self) -> Uint64Reader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn log_entry_data(&self) -> BytesReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        BytesReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn receipt_index(&self) -> Uint64Reader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        let end = molecule::unpack_number(&slice[16..]) as usize;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn receipt_data(&self) -> BytesReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[16..]) as usize;
        let end = molecule::unpack_number(&slice[20..]) as usize;
        BytesReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn header_data(&self) -> BytesReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[20..]) as usize;
        let end = molecule::unpack_number(&slice[24..]) as usize;
        BytesReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn proof(&self) -> BytesVecReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[24..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[28..]) as usize;
            BytesVecReader::new_unchecked(&self.as_slice()[start..end])
        } else {
            BytesVecReader::new_unchecked(&self.as_slice()[start..])
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for ETHSPVProofReader<'r> {
    type Entity = ETHSPVProof;
    const NAME: &'static str = "ETHSPVProofReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        ETHSPVProofReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let total_size = molecule::unpack_number(slice) as usize;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        if slice_len == molecule::NUMBER_SIZE && Self::FIELD_COUNT == 0 {
            return Ok(());
        }
        if slice_len < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE * 2, slice_len);
        }
        let offset_first = molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]) as usize;
        if offset_first % 4 != 0 || offset_first < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, OffsetsNotMatch);
        }
        let field_count = offset_first / 4 - 1;
        if field_count < Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        } else if !compatible && field_count > Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        };
        let header_size = molecule::NUMBER_SIZE * (field_count + 1);
        if slice_len < header_size {
            return ve!(Self, HeaderIsBroken, header_size, slice_len);
        }
        let mut offsets: Vec<usize> = slice[molecule::NUMBER_SIZE..]
            .chunks(molecule::NUMBER_SIZE)
            .take(field_count)
            .map(|x| molecule::unpack_number(x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            return ve!(Self, OffsetsNotMatch);
        }
        Uint64Reader::verify(&slice[offsets[0]..offsets[1]], compatible)?;
        BytesReader::verify(&slice[offsets[1]..offsets[2]], compatible)?;
        Uint64Reader::verify(&slice[offsets[2]..offsets[3]], compatible)?;
        BytesReader::verify(&slice[offsets[3]..offsets[4]], compatible)?;
        BytesReader::verify(&slice[offsets[4]..offsets[5]], compatible)?;
        BytesVecReader::verify(&slice[offsets[5]..offsets[6]], compatible)?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct ETHSPVProofBuilder {
    pub(crate) log_index: Uint64,
    pub(crate) log_entry_data: Bytes,
    pub(crate) receipt_index: Uint64,
    pub(crate) receipt_data: Bytes,
    pub(crate) header_data: Bytes,
    pub(crate) proof: BytesVec,
}
impl ETHSPVProofBuilder {
    pub const FIELD_COUNT: usize = 6;
    pub fn log_index(mut self, v: Uint64) -> Self {
        self.log_index = v;
        self
    }
    pub fn log_entry_data(mut self, v: Bytes) -> Self {
        self.log_entry_data = v;
        self
    }
    pub fn receipt_index(mut self, v: Uint64) -> Self {
        self.receipt_index = v;
        self
    }
    pub fn receipt_data(mut self, v: Bytes) -> Self {
        self.receipt_data = v;
        self
    }
    pub fn header_data(mut self, v: Bytes) -> Self {
        self.header_data = v;
        self
    }
    pub fn proof(mut self, v: BytesVec) -> Self {
        self.proof = v;
        self
    }
}
impl molecule::prelude::Builder for ETHSPVProofBuilder {
    type Entity = ETHSPVProof;
    const NAME: &'static str = "ETHSPVProofBuilder";
    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1)
            + self.log_index.as_slice().len()
            + self.log_entry_data.as_slice().len()
            + self.receipt_index.as_slice().len()
            + self.receipt_data.as_slice().len()
            + self.header_data.as_slice().len()
            + self.proof.as_slice().len()
    }
    fn write<W: ::molecule::io::Write>(&self, writer: &mut W) -> ::molecule::io::Result<()> {
        let mut total_size = molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1);
        let mut offsets = Vec::with_capacity(Self::FIELD_COUNT);
        offsets.push(total_size);
        total_size += self.log_index.as_slice().len();
        offsets.push(total_size);
        total_size += self.log_entry_data.as_slice().len();
        offsets.push(total_size);
        total_size += self.receipt_index.as_slice().len();
        offsets.push(total_size);
        total_size += self.receipt_data.as_slice().len();
        offsets.push(total_size);
        total_size += self.header_data.as_slice().len();
        offsets.push(total_size);
        total_size += self.proof.as_slice().len();
        writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
        for offset in offsets.into_iter() {
            writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
        }
        writer.write_all(self.log_index.as_slice())?;
        writer.write_all(self.log_entry_data.as_slice())?;
        writer.write_all(self.receipt_index.as_slice())?;
        writer.write_all(self.receipt_data.as_slice())?;
        writer.write_all(self.header_data.as_slice())?;
        writer.write_all(self.proof.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        ETHSPVProof::new_unchecked(inner.into())
    }
}
