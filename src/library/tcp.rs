use bytes::{Bytes, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

//------------------------------------------------------------------------------------------------
//  RequestChunks
//------------------------------------------------------------------------------------------------

pub struct RequestChunksDecoder {
    body_len: Option<u32>,
}

impl RequestChunksDecoder {
    pub fn new() -> Self {
        Self { body_len: None }
    }
}

impl Decoder for RequestChunksDecoder {
    type Item = BytesMut;

    type Error = eyre::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        println!("Incoming tcp-packet...");
        if self.body_len.is_none() {
            if src.len() < 4 {
                return Ok(None);
            }
            let bytes = src.split_to(4).as_ref().try_into().unwrap();
            self.body_len = Some(u32::from_le_bytes(bytes));
            println!("Body length decoded. It is is: {}", self.body_len.unwrap());
        }

        let body_len = self.body_len.unwrap() as usize;
        if src.len() < body_len {
            println!("Error: Body was smaller than the length {body_len} decoded");
            return Ok(None);
        }

        let rlp = src.split_to(body_len);
        let _body_len = self.body_len.take().unwrap();
        Ok(Some(rlp))
    }
}

pub struct RequestChunksEncoder;

impl Encoder<&Bytes> for RequestChunksEncoder {
    type Error = eyre::Error;

    fn encode(&mut self, item: &Bytes, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let body_len: [u8; 4] = (item.len() as u32).to_le_bytes();

        dst.reserve(4 + item.len());
        dst.extend_from_slice(&body_len);
        dst.extend_from_slice(item);

        Ok(())
    }
}

//------------------------------------------------------------------------------------------------
//  SendChunks
//------------------------------------------------------------------------------------------------

pub struct SendChunksDecoder {
    start_chunk_id: Option<u32>,
    body_len: Option<u32>,
}

impl SendChunksDecoder {
    pub fn new() -> Self {
        Self {
            body_len: None,
            start_chunk_id: None,
        }
    }
}

impl Decoder for SendChunksDecoder {
    /// The start-chunk-id and the chunk-bytes
    type Item = (u32, BytesMut);
    type Error = eyre::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if self.start_chunk_id.is_none() {
            if src.len() < 4 {
                return Ok(None);
            }
            self.start_chunk_id = Some(u32::from_le_bytes(
                src.split_to(4).as_ref().try_into().unwrap(),
            ));
        };

        if self.body_len.is_none() {
            if src.len() < 4 {
                return Ok(None);
            }
            self.body_len = Some(u32::from_le_bytes(
                src.split_to(4).as_ref().try_into().unwrap(),
            ));
        }

        let body_len = self.body_len.unwrap() as usize;
        if src.len() < body_len {
            return Ok(None);
        }

        let chunks = src.split_to(body_len);
        let start_chunk_id = self.start_chunk_id.take().unwrap();
        let _body_len = self.body_len.take().unwrap();
        Ok(Some((start_chunk_id, chunks)))
    }
}

pub struct SendChunksEncoder;

impl Encoder<(u32, &Bytes)> for SendChunksEncoder {
    type Error = eyre::Error;

    fn encode(&mut self, item: (u32, &Bytes), dst: &mut BytesMut) -> Result<(), Self::Error> {
        let start_chunk_id: [u8; 4] = item.0.to_le_bytes();
        let body_len: [u8; 4] = (item.1.len() as u32).to_le_bytes();

        dst.reserve(8 + item.1.len());
        dst.extend_from_slice(&start_chunk_id);
        dst.extend_from_slice(&body_len);
        dst.extend_from_slice(&item.1);
        Ok(())
    }
}
