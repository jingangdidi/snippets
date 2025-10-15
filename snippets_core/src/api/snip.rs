use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::PathBuf;

use arboard::Clipboard;
use ron::{
    de::from_str,
    error::{
        SpannedError,
        Error as ron_error,
        Position,
    },
};
use serde::Deserialize;
use tabled::{
    builder::Builder,
    settings::{
        Color,
        Span,
        Style,
        style::BorderColor,
    },
};

use crate::{
    SnipTag,
    deserialize_tags_from_strings,
    parse_paras::{
        ModelInfo,
        ParsedParas,
    },
    utils::{
        get_snippet_files,
        my_writer,
    },
    error::MyError,
};

#[cfg(feature = "embedding")]
use crate::utils::cosine_similarity;

use embedding_lib::split_discription;

#[cfg(feature = "embedding")]
use embedding_lib::Model;


/// compile default snippets file to binary
/// build.rs will combine all ../snippets_database/*.snippets (exclude default.snippets) to default.snippets
const SNIPPETS: &str = include_str!("../../../snippets_database/default.snippets");

/*
/// snippets category, add this tag to each snippet
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SnipTag {
    Bio,
    Code,
    Command,
    Css,
    Docker,
    File,
    Git,
    Github,
    Go,
    Install,
    Js,
    Note,
    Other,
    Python,
    R,
    Rust,
    Shell,
    Tool,
    Usage,
}

impl SnipTag {
    /// tag to string
    fn to_string(&self) -> String {
        match self {
            Self::Bio     => "Bio".to_string(),
            Self::Code    => "Code".to_string(),
            Self::Command => "Command".to_string(),
            Self::Css     => "Css".to_string(),
            Self::Docker  => "Docker".to_string(),
            Self::File    => "File".to_string(),
            Self::Git     => "Git".to_string(),
            Self::Github  => "Github".to_string(),
            Self::Go      => "Go".to_string(),
            Self::Install => "Install".to_string(),
            Self::Js      => "Js".to_string(),
            Self::Note    => "Note".to_string(),
            Self::Other   => "Other".to_string(),
            Self::Python  => "Python".to_string(),
            Self::R       => "R".to_string(),
            Self::Rust    => "Rust".to_string(),
            Self::Shell   => "Shell".to_string(),
            Self::Tool    => "Tool".to_string(),
            Self::Usage   => "Usage".to_string(),
        }
    }
}
*/

/// single snippet
/// #[allow(dead_code)]
#[derive(Clone, Deserialize)]
struct SingleSnippet {
    #[serde(deserialize_with = "deserialize_tags_from_strings")]
    tags:        HashSet<SnipTag>,                 // one snippet could have multiple tag
    discription: String,                           // discription of this snippet
    #[cfg(feature = "embedding")]
    #[cfg_attr(not(feature = "embedding"), serde(skip))]
    embedding:   Option<HashMap<Model, Vec<f32>>>, // discription's embedding, key: model, value: embedding vec
    content:     String,                           // snippet content
}

impl SingleSnippet {
    /// convert tags to sorted Vec
    fn sorted_tags(&self) -> Vec<SnipTag> {
        let mut tags: Vec<SnipTag> = self.tags.clone().into_iter().collect();
        tags.sort();
        tags
    }

    /// convert tags to string
    fn format_tags(&self) -> String {
        self.sorted_tags()
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// store all snippets
pub struct Snippets {
    data:      Vec<SingleSnippet>,
    #[cfg(feature = "embedding")]
    embedding: Option<ModelInfo>,
}

impl Snippets {
    /// construct Snippets
    pub fn new(
        files: &Vec<PathBuf>,
        #[cfg_attr(not(feature = "embedding"), allow(unused_variables))]
        embedding: Option<ModelInfo>,
    ) -> Result<Self, MyError> {
        // if current path or binary file path contains *.snippets, read these files
        // else use snippets in the binary file.
        let files: Vec<PathBuf> = if files.is_empty() {
            get_snippet_files()?
        } else {
            files.clone()
        };
        let mut data: Vec<SingleSnippet> = if files.is_empty() { // use snippets in the binary file
            match from_str(SNIPPETS) {
                Ok(p) => p,
                Err(e) => if let SpannedError{code: ron_error::Message(m), position: Position{line, col}} = e {
                    return Err(MyError::ParaError{para: format!("{} position: line={}, column={}", m, line, col)})
                } else {
                    return Err(MyError::ParaError{para: format!("parse snippets error: {:?}", e)})
                },
            }
        } else { // get *.snippets files from current path or binary file path
            let mut data: Vec<SingleSnippet> = Vec::new();
            for f in files {
                match from_str::<Vec<SingleSnippet>>(&read_to_string(&f)?) {
                    Ok(mut p) => {
                        for i in 0..p.len() {
                            // trim space and remove `\r`
                            p[i].discription = p[i].discription.trim().replace("\r", "");
                            // split long discription to multiple short lines
                            p[i].discription = split_discription(&p[i].discription, 20);
                        }
                        data.extend(p);
                    },
                    Err(e) => if let SpannedError{code: ron_error::Message(m), position: Position{line, col}} = e {
                        return Err(MyError::ParaError{para: format!("{} position: line={}, column={}", m, line, col)})
                    } else {
                        return Err(MyError::ParaError{para: format!("parse snippets error: {:?}", e)})
                    },
                };
            }
            data
        };
        // sort snippets
        data.sort_by(|a, b| a.sorted_tags().cmp(&b.sorted_tags()).then(a.discription.cmp(&b.discription)));

        Ok(Self {
            data,
            #[cfg(feature = "embedding")]
            embedding,
        })
    }

    /// get snippets by ids
    fn get_by_ids(&self, ids: &Vec<usize>) -> Result<Vec<(usize, SingleSnippet, Option<f32>)>, MyError> {
        let mut snippets = Vec::new();
        for id in ids {
            if *id >= self.data.len() {
                return Err(MyError::ParaError{para: format!("no such snippet id: {}", id)})
            }
            for (i, s) in self.data.iter().enumerate() {
                if i == *id {
                    snippets.push((*id, s.clone(), None));
                }
            }
        }
        Ok(snippets)
    }

    /// get snippets by category
    /// if specify multiple categories, select only snippets that contain all specified tags
    fn get_by_categories(&self, categories: &Vec<SnipTag>) -> Result<Vec<(usize, SingleSnippet, Option<f32>)>, MyError> {
        let mut snippets = Vec::new();
        for (i, s) in self.data.iter().enumerate() {
            if categories.iter().all(|c| s.tags.contains(c)) {
                snippets.push((i, s.clone(), None));
            }
        }
        Ok(snippets)
    }

    /// get snippets by search keyword, ignore case
    /// -t and -e can be used simultaneously
    fn get_by_search(&self, categories: Option<Vec<SnipTag>>, keyword: &str) -> Result<Vec<(usize, SingleSnippet, Option<f32>)>, MyError> {
        let mut snippets = Vec::new();
        let kw = keyword.to_lowercase();
        if cfg!(feature = "embedding") {
            // keyword search or semantic search
            #[cfg(feature = "embedding")]
            match &self.embedding {
                Some(model_info) => { // semantic search
                    // load embedding model
                    let embedding_model = embedding_lib::EmbeddingModel::load_model(
                        &model_info.model_type,
                        &model_info.model_path, // model.safetensors
                        &model_info.config_path, // config.json
                        &model_info.tokenizer_path, // tokenizer.json
                        model_info.use_cpu,
                    ).map_err(|e| MyError::EmbeddingError{error: e})?;
                    // calculate embedding
                    let kw_embedding = embedding_model.get_embedding(&kw).map_err(|e| MyError::EmbeddingError{error: e})?;
                    let mut snippets_similarity: Vec<(usize, f32)> = Vec::new();
                    let mut discription_embedding: Vec<f32>;
                    for (i, s) in self.data.iter().enumerate() {
                        if let Some(categ) = &categories {
                            if !categ.iter().all(|c| s.tags.contains(c)) {
                                continue
                            }
                        }
                        if let Some(snip_embed) = &s.embedding {
                            if let Some(embed) = snip_embed.get(&model_info.model) {
                                snippets_similarity.push((i, cosine_similarity(&kw_embedding, &embed)?));
                            } else {
                                discription_embedding = embedding_model.get_embedding(&s.discription.replace("\n", "")).map_err(|e| MyError::EmbeddingError{error: e})?;
                                snippets_similarity.push((i, cosine_similarity(&kw_embedding, &discription_embedding)?));
                            }
                        } else {
                            discription_embedding = embedding_model.get_embedding(&s.discription.replace("\n", "")).map_err(|e| MyError::EmbeddingError{error: e})?;
                            snippets_similarity.push((i, cosine_similarity(&kw_embedding, &discription_embedding)?));
                        }
                    }
                    // sort by similarity
                    snippets_similarity.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                    let top_num = model_info.top_num.min(snippets_similarity.len());
                    //println!("top {} similarity results:\nid\tsimilarity    discription", top_num);
                    for (i, s) in &snippets_similarity[0..top_num] {
                        //println!("{}\t{:.4}        {:}", i, s, self.data[*i].discription.replace("\n", ", "));
                        snippets.push((*i, self.data[*i].clone(), Some(*s)));
                    }
                },
                None => { // keyword search
                    for (i, s) in self.data.iter().enumerate() {
                        if let Some(categ) = &categories {
                            if !categ.iter().all(|c| s.tags.contains(c)) {
                                continue
                            }
                        }
                        if s.discription.to_lowercase().replace("\n", "").contains(&kw) || s.content.to_lowercase().contains(&kw) {
                            snippets.push((i, s.clone(), None));
                        }
                    }
                },
            }
        } else {
            for (i, s) in self.data.iter().enumerate() {
                if let Some(categ) = &categories {
                    if !categ.iter().all(|c| s.tags.contains(c)) {
                        continue
                    }
                }
                if s.discription.to_lowercase().replace("\n", "").contains(&kw) || s.content.to_lowercase().contains(&kw) {
                    snippets.push((i, s.clone(), None));
                }
            }
        }
        Ok(snippets)
    }

    /// get snippets by -i, -t, -e
    pub fn get(&self, paras: ParsedParas) -> Result<(), MyError> {
        let selected_snippets = if !paras.ids.is_empty() {
            self.get_by_ids(&paras.ids)?
        } else {
            if let (true, Some(keyword)) = (!paras.tags.is_empty(), &paras.search) { // -t and -e can be used simultaneously
                //self.get_by_categories_and_search(paras.tags, &keyword)?
                self.get_by_search(Some(paras.tags), &keyword)?
            } else if !paras.tags.is_empty() {
                self.get_by_categories(&paras.tags)?
            } else if let Some(keyword) = &paras.search {
                self.get_by_search(None, &keyword)?
            } else {
                unreachable!()
            }
        };

        // save
        if paras.save {
            for (id, s, _) in &selected_snippets {
                let (comment, fmt) = SnipTag::commit_format(&s.tags);
                let out_file = paras.outpath.join(&format!("{}.{}", id, fmt));
                let mut writer = my_writer(&out_file)?;
                if !comment.is_empty() {
                    writer.write_all(comment.as_bytes())?;
                }
                writer.write_all(s.discription.replace("\n", "").as_bytes())?;
                writer.write_all(b"\n")?;
                writer.write_all(s.content.trim().replace("\r", "").as_bytes())?;
                writer.write_all(b"\n")?;
            }
        }

        // cp to clipboard
        if paras.clipboard {
            let mut clipboard = Clipboard::new().map_err(|e| MyError::ClipboardError{error: e})?;
            let mut all_content = "".to_string();
            for (_, s, _) in &selected_snippets {
                all_content += &s.content.trim().replace("\r", "");
                all_content += "\n\n";
            }
            clipboard.set_text(all_content).map_err(|e| MyError::ClipboardError{error: e})?;
        }

        // print each detail result
        let mut merge: Vec<usize> = Vec::new();
        let mut idx = 0;
        let mut builder = Builder::default();
        for (i, s, _) in &selected_snippets {
            builder.push_record(vec!["id", "discription", "categories"]);
            builder.push_record(vec![&i.to_string(), &s.discription, &s.format_tags()]);
            idx += 2;
            builder.push_record(vec![&s.content.trim().replace("\r", ""), "", ""]);
            merge.push(idx);
            idx += 1;
        }
        let mut table = builder.build();
        table.with(Style::modern()); // table style: ascii, extended, markdown, re_structured_text, dots, psql, ascii_rounded, blank, empty, rounded, modern, sharp
        for i in merge {
            table.modify((i, 0), Span::column(3));
            table.modify((i, 0), BorderColor::filled(Color::FG_BRIGHT_RED)); // `FG_RED` not work in Windows Cmder
            table.modify((i, 1), BorderColor::filled(Color::FG_BRIGHT_RED));
            table.modify((i, 2), BorderColor::filled(Color::FG_BRIGHT_RED));
        }
        println!("{}", table);
        // print summary
        if selected_snippets.len() > 1 || (selected_snippets.len() > 0 && selected_snippets[0].2.is_some()) {
            let mut builder = Builder::default();
            if selected_snippets.len() > 0 && selected_snippets[0].2.is_some() {
                builder.push_record(vec!["id", "similarity", "discription", "categories"]);
            } else {
                builder.push_record(vec!["id", "discription", "categories"]);
            }
            for (i, s, similarity) in selected_snippets {
                if let Some(simi) = similarity {
                    builder.push_record(vec![&i.to_string(), &format!("{:.4}", simi), &s.discription, &s.format_tags()]);
                } else {
                    builder.push_record(vec![&i.to_string(), &s.discription, &s.format_tags()]);
                }
            }
            let mut table = builder.build();
            table.with(Style::ascii()); // table style: ascii, extended, markdown, re_structured_text, dots, psql, ascii_rounded, blank, empty, rounded, modern, sharp
            println!("{}", table);
        }
        Ok(())
    }

    /// print all snippets summary
    pub fn print_summary(&self, categories: &Vec<SnipTag>) -> Result<(), MyError> {
        let mut stat: HashMap<SnipTag, usize> = HashMap::new(); // key: SnipTag, value: count
        let mut builder = Builder::default();
        builder.push_record(vec!["id", "discription", "categories"]);
        for (i, s) in self.data.iter().enumerate() {
            if categories.iter().any(|c| s.tags.contains(c)) {
                for tag in &s.tags {
                    match stat.get_mut(tag) {
                        Some(c) => *c += 1,
                        None => {
                            stat.insert(tag.clone(), 1);
                        },
                    }
                }
                builder.push_record(vec![&i.to_string(), &s.discription, &s.format_tags()]);
            }
        }
        let mut table = builder.build();
        table.with(Style::modern()); // table style: ascii, extended, markdown, re_structured_text, dots, psql, ascii_rounded, blank, empty, rounded, modern, sharp
        println!("{}", table);
        // print each tag number
        let mut tags_count: Vec<(String, usize)> = stat.iter().map(|(k, v)| (k.to_string(), *v)).collect();
        tags_count.sort_by(|a, b| b.1.cmp(&a.1)); // sort by tag count
        let mut builder = Builder::default();
        builder.push_record(vec!["categories", "count"]);
        for t in tags_count {
            builder.push_record(vec![&t.0, &t.1.to_string()]);
        }
        let mut table = builder.build();
        table.with(Style::ascii()); // table style: ascii, extended, markdown, re_structured_text, dots, psql, ascii_rounded, blank, empty, rounded, modern, sharp
        println!("{}", table);
        Ok(())
    }
}

