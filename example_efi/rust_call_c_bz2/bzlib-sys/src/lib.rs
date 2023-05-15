
// 生成的 bindings 代码根据 C/C++ 代码生成，里面有一些不符合 Rust 约定，我们不让编译期报警
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

use anyhow::{anyhow, Result};
use std::mem;

mod bindings;

pub use bindings::*;


// compress 函数接受一个字节切片作为输入，并返回一个压缩后的字节向量或错误
pub fn compress(input: &[u8]) -> Result<Vec<u8>> {
    // 创建一个与输入长度相同的零初始化向量作为输出
    let output = vec![0u8; input.len()];
    unsafe {
        // 使用 zeroed 方法创建一个初始化为零的 bz_stream 结构
        let mut stream: bz_stream = mem::zeroed();
        // 调用 BZ2_bzCompressInit 初始化压缩流，如果返回值不是 BZ_OK，就返回一个错误
        let result = BZ2_bzCompressInit(&mut stream as *mut _, 1, 0, 0);
        if result != BZ_OK as _ {
            return Err(anyhow!("Failed to initialize"));
        }

        // 设置压缩流的输入和输出
        stream.next_in = input.as_ptr() as *mut _;
        stream.avail_in = input.len() as _;
        stream.next_out = output.as_ptr() as *mut _;
        stream.avail_out = output.len() as _;
        // 调用 BZ2_bzCompress 进行压缩，如果返回值不是 BZ_STREAM_END，就返回一个错误
        let result = BZ2_bzCompress(&mut stream as *mut _, BZ_FINISH as _);
        if result != BZ_STREAM_END as _ {
            return Err(anyhow!("Failed to compress"));
        }

        // 调用 BZ2_bzCompressEnd 结束压缩，如果返回值不是 BZ_OK，就返回一个错误
        let result = BZ2_bzCompressEnd(&mut stream as *mut _);
        if result != BZ_OK as _ {
            return Err(anyhow!("Failed to end compression"));
        }
    }

    // 返回压缩后的输出
    Ok(output)
}

// decompress 函数接收一个字节切片作为输入，返回解压后的字节向量或一个错误
pub fn decompress(input: &[u8]) -> Result<Vec<u8>> {
    // 创建一个与输入长度相同的零初始化向量作为输出
    let output = vec![0u8; input.len()];
    unsafe {
        // 使用 zeroed 方法创建一个初始化为零的 bz_stream 结构
        let mut stream: bz_stream = mem::zeroed();
        // 调用 BZ2_bzDecompressInit 初始化解压缩流，如果返回值不是 BZ_OK，就返回一个错误
        let result = BZ2_bzDecompressInit(&mut stream as *mut _, 0, 0);
        if result != BZ_OK as _ {
            return Err(anyhow!("Failed to initialize"));
        }

        // 设置解压缩流的输入和输出
        stream.next_in = input.as_ptr() as *mut _;
        stream.avail_in = input.len() as _;
        stream.next_out = output.as_ptr() as *mut _;
        stream.avail_out = output.len() as _;
        // 调用 BZ2_bzDecompress 进行解压缩，如果返回值不是 BZ_STREAM_END，那么返回一个错误
        let result = BZ2_bzDecompress(&mut stream as *mut _);
        if result != BZ_STREAM_END as _ {
            return Err(anyhow!("Failed to compress"));
        }

        // 调用 BZ2_bzDecompressEnd 结束解压缩，如果返回值不是 BZ_OK，那么返回一个错误
        let result = BZ2_bzDecompressEnd(&mut stream as *mut _);
        if result != BZ_OK as _ {
            return Err(anyhow!("Failed to end compression"));
        }
    }

    // 返回解压后的输出
    Ok(output)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compression_decompression_should_work() {
        let input = include_str!("bindings.rs").as_bytes();
        let compressed = compress(input).unwrap();
        let decompressed = decompress(&compressed).unwrap();

        assert_eq!(input, &decompressed);
    }
}