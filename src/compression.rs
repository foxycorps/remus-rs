use crate::ProtocolError;
use bytes::Bytes;
use std::io::prelude::*;
use zstd;

pub fn compress(data: &[u8]) -> Result<Vec<u8>, ProtocolError> {
    let mut encoder = zstd::Encoder::new(Vec::new(), 3)?;
    encoder.write_all(data)?;
    Ok(encoder.finish()?)
}

pub fn decompress(data: &[u8]) -> Result<Vec<u8>, ProtocolError> {
    let mut decoder = zstd::Decoder::new(data)?;
    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf)?;
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_roundtrip() {
        // Create highly compressible data
        let original: Vec<u8> = (0..1000).map(|i| (i % 10) as u8).collect();
        
        let compressed = compress(&original).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        
        assert_eq!(original, decompressed);
        assert!(compressed.len() < original.len(), 
            "Compressed size {} should be less than original size {}", 
            compressed.len(), original.len());
    }

    #[test]
    fn test_compression_large_data() {
        let original: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();
        
        let compressed = compress(&original).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        
        assert_eq!(original, decompressed);
    }

    #[test]
    fn test_compression_empty() {
        let original = vec![];
        
        let compressed = compress(&original).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        
        assert_eq!(original, decompressed);
    }
}

// Helper functions
pub fn compress_if_beneficial(data: &[u8]) -> Result<Bytes, ProtocolError> {
    let compressed = compress(data)?;
    if compressed.len() < data.len() {
        Ok(Bytes::from(compressed))
    } else {
        Ok(Bytes::from(data.to_vec()))
    }
}