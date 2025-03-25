use byteorder::ReadBytesExt;
use std::io::{BufReader, Read};

//
// Read a ClickHouse columnar compact data file
// The file contains a string and an integer
//
// References to ClickHouse source code:
//
// MergeTreeReadTask::BlockAndProgress MergeTreeReadTask::read()
// MergeTreeReadersChain::ReadResult MergeTreeReadersChain::read(size_t max_rows, MarkRanges & ranges)
// MergeTreeRangeReader::ReadResult MergeTreeRangeReader::startReadingChain
// size_t MergeTreeRangeReader::Stream::finalize
// size_t MergeTreeRangeReader::DelayedStream::finalize
// size_t MergeTreeRangeReader::DelayedStream::readRows
// size_t MergeTreeReaderCompactSingleBuffer::readRows
// void MergeTreeReaderCompact::readData
// void ISerialization::deserializeBinaryBulkWithMultipleStreams(
// static NO_INLINE void deserializeBinarySSE2 (SerializationString.cpp)
// DB::ReadBuffer::next()
// bool CompressedReadBufferFromFile::nextImpl()
// size_t CompressedReadBufferBase::readCompressedData(
//     size_t & size_decompressed,
//     size_t & size_compressed_without_checksum, bool always_copy)
//
//
fn main() -> anyhow::Result<()> {
    let ch_root = "/opt/clickhouse/clickhouse/store";
    let part_name = "249f846c-5c77-46db-b7cf-13414588f729";

    // Part data files. In compact format, all the data is stored in a single file
    // called `data.bin`.
    let filename = format!("{}/249/{}/all_1_1_0/data.bin", ch_root, part_name);
    // let filename = format!("{}/249/{}/all_2_2_0/data.bin", ch_root, part_name);

    println!("Reading ClickHouse columnar data from {}", filename);

    let file = std::fs::File::open(filename)?;
    println!("File length: {}", file.metadata()?.len());
    let mut reader = BufReader::new(file);

    _read_compressed_data(&mut reader)?;

    Ok(())
}

fn _read_compressed_data(reader: &mut BufReader<std::fs::File>) -> anyhow::Result<()> {
    // 128 bits for checksum (CityHash64)
    let _size_of_checksum = 16;
    // 1 byte for compression method,
    // 4 bytes for compressed size,
    // 4 bytes for uncompressed size */
    let _header_size = 0x9;

    println!();
    println!("Reading first data block");
    let _compression_info = __read_header_and_get_codec_and_size(reader)?;
    // __validate_checksum(&[])?;

    // Create a buffer sized to:
    // sizeof(Checksum) + size_compressed_without_checksum + additional_size_at_the_end_of_buffer
    // Compressed buffer starts after the checksum
    _read_string(reader)?;

    println!();
    println!("Reading second data block");

    let _compression_info = __read_header_and_get_codec_and_size(reader)?;
    // __validate_checksum(&[])?;
    _read_int(reader, &_compression_info)?;

    Ok(())
}

//
// CompressedReadBufferBase.cpp
fn __read_header_and_get_codec_and_size(
    reader: &mut BufReader<std::fs::File>,
) -> anyhow::Result<CompressionInfo> {
    // skip the 16 bytes of checksum
    // reader.seek(std::io::SeekFrom::Current(16))?;
    //
    // Read 16 bytes of checksum
    let mut buffer = [0u8; 16];
    reader.read_exact(&mut buffer)?;
    let checksum = buffer;

    let mut buffer = [0u8; 1];
    reader.read_exact(&mut buffer)?;
    let compression_method = CompressMethod::from_u8(buffer[0]);
    println!("Compression method: {:?}", compression_method);

    let mut buffer = [0u8; 4];
    reader.read_exact(&mut buffer)?;
    let size_compressed_without_checksum = u32::from_le_bytes(buffer);
    println!(
        "Compressed size without checksum: {}",
        size_compressed_without_checksum
    );

    let mut buffer = [0u8; 4];
    reader.read_exact(&mut buffer)?;
    let size_decompressed = u32::from_le_bytes(buffer);
    println!("Decompressed size: {}", size_decompressed);

    // Read additional bytes based on the compression method
    // Some codecs like (LZ4 for example), require additional bytes at
    // end of buffer
    let _additional_size_at_the_end_of_buffer = 0;

    let compression_info = CompressionInfo {
        checksum,
        compression_method,
        size_compressed: size_compressed_without_checksum,
        size_decompressed,
    };

    Ok(compression_info)
}

fn _read_string(reader: &mut BufReader<std::fs::File>) -> anyhow::Result<()> {
    // read past the header of 9 bytes
    // amount to read: size_compressed_without_checksum - header_size
    // let _n = size_compressed_without_checksum as usize - header_size;
    // println!("Reading data block of size: {}", _n);
    // The first unsigned byte is the length of the string
    let len = reader.read_u8()?;
    let mut buffer = vec![0u8; len as usize];
    reader.read_exact(&mut buffer)?;
    let s = std::str::from_utf8(&buffer)?;
    println!("String: {}", s);

    Ok(())
}

fn _read_int(
    reader: &mut BufReader<std::fs::File>,
    compression_info: &CompressionInfo,
) -> anyhow::Result<()> {
    let mut buffer = vec![0u8; compression_info.size_decompressed as usize];
    reader.read_exact(&mut buffer)?;
    let n = u32::from_le_bytes(buffer.try_into().unwrap());
    println!("Int: {}", n);
    Ok(())
}

// Validate checksum of data, and if it mismatches, find out possible reason
// and return an error.
fn __validate_checksum(_data: &[u8]) -> anyhow::Result<()> {
    // calculate CityHash128 of the data
    todo!()
}

pub struct CompressionInfo {
    pub checksum: [u8; 16],
    pub compression_method: CompressMethod,
    pub size_compressed: u32,
    pub size_decompressed: u32,
}

//  src/Compression/CompressionInfo.h
//  The first 16 bytes are the checksum from all other bytes of the block.
//  Now only CityHash128 is used.
//
//  The next byte specifies the compression algorithm. Then everything depends
//  on the algorithm. All sizes are little endian.
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum CompressMethod {
    NONE = 0x02,
    LZ4 = 0x82,
    ZSTD = 0x90,
    Multiple = 0x91,
    Delta = 0x92,
    T64 = 0x93,
    DoubleDelta = 0x94,
    Gorilla = 0x95,
    AES_128_GCM_SIV = 0x96,
    AES_256_GCM_SIV = 0x97,
    FPC = 0x98,
    DeflateQpl = 0x99,
    GCD = 0x9a,
    ZSTD_QPL = 0x9b,
}
impl CompressMethod {
    fn from_u8(buffer: u8) -> CompressMethod {
        match buffer {
            0x02 => CompressMethod::NONE,
            0x82 => CompressMethod::LZ4,
            0x90 => CompressMethod::ZSTD,
            0x91 => CompressMethod::Multiple,
            0x92 => CompressMethod::Delta,
            0x93 => CompressMethod::T64,
            0x94 => CompressMethod::DoubleDelta,
            0x95 => CompressMethod::Gorilla,
            0x96 => CompressMethod::AES_128_GCM_SIV,
            0x97 => CompressMethod::AES_256_GCM_SIV,
            0x98 => CompressMethod::FPC,
            0x99 => CompressMethod::DeflateQpl,
            0x9a => CompressMethod::GCD,
            0x9b => CompressMethod::ZSTD_QPL,
            _ => panic!("Unknown compression method"),
        }
    }
}

// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {

    use cityhash_sys::city_hash_64;

    #[test]
    fn test_city_hash() {
        let city = "Moscow";
        let hash = city_hash_64(city.as_bytes());
        println!("City: {}, Hash: {:?}", city, hash);
    }
}
