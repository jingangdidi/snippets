use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;

use ron::de::from_str;
use serde::{Deserialize, Serialize};

use embedding_lib::{
    Model,
    ModelType,
    split_discription,
};

/// single snippet
#[derive(Deserialize, Serialize)]
struct SingleSnippet {
    tags:        HashSet<String>,                  // one snippet could have multiple tag
    discription: String,                           // discription of this snippet
    embedding:   Option<HashMap<Model, Vec<f32>>>, // discription's embedding, key: model, value: embedding vec
    content:     String,                           // snippet content
}

/// merge all ../snippets_database/*.snippets to ../snippets_database/default.snippets
/// create ../snippets_database/enum.rs based on all *.snippets tags
/// verbose: cargo build --release -vv
fn main() {
    // get all embedding models from ../embedding_models
    let mut all_models: Vec<(ModelType, Model, (PathBuf, PathBuf, PathBuf))> = Vec::new(); // (Model, (model.safetensors, config.json, tokenizer.json))
    let model_path = PathBuf::from("../embedding_models");
    if model_path.exists() && model_path.is_dir() {
        if let Ok(dirs) = model_path.read_dir() {
            for i in dirs {
                if let Ok(entry) = i {
                    let tmp_path = entry.path();
                    if let Ok(Some(m)) = Model::check_model(&tmp_path) {
                        all_models.push(m);
                    }
                }
            }
        }
    }
    // get all snippets
    let mut all_snippets = Vec::new();
    let mut all_tags: HashSet<String> = HashSet::new();
    let tmp_dir = Path::new("../snippets_database/");
    if tmp_dir.exists() && tmp_dir.is_dir() {
        if let Ok(dirs) = tmp_dir.read_dir() {
            for i in dirs {
                if let Ok(entry) = i {
                    let tmp_file_path = entry.path();
                    if tmp_file_path.is_file() {
                        if let (Some(name), Some(ext)) = (tmp_file_path.file_name(), tmp_file_path.extension()) {
                            if name != "default.snippets" && ext == "snippets" {
                                let (tags, snippets) = read_file_as_snippets(&tmp_file_path, &all_models);
                                all_tags.extend(tags);
                                all_snippets.extend(snippets);
                            }
                        }
                    }
                }
            }
            // save default.snippets
            let f = fs::File::options()
                .create(true)
                .write(true)
                .truncate(true)
                .open("../snippets_database/default.snippets")
                .expect("Error - Failed opening file ../snippets_database/default.snippets");
            ron::Options::default()
                .to_io_writer_pretty(f, &all_snippets, ron::ser::PrettyConfig::new().escape_strings(false).compact_arrays(true))
                .expect("Error - Failed to write to file ../snippets_database/default.snippets");
            // save enum
            // tag's first letter must be alphabetic and uppercase
            if all_tags.iter().any(|t| t.chars().next().map(|c| !c.is_alphabetic() || !c.is_uppercase()).unwrap_or(true)) {
                panic!("all tags must start with a capital letter")
            }
            let mut sorted_tags: Vec<String> =all_tags.into_iter().collect::<Vec<_>>();
            sorted_tags.sort();
            let enum_str = format!(
                r##"use std::collections::HashSet;

use serde::{{
    Deserialize,
    Deserializer,
    de::Error,
}};

/// snippets category, add this tag to each snippet
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SnipTag {{
    {},
}}

impl SnipTag {{
    /// tag to string
    pub fn to_string(&self) -> String {{
        match self {{
            {},
        }}
    }}

    /// get all tags
    pub fn all_tags() -> Vec<Self> {{
        vec![{}]
    }}

    /// string to SnipTag
    pub fn string_to_tag(tag_str: &str) -> Option<SnipTag> {{
        let tag = match tag_str {{
            {},
            _ => return None,
        }};
        Some(tag)
    }}

    /// if tag is programming language, add commit before discription, suffix format not use txt
    pub fn commit_format(tags: &HashSet<SnipTag>) -> (String, String) {{
        let tags_str: HashSet<String> = tags.iter().map(|t| t.to_string()).collect();
        if tags_str.contains("Python") {{
            ("# ".to_string(), "py".to_string())
        }} else if tags_str.contains("R") {{
            ("# ".to_string(), "r".to_string())
        }} else if tags_str.contains("Rust") {{
            ("// ".to_string(), "rs".to_string())
        }} else if tags_str.contains("Shell") {{
            ("# ".to_string(), "sh".to_string())
        }} else if tags_str.contains("Go") {{
            ("// ".to_string(), "go".to_string())
        }} else if tags_str.contains("Js") {{
            ("// ".to_string(), "js".to_string())
        }} else {{
            ("".to_string(), "txt".to_string())
        }}
    }}

    /// all supported tags
    pub fn supported_tags() -> String {{
        "{}".to_string()
    }}
}}

/// deserialize string to SnipTag
pub fn deserialize_tags_from_strings<'de, D>(deserializer: D) -> Result<HashSet<SnipTag>, D::Error>
where
    D: Deserializer<'de>,
{{
    let strings: Vec<String> = Vec::deserialize(deserializer)?;
    //Ok(strings.into_iter().map(|s| SnipTag::string_to_tag(&s.to_lowercase()).unwrap()).collect::<HashSet<SnipTag>>())
    strings
        .into_iter()
        .map(|s| SnipTag::string_to_tag(&s.to_lowercase()).ok_or_else(|| D::Error::custom(format!(r#"The tag ("{{}}") must belong to the tags contained in the "*.snippets" files used during compilation."#, s))))
        .collect::<Result<HashSet<SnipTag>, D::Error>>()
}}
"##,
                sorted_tags.join(",\n    "),
                sorted_tags.iter().map(|t| format!("Self::{} => \"{}\".to_string()", t, t)).collect::<Vec<_>>().join(",\n            "),
                sorted_tags.iter().map(|t| format!("Self::{}", t)).collect::<Vec<_>>().join(", "),
                sorted_tags.iter().map(|t| format!("\"{}\" => Self::{}", t.to_lowercase(), t)).collect::<Vec<_>>().join(",\n            "),
                sorted_tags.join(", "),
            );
            fs::write("../snippets_database/tags.rs", enum_str).expect("Error - save ../snippets_database/tags.rs failed");
        }
    } else {
        println!("Error - no such path: ../snippets_database/");
    }
}

/// read file to string, skip first line and last line
fn read_file_as_snippets(
    file_path: &Path,
    #[cfg_attr(not(feature = "embedding"), allow(unused_variables))]
    all_models: &Vec<(ModelType, Model, (PathBuf, PathBuf, PathBuf))>,
) -> (HashSet<String>, Vec<SingleSnippet>) {
    let content = fs::read_to_string(file_path).expect(&format!("Error - read file to string failed: {}", file_path.display()));
    // calculate embedding
    let mut tags: HashSet<String> = HashSet::new();
    let snippets = match from_str::<Vec<SingleSnippet>>(&content) {
        Ok(mut snippets) => {
            // if use embedding features in Cargo.toml, calculate embeddings
            #[cfg(feature = "embedding")]
            for (model_type, model, (model_path, config_path, tokenizer_path)) in all_models {
                // load embedding model
                match embedding_lib::EmbeddingModel::load_model(
                    &model_type,
                    &model_path, // model.safetensors
                    &config_path, // config.json
                    &tokenizer_path, // tokenizer.json
                    false, // use cpu
                ) {
                    Ok(embedding_model) => {
                        for i in 0..snippets.len() {
                            match &snippets[i].embedding {
                                Some(snip_embed) => {
                                    if let Ok(embedding) = embedding_model.get_embedding(&snippets[i].discription) {
                                        if !snip_embed.contains_key(&model) {
                                            if let Some(snip_embed) = &mut snippets[i].embedding {
                                                snip_embed.insert(model.clone(), embedding);
                                            }
                                        }
                                    }
                                },
                                None => {
                                    if let Ok(embedding) = embedding_model.get_embedding(&snippets[i].discription) {
                                        snippets[i].embedding = Some(HashMap::from([(model.clone(), embedding)]));
                                    }
                                },
                            }
                        }
                    },
                    Err(e) => println!("{}", e),
                }
            }
            for i in 0..snippets.len() {
                tags.extend(snippets[i].tags.clone());
                // trim space and remove `\r`
                snippets[i].discription = snippets[i].discription.trim().replace("\r", "");
                // split long discription to multiple short lines
                snippets[i].discription = split_discription(&snippets[i].discription, 20);
            }
            snippets
        },
        Err(e) => {
            println!("{}", e);
            Vec::new()
        },
    };

    (tags, snippets)
}
