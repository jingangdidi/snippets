use std::path::{Path, PathBuf};

#[cfg(feature = "embedding")]
use candle_core::{
    Device,
    Tensor,
    utils::{
        cuda_is_available,
        metal_is_available,
    },
};
use serde::{Deserialize, Serialize};

pub mod error;
#[cfg(feature = "embedding")]
pub mod granite_english_r2;
#[cfg(feature = "embedding")]
pub mod granite_multilingual;
#[cfg(feature = "embedding")]
pub mod all_minilm_v2;
#[cfg(feature = "embedding")]
pub mod paraphrase_multilingual_minilm_l12_v2;
#[cfg(feature = "embedding")]
pub mod mxbai_embed_v1;
#[cfg(feature = "embedding")]
pub mod e5_base_v2;
#[cfg(feature = "embedding")]
pub mod multilingual_e5_small;

use error::EmbeddingError;
#[cfg(feature = "embedding")]
use granite_english_r2::GraniteEnglishR2Model;
#[cfg(feature = "embedding")]
use granite_multilingual::GraniteMultilingualModel;
#[cfg(feature = "embedding")]
use all_minilm_v2::AllMiniLmV2Model;
#[cfg(feature = "embedding")]
use paraphrase_multilingual_minilm_l12_v2::ParaphraseMultilingualMiniLmL12V2Model;
#[cfg(feature = "embedding")]
use mxbai_embed_v1::MxbaiEmbedV1Model;
#[cfg(feature = "embedding")]
use e5_base_v2::E5BaseV2Model;
#[cfg(feature = "embedding")]
use multilingual_e5_small::MultilingualE5SmallModel;

/// model path name
#[derive(Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum Model {
    GraniteEmbeddingSmallEnglishR247m,
    GraniteEmbeddingEnglishR2149m,
    GraniteEmbeddingMultilingual107m,
    GraniteEmbeddingMultilingual278m,
    AllMiniLmL6V2,
    AllMiniLmL12V2,
    ParaphraseMultilingualMiniLmL12V2,
    MxbaiEmbedLargeV1,
    MxbaiEmbedXsmallV1,
    E5BaseV2,
    MultilingualE5Small,
}

/// all supported model name
const MODEL_NAME: &[&str;11] = &[
    "granite-embedding-small-english-r2", // https://huggingface.co/ibm-granite/granite-embedding-small-english-r2
    "granite-embedding-english-r2", // https://huggingface.co/ibm-granite/granite-embedding-english-r2
    "granite-embedding-107m-multilingual", // https://huggingface.co/ibm-granite/granite-embedding-107m-multilingual
    "granite-embedding-278m-multilingual", // https://huggingface.co/ibm-granite/granite-embedding-278m-multilingual
    "all-MiniLM-L6-v2", // https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2
    "all-MiniLM-L12-v2", // https://huggingface.co/sentence-transformers/all-MiniLM-L12-v2
    "paraphrase-multilingual-MiniLM-L12-v2", // https://huggingface.co/sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2
    "mxbai-embed-large-v1", // https://huggingface.co/mixedbread-ai/mxbai-embed-large-v1
    "mxbai-embed-xsmall-v1", // https://hf-mirror.com/mixedbread-ai/mxbai-embed-xsmall-v1
    "e5-base-v2", // https://huggingface.co/intfloat/e5-base-v2
    "multilingual-e5-small", // https://huggingface.co/intfloat/multilingual-e5-small
];

impl Model {
    /// create from -m id
    pub fn new(id: usize) -> Result<Self, EmbeddingError> {
        match id {
            1  => Ok(Self::GraniteEmbeddingSmallEnglishR247m),
            2  => Ok(Self::GraniteEmbeddingEnglishR2149m),
            3  => Ok(Self::GraniteEmbeddingMultilingual107m),
            4  => Ok(Self::GraniteEmbeddingMultilingual278m),
            5  => Ok(Self::AllMiniLmL6V2),
            6  => Ok(Self::AllMiniLmL12V2),
            7  => Ok(Self::ParaphraseMultilingualMiniLmL12V2),
            8  => Ok(Self::MxbaiEmbedLargeV1),
            9  => Ok(Self::MxbaiEmbedXsmallV1),
            10 => Ok(Self::E5BaseV2),
            11 => Ok(Self::MultilingualE5Small),
            _  => return Err(EmbeddingError::ParaError{para: format!("-m not support this model: {}", id)}),
        }
    }

