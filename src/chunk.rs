use crate::chunk_type::ChunkType;
use anyhow::{anyhow, Error, Result};
use std::fmt::{Display, Formatter};
pub struct Chunk {
    data_length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let data_length = data.len() as u32;
        let crc = Chunk::calculate_crc(&chunk_type, &data);

        Self {
            data_length,
            chunk_type,
            data,
            crc,
        }
    }
    pub fn length(&self) -> u32 {
        self.data_length
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.data.clone()).unwrap_or_else(|_| {
            self.data
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>()
        }))
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect()
    }
    pub fn calculate_crc(chunk_type: &ChunkType, data: &[u8]) -> u32 {
        const CRC_ALG: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let data = chunk_type
            .bytes()
            .to_vec()
            .iter()
            .chain(data.iter())
            .copied()
            .collect::<Vec<_>>();
        CRC_ALG.checksum(data.as_slice())
    }
}
impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut iter = value.iter();

        let data_length_bytes = iter.by_ref().take(4).copied().collect::<Vec<_>>();
        let data_length = u32::from_be_bytes(
            data_length_bytes
                .try_into()
                .map_err(|_| anyhow!("data_length must be four bytes"))?,
        );
        if data_length >= 2u32.pow(31) {
            return Err(anyhow!("data_length exceeds 2^31"));
        }

        let chuck_type_bytes = iter.by_ref().take(4).copied().collect::<Vec<_>>();
        let chunk_type_bytes: [u8; 4] = chuck_type_bytes
            .try_into()
            .map_err(|_| anyhow!("chunk type must be four bytes"))?;
        let chunk_type = ChunkType::try_from(chunk_type_bytes).unwrap();

        let data = iter
            .by_ref()
            .take(data_length as usize)
            .cloned()
            .collect::<Vec<_>>();
        if data.len() != data_length as usize {
            return Err(anyhow!("data_length does not match actual data length"));
        }

        let crc_bytes = iter.by_ref().take(4).cloned().collect::<Vec<_>>();
        let crc = u32::from_be_bytes(
            crc_bytes
                .try_into()
                .map_err(|_| anyhow!("crc must be four bytes"))?,
        );
        let calculated_crc = Chunk::calculate_crc(&chunk_type, &data);
        if crc != calculated_crc {
            return Err(anyhow!(
                "crc does not match calculated crc. expected:{crc} caclulated:{calculated_crc}"
            ));
        }
        Ok(Self {
            data_length,
            chunk_type,
            data,
            crc: calculated_crc,
        })
    }
}
impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Length: {}, Chunk type: {}, Data: {}, Crc: {}",
            self.length(),
            self.chunk_type,
            self.data_as_string().unwrap(),
            self.crc()
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
