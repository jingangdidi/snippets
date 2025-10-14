use std::io;

#[cfg(feature = "embedding")]
use candle_core::error::Error as candle_error;
use serde_json::Error as json_error;
use thiserror::Error;
#[cfg(feature = "embedding")]
use tokenizers::tokenizer::Error as tokenizer_error;

/// srx添加，自定义的错误类型，方便传递错误
/// 参考：https://github.com/dtolnay/thiserror
/// 参考：https://crates.io/crates/thiserror
/// 参考：https://juejin.cn/post/7272005801081126968
/// 参考：https://www.shakacode.com/blog/thiserror-anyhow-or-how-i-handle-errors-in-rust-apps/
/// 参考：https://rustcc.cn/article?id=1e20f814-c7d5-4aca-bb67-45dcfb65d9f9
#[derive(Debug, Error)]
pub enum EmbeddingError {
    // file not exist error
    #[error("Error - {file} does not exist")]
    FileNotExistError{file: String},

    // 数据转为json字符串错误
    #[error("Error - to json string: {error}")]
    ToJsonStirngError{uuid: String, error: json_error},

    // json转字符串错误
    #[error("Error - serde_json::to_string: {error}")]
    JsonToStringError{error: io::Error},

    // string to json error
    #[error("Error - serde_json::from_str: {error}")]
    JsonFromStrError{error: json_error},

    // load tokenizer file error
    #[cfg(feature = "embedding")]
    #[error("Error - Tokenizer::from_file: {error}")]
    TokenizerFromFileError{error: tokenizer_error},

    // tokenizer encode batch error
    #[cfg(feature = "embedding")]
    #[error("Error - tokenizer encode: {error}")]
    TokenizerEncodeError{error: tokenizer_error},

    // candle error
    #[cfg(feature = "embedding")]
    #[error("Error - candle: {0}")]
    CandleError(#[from] candle_error),

    // candle does not have some dtype in safetensors
    #[error("Error - {info}")]
    DtypeError{info: String},

    // 参数使用错误
    #[error("Error - {para}")]
    ParaError{para: String},

    // 常规io::Error，这里可以改为向上面那样将错误传过来，但不知道还能否使用`#[from]`
    #[error("I/O error occurred")]
    IoError(#[from] io::Error),
}