    /// get model type and model path
    pub fn model_type_and_path(&self, base_path: &Path) -> (ModelType, PathBuf) {
        match self {
            Self::GraniteEmbeddingSmallEnglishR247m  => (ModelType::GraniteEnglishR2,                  base_path.join(MODEL_NAME[0])),
            Self::GraniteEmbeddingEnglishR2149m      => (ModelType::GraniteEnglishR2,                  base_path.join(MODEL_NAME[1])),
            Self::GraniteEmbeddingMultilingual107m   => (ModelType::GraniteMultilingual,               base_path.join(MODEL_NAME[2])),
            Self::GraniteEmbeddingMultilingual278m   => (ModelType::GraniteMultilingual,               base_path.join(MODEL_NAME[3])),
            Self::AllMiniLmL6V2                      => (ModelType::AllMiniLmV2,                       base_path.join(MODEL_NAME[4])),
            Self::AllMiniLmL12V2                     => (ModelType::AllMiniLmV2,                       base_path.join(MODEL_NAME[5])),
            Self::ParaphraseMultilingualMiniLmL12V2  => (ModelType::ParaphraseMultilingualMiniLmL12V2, base_path.join(MODEL_NAME[6])),
            Self::MxbaiEmbedLargeV1                  => (ModelType::MxbaiEmbedV1,                      base_path.join(MODEL_NAME[7])),
            Self::MxbaiEmbedXsmallV1                 => (ModelType::MxbaiEmbedV1,                      base_path.join(MODEL_NAME[8])),
            Self::E5BaseV2                           => (ModelType::E5BaseV2,                          base_path.join(MODEL_NAME[9])),
            Self::MultilingualE5Small                => (ModelType::MultilingualE5Small,               base_path.join(MODEL_NAME[10])),
        }
    }

    /// get model type
    pub fn model_type(&self) -> ModelType {
        match self {
            Self::GraniteEmbeddingSmallEnglishR247m  => ModelType::GraniteEnglishR2,
            Self::GraniteEmbeddingEnglishR2149m      => ModelType::GraniteEnglishR2,
            Self::GraniteEmbeddingMultilingual107m   => ModelType::GraniteMultilingual,
            Self::GraniteEmbeddingMultilingual278m   => ModelType::GraniteMultilingual,
            Self::AllMiniLmL6V2                      => ModelType::AllMiniLmV2,
            Self::AllMiniLmL12V2                     => ModelType::AllMiniLmV2,
            Self::ParaphraseMultilingualMiniLmL12V2  => ModelType::ParaphraseMultilingualMiniLmL12V2,
            Self::MxbaiEmbedLargeV1                  => ModelType::MxbaiEmbedV1,
            Self::MxbaiEmbedXsmallV1                 => ModelType::MxbaiEmbedV1,
            Self::E5BaseV2                           => ModelType::E5BaseV2,
            Self::MultilingualE5Small                => ModelType::MultilingualE5Small
        }
    }

    /// check valid model path, model.safetensors, config.json, tokenizer.json exist
    pub fn check_model(base_path: &Path) -> Result<Option<(ModelType, Model, (PathBuf, PathBuf, PathBuf))>, EmbeddingError> {
        if base_path.exists() && base_path.is_dir() {
            if let Some(name) = base_path.file_name() {
                if let Some(index) = MODEL_NAME.iter().position(|n| *n == name) {
                    let model = Model::new(index+1)?;
                    Ok(Some((model.model_type(), model, Self::check_model_files(base_path)?)))
                } else {
                    Ok(None)
                }
            } else {
                Ok(None)
            }
        } else {
            Err(EmbeddingError::ParaError{para: format!("path not exist: {}", base_path.display())})
        }
    }

    /// check valid model model.safetensors, config.json, tokenizer.json exist
    pub fn check_model_files(base_path: &Path) -> Result<(PathBuf, PathBuf, PathBuf), EmbeddingError> {
        if base_path.exists() && base_path.is_dir() {
            let model_path = base_path.join("model.safetensors");
            if model_path.exists() && model_path.is_file() {
                let config_path = base_path.join("config.json");
                if config_path.exists() && config_path.is_file() {
                    let tokenizer_path = base_path.join("tokenizer.json");
                    if tokenizer_path.exists() && tokenizer_path.is_file() {
                        Ok((model_path, config_path, tokenizer_path))
                    } else {
                        Err(EmbeddingError::FileNotExistError{file: config_path.display().to_string()})
                    }
                } else {
                    Err(EmbeddingError::FileNotExistError{file: config_path.display().to_string()})
                }
            } else {
                Err(EmbeddingError::FileNotExistError{file: model_path.display().to_string()})
            }
        } else {
            Err(EmbeddingError::ParaError{para: format!("path not exist: {}", base_path.display())})
        }
    }
}

/// embedding model type
#[derive(Clone)]
pub enum ModelType {
    GraniteEnglishR2,                  // 1, 2
    GraniteMultilingual,               // 3, 4
    AllMiniLmV2,                       // 5, 6
    ParaphraseMultilingualMiniLmL12V2, // 7
    MxbaiEmbedV1,                      // 8, 9
    E5BaseV2,                          // 10
    MultilingualE5Small,               // 11
}

/// embedding trait
#[cfg(feature = "embedding")]
trait EmbeddingMethod {
    /// init model
    fn new(model_file: &Path, config_file: &Path, tokenizer_file: &Path, device: Device) -> Result<Self, EmbeddingError> where Self: Sized;

