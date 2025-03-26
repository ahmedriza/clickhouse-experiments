use byteorder::ReadBytesExt;
use cityhash_clickhouse_sys::u128_low_high::LowHigh;
use std::io::{BufReader, Cursor, Read, Seek};

// In a ClickHouse columnar data file, the first 16 bytes are the checksum.
// The next 9 bytes are the header.
// This is followed by the data (which may be compressed or uncompressed
// depending on the column setting) for the row.
//
// Each row entry is encoded like that.
//
// The checksum is calculated using CityHash128 and uses the header + the
// compressed data bits for the calculation.

// 128 bits for checksum (CityHash128)
const CHECKSUM_SIZE: usize = std::mem::size_of::<u128>();

// 1 byte for compression method,
// 4 bytes for compressed size,
// 4 bytes for uncompressed size */
const __HEADER_SIZE: usize = 0x9;
//
// Read a ClickHouse columnar compact data file
// The file contains a string and an integer
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

    read(&mut reader)?;

    Ok(())
}

fn read(reader: &mut BufReader<std::fs::File>) -> anyhow::Result<()> {
    println!();
    println!("Reading first data block");
    let _compression_info = read_header_and_get_codec_and_size(reader)?;
    // Create a buffer sized to:
    // sizeof(Checksum) + size_compressed_without_checksum + additional_size_at_the_end_of_buffer
    // Compressed buffer starts after the checksum
    read_string(reader)?;
    // validate_checksum(reader, &compression_info)?;

    println!();
    println!("Reading second data block");
    let compression_info = read_header_and_get_codec_and_size(reader)?;
    read_int(reader, &compression_info)?;
    // validate_checksum(reader, &compression_info)?;

    Ok(())
}

//
// CompressedReadBufferBase.cpp
fn read_header_and_get_codec_and_size(
    reader: &mut BufReader<std::fs::File>,
) -> anyhow::Result<CompressionInfo> {
    // Read 16 bytes of checksum
    let mut buffer = [0u8; CHECKSUM_SIZE];
    reader.read_exact(&mut buffer)?;
    let checksum_bytes = buffer;
    println!("{:35} {:x?}", "Checksum:", checksum_bytes);

    let _method = reader.read_u8()?;
    let compression_method = CompressMethod::from_u8(_method);

    // let compression_method = CompressMethod::from_u8(buffer[0]);
    println!("{:35} {:?}", "Compression method:", compression_method);

    // The next 4 bytes are the size of the compressed data
    let mut buffer = [0u8; 4];
    reader.read_exact(&mut buffer)?;
    let size_compressed_without_checksum = u32::from_le_bytes(buffer);
    println!(
        "{:35} {:?}",
        "Compressed size without checksum:", size_compressed_without_checksum
    );

    // The next 4 bytes are the size of the decompressed data
    let mut buffer = [0u8; 4];
    reader.read_exact(&mut buffer)?;
    let size_decompressed = u32::from_le_bytes(buffer);
    println!("{:35} {:?}", "Decompressed size:", size_decompressed);

    // Read additional bytes based on the compression method
    // Some codecs like (LZ4 for example), require additional bytes at
    // end of buffer
    let _additional_size_at_the_end_of_buffer = 0;

    // Seek back to read the compressed data block
    reader.seek(std::io::SeekFrom::Current(-(__HEADER_SIZE as i64)))?;
    let mut data_bytes = vec![0u8; size_compressed_without_checksum as usize];
    reader.read_exact(&mut data_bytes)?;
    __validate_checksum(&data_bytes, &checksum_bytes)?;

    // adjust the reader position to the start of the data block
    // go back to the beginning of the compressed data block
    // and then skip the header
    reader.seek(std::io::SeekFrom::Current(
        -(size_compressed_without_checksum as i64),
    ))?;
    reader.seek(std::io::SeekFrom::Current(__HEADER_SIZE as i64))?;

    let compression_info = CompressionInfo {
        checksum_bytes,
        compression_method,
        size_compressed_without_checksum,
        size_decompressed,
    };

    Ok(compression_info)
}

fn __validate_checksum(data_bytes: &[u8], checksum_bytes: &[u8]) -> anyhow::Result<()> {
    let mut cursor = Cursor::new(checksum_bytes);
    let _low = cursor.read_u64::<byteorder::LittleEndian>()?;
    let _high = cursor.read_u64::<byteorder::LittleEndian>()?;
    println!("{:35} {}, high: {}", "Checksum actual low:", _low, _high);

    let calculated = cityhash_clickhouse_sys::cityhash::city_hash_128(&data_bytes);
    let _calculated_low = calculated.low_half();
    let _calculated_high = calculated.high_half();
    println!(
        "{:35} {}, High: {}",
        "Checksum calculated Low:", _calculated_low, _calculated_high
    );

    Ok(())
}

fn read_string(reader: &mut BufReader<std::fs::File>) -> anyhow::Result<()> {
    // read past the header of 9 bytes
    // amount to read: size_compressed_without_checksum - header_size
    // let _n = size_compressed_without_checksum as usize - header_size;
    // println!("Reading data block of size: {}", _n);
    // The first unsigned byte is the length of the string
    let len = reader.read_u8()?;
    println!("{:35} {}", "Length:", len);
    let mut buffer = vec![0u8; len as usize];
    reader.read_exact(&mut buffer)?;
    let s = std::str::from_utf8(&buffer)?;
    println!("{:35} {}", "String:", s);

    Ok(())
}

fn read_int(
    reader: &mut BufReader<std::fs::File>,
    compression_info: &CompressionInfo,
) -> anyhow::Result<()> {
    let mut buffer = vec![0u8; compression_info.size_decompressed as usize];
    reader.read_exact(&mut buffer)?;
    let n = u32::from_le_bytes(buffer.try_into().unwrap());
    println!("{:35} {}", "Int:", n);
    Ok(())
}

pub struct CompressionInfo {
    pub checksum_bytes: [u8; CHECKSUM_SIZE],
    pub compression_method: CompressMethod,
    pub size_compressed_without_checksum: u32,
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

    use cityhash_clickhouse_sys::cityhash::city_hash_64;

    #[test]
    fn test_city_hash() {
        let city = "Moscow";
        let hash = city_hash_64(city.as_bytes());
        println!("City: {}, Hash: {:?}", city, hash);
    }
}
