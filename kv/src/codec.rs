use bytes::{Buf, BufMut};

pub struct ProstCodec<T, U> {
    _marker: std::marker::PhantomData<(T, U)>,
}

impl<T, U> Default for ProstCodec<T, U> {
    fn default() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: prost::Message, U> tokio_util::codec::Encoder<T> for ProstCodec<T, U> {
    type Error = std::io::Error;
    fn encode(&mut self, item: T, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        let len = item.encoded_len();
        dst.reserve(4 + len);
        dst.put_u32(len as u32);
        item.encode(dst).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to encode {}", e),
            )
        })?;
        Ok(())
    }
}

impl<T, U: prost::Message + Default> tokio_util::codec::Decoder for ProstCodec<T, U> {
    type Item = U;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            return Ok(None);
        }

        let len = u32::from_be_bytes([src[0], src[1], src[2], src[3]]) as usize;
        if src.len() < 4 + len {
            return Ok(None);
        }

        src.advance(4);

        let msg_bytes = src.split_to(len);

        let msg = U::decode(&mut &msg_bytes[..]).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to decode for message: {}", e),
            )
        })?;

        Ok(Some(msg))
    }
}
