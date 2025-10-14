use std::fs::read_to_string;
use std::path::Path;

use candle_core::{Device, DType, IndexOp, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::xlm_roberta::{XLMRobertaModel, Config};
use tokenizers::{PaddingParams, Tokenizer, TruncationParams};

use crate::{
    error::EmbeddingError,
    EmbeddingMethod,
};

/// embedding model
pub struct GraniteMultilingualModel {
    model:     XLMRobertaModel,
    tokenizer: Tokenizer,
    device:    Device,
}

impl EmbeddingMethod for GraniteMultilingualModel {
    /// Loads the model and tokenizer from local
    fn new(model_file: &Path, config_file: &Path, tokenizer_file: &Path, device: Device) -> Result<Self, EmbeddingError> {
        // Load model configuration
        let config: Config = serde_json::from_str(&read_to_string(config_file)?).map_err(|e| EmbeddingError::JsonFromStrError{error: e})?;

        // Load model weights
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[model_file], DType::F32, &device)?
        };

        // init modern bert
        let model = XLMRobertaModel::new(&config, vb)?;

        // Load model tokenizer
        let mut tokenizer = Tokenizer::from_file(tokenizer_file).map_err(|e| EmbeddingError::TokenizerFromFileError{error: e})?;

        // tokenizer truncation
        // This model produces embedding vectors of size 384 based on context length of upto 8192 tokens
        let truncation_params = TruncationParams {
            max_length: config.max_position_embeddings,
            ..Default::default()
        };
        let _ = tokenizer.with_truncation(Some(truncation_params));

        // tokenizer padding
        let padding_params = PaddingParams {
            strategy: tokenizers::PaddingStrategy::BatchLongest,
            ..Default::default()
        };
        tokenizer.with_padding(Some(padding_params));

        Ok(Self { model, tokenizer, device })
    }

    /// Generates embedding for a sentence
    fn get_embedding(&self, sentence: &str) -> Result<Vec<f32>, EmbeddingError> {
        // encode the given input, accepts both single sequences, as well as pair sequences.
        // A sequence can be a string, or already pre-tokenized input directly
        let tokens = self.tokenizer.encode(sentence, true).map_err(|e| EmbeddingError::TokenizerEncodeError{error: e})?;

        // Convert the tokenized data (Vec<u32>) into Candle Tensors
        let token_ids: Vec<u32> = tokens.get_ids().to_vec();

        let attention_mask = vec![1u32; token_ids.len()];
        let attention_mask = Tensor::new(attention_mask, &self.device)?.unsqueeze(0)?;

        let token_ids = Tensor::new(token_ids, &self.device)?.unsqueeze(0)?;

        let token_type_ids: Vec<u32> = tokens.get_type_ids().to_vec();
        let token_type_ids = Tensor::new(token_type_ids, &self.device)?.unsqueeze(0)?;

        // forward pass
        let last_hidden_state = self.model.forward(&token_ids, &attention_mask, &token_type_ids, None, None, None)?.to_dtype(DType::F32)?;

        // Pooling and Normalization
        // `model_output[0][:, 0]` becomes `last_hidden_state.i((.., 0))?`
        // This performs CLS Pooling by taking the embedding of the first token (`[CLS]`).
        // The `..` syntax means "select all elements on this axis" (the batch axis).
        let query_embeddings = last_hidden_state.i((.., 0))?;

        // Manually implement L2 normalization
        // The formula is `x / sqrt(sum(x^2))`
        let norms = query_embeddings.sqr()?.sum_keepdim(1)?.sqrt()?;
        let normalized_embeddings = query_embeddings.broadcast_div(&norms)?;

        // convert Tensor [1, 384] to Vec<f32>
        Ok(normalized_embeddings.squeeze(0)?.to_vec1::<f32>()?)
    }
}
