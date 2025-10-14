use std::collections::HashMap;
use std::fs::{File, read_to_string};
use std::io::Read;
use std::path::Path;

use candle_core::{Device, DType, IndexOp, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::modernbert::{ModernBert, Config};
use safetensors::{
    SafeTensors,
    tensor::Dtype,
};
use tokenizers::{PaddingParams, Tokenizer, TruncationParams};

use crate::{
    error::EmbeddingError,
    EmbeddingMethod,
};

/// embedding model
pub struct GraniteEnglishR2Model {
    model:     ModernBert,
    tokenizer: Tokenizer,
    device:    Device,
}

impl EmbeddingMethod for GraniteEnglishR2Model {
    /// Loads the model and tokenizer from local
    fn new(model_file: &Path, config_file: &Path, tokenizer_file: &Path, device: Device) -> Result<Self, EmbeddingError> {
        // Load model configuration
        let config: Config = serde_json::from_str(&read_to_string(config_file)?).map_err(|e| EmbeddingError::JsonFromStrError{error: e})?;

        // read model.safetensors
        let mut file = File::open(model_file)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        // deserialize safetensors
        let tensors = SafeTensors::deserialize(&buffer).unwrap();
        // get tensor name and tensor
        let mut name_tensor: HashMap<String, Tensor> = HashMap::new(); // key: name, value: Tensor
        for (name, tensor_view) in tensors.tensors() {
            //println!("name: {}, shape: {:?}, dtype: {:?}", name, &tensor_view.shape().to_vec(), tensor_view.dtype());
            name_tensor.insert(
                name.to_string(),
                Tensor::from_raw_buffer(
                    tensor_view.data(),
                    to_candle_dtype(tensor_view.dtype())?,
                    &tensor_view.shape().to_vec(),
                    &device,
                ).unwrap(),
            );
        }

        // Load model weights
        /* this will fail
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[model_file], DType::F32, &device)?
        };
        */

        // add "model" prefix
        let mut new_tensors = HashMap::new(); // key: new_name, value: tensor
        for (name, tensor) in name_tensor {
            new_tensors.insert(format!("model.{}", &name), tensor);
        }
        let vb = VarBuilder::from_tensors(new_tensors, DType::F32, &device);

        // init modern bert
        let model = ModernBert::load(vb, &config)?;

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
        let token_ids = Tensor::new(token_ids, &self.device)?.unsqueeze(0)?;

        let token_type_ids: Vec<u32> = tokens.get_type_ids().to_vec();
        let token_type_ids = Tensor::new(token_type_ids, &self.device)?.unsqueeze(0)?;

        // forward pass
        let last_hidden_state = self.model.forward(&token_ids, &token_type_ids)?.to_dtype(DType::F32)?;

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

/// safetensors dtype to candle dtype
fn to_candle_dtype(dtype: Dtype) -> Result<DType, EmbeddingError> {
    match dtype {
        Dtype::U8   => Ok(DType::U8),
        Dtype::F16  => Ok(DType::F16),
        Dtype::BF16 => Ok(DType::BF16),
        Dtype::U32  => Ok(DType::U32),
        Dtype::F32  => Ok(DType::F32),
        Dtype::F64  => Ok(DType::F64),
        Dtype::I64  => Ok(DType::I64),
        t           => Err(EmbeddingError::DtypeError{info: format!("candle does not have dtype {} in safetensors", t)}), // BOOL, F4, F6_F6_E2M3, F6_E3M2, F8_E5M2, F8_E4M3, F8_E8M0, I16, U16, I32, U64
    }
}
