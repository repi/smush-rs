pub fn encode_data(data: &[u8]) -> std::io::Result<Vec<u8>> {
    match bincode::serialize(&data[..]) {
        Err(err) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to encode with bincode - details: {:?}", err),
        )),
        Ok(buf) => Ok(buf),
    }
}

pub fn decode_data(data: &[u8]) -> std::io::Result<Vec<u8>> {
    match bincode::deserialize(&data[..]) {
        Err(err) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to decode with bincode - details: {:?}", err),
        )),
        Ok(buf) => Ok(buf),
    }
}