    /// generates embedding for a sentence
    fn get_embedding(&self, sentence: &str) -> Result<Vec<f32>, EmbeddingError>;
}

/// embedding model
#[cfg(feature = "embedding")]
pub enum EmbeddingModel {
    GraniteEnglishR2(GraniteEnglishR2Model),
    GraniteMultilingual(GraniteMultilingualModel),
    AllMiniLmV2(AllMiniLmV2Model),
    ParaphraseMultilingualMiniLmL12V2(ParaphraseMultilingualMiniLmL12V2Model),
    MxbaiEmbedV1(MxbaiEmbedV1Model),
    E5BaseV2(E5BaseV2Model),
    MultilingualE5Small(MultilingualE5SmallModel),
}

#[cfg(feature = "embedding")]
impl EmbeddingModel {
    /// load model
    pub fn load_model(model: &ModelType, model_file: &Path, config_file: &Path, tokenizer_file: &Path, use_cpu: bool) -> Result<Self, EmbeddingError> {
        let device = if use_cpu {
            Device::Cpu
        } else {
            if (cfg!(target_os = "windows") || cfg!(target_os = "linux")) && cuda_is_available() {
                Device::new_cuda(0)?
            } else if cfg!(target_os = "macos") && metal_is_available() {
                Device::new_metal(0)?
            } else {
                Device::Cpu
            }
        };
        Ok(
            match model {
                ModelType::GraniteEnglishR2                  => Self::GraniteEnglishR2(GraniteEnglishR2Model::new(model_file, config_file, tokenizer_file, device)?),
                ModelType::GraniteMultilingual               => Self::GraniteMultilingual(GraniteMultilingualModel::new(model_file, config_file, tokenizer_file, device)?),
                ModelType::AllMiniLmV2                       => Self::AllMiniLmV2(AllMiniLmV2Model::new(model_file, config_file, tokenizer_file, device)?),
                ModelType::ParaphraseMultilingualMiniLmL12V2 => Self::ParaphraseMultilingualMiniLmL12V2(ParaphraseMultilingualMiniLmL12V2Model::new(model_file, config_file, tokenizer_file, device)?),
                ModelType::MxbaiEmbedV1                      => Self::MxbaiEmbedV1(MxbaiEmbedV1Model::new(model_file, config_file, tokenizer_file, device)?),
                ModelType::E5BaseV2                          => Self::E5BaseV2(E5BaseV2Model::new(model_file, config_file, tokenizer_file, device)?),
                ModelType::MultilingualE5Small               => Self::MultilingualE5Small(MultilingualE5SmallModel::new(model_file, config_file, tokenizer_file, device)?),
            }
        )
    }

    /// generates embedding for a sentence
    pub fn get_embedding(&self, sentence: &str) -> Result<Vec<f32>, EmbeddingError> {
        match self {
            Self::GraniteEnglishR2(model)                  => model.get_embedding(sentence),
            Self::GraniteMultilingual(model)               => model.get_embedding(sentence),
            Self::AllMiniLmV2(model)                       => model.get_embedding(sentence),
            Self::ParaphraseMultilingualMiniLmL12V2(model) => model.get_embedding(sentence),
            Self::MxbaiEmbedV1(model)                      => model.get_embedding(sentence),
            Self::E5BaseV2(model)                          => model.get_embedding(sentence),
            Self::MultilingualE5Small(model)               => model.get_embedding(sentence),
        }
    }
}

/// average pooling
/// last_hidden_state: (1, 6, 768)
/// attention_mask: (1, 6)
/// last_hidden: (1, 6, 768)
/// sum_embeddings: (1, 768)
/// sum_mask: (1, 1)
/// return: (1, 768)
#[cfg(feature = "embedding")]
pub fn average_pool(last_hidden_state: &Tensor, attention_mask: &Tensor) -> Result<Tensor, EmbeddingError> {
    let attention_mask = attention_mask.unsqueeze(2)?.to_dtype(last_hidden_state.dtype())?;
    let last_hidden = last_hidden_state.broadcast_mul(&attention_mask)?;
    let sum_embeddings = last_hidden.sum(1)?;
    let sum_mask = attention_mask.sum(1)?;
    Ok(sum_embeddings.broadcast_div(&sum_mask)?)
}

/// check embedding feature is enaabled
pub fn embedding_enabled() -> bool {
    cfg!(feature = "embedding")
}

/// split long discription to multiple short lines
pub fn split_discription(disc: &str, max_width: usize) -> String {
    let mut short_lines: Vec<String> = Vec::new();
    let lines: Vec<&str> = disc.lines().collect();
    for line in lines {
        let chars = line.chars();
        if chars.clone().count() > max_width {
            short_lines.extend(
                chars.collect::<Vec<char>>()
                    .chunks(max_width)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect::<Vec<String>>()
            );
        } else {
            short_lines.push(line.to_string());
        }
    }
    short_lines.join("\n")
}
