use std::env::{self, current_exe, VarError};
use std::fs::create_dir_all;
use std::path::PathBuf;

use argh::FromArgs;
use embedding_lib::{Model, ModelType};

use crate::{
    //snip::SnipTag,
    SnipTag,
    error::MyError,
};

#[derive(FromArgs)]
#[argh(help_triggers("-h", "--help"))] // https://github.com/google/argh/pull/106
/// command line snippets
struct Paras {
    /// get snippets by id, multiple ids separated by commas
    #[argh(option, short = 'i')]
    id: Option<String>,

    /// get snippets by tag, supported tags were written in snippets files or default 11 tags, multiple categories separated by commas
    #[argh(option, short = 't')]
    tag: Option<String>,

    /// get snippets by keyword search or semantic search (need -m embedding model)
    #[argh(option, short = 'e')]
    search: Option<String>,

    /// specify *.snippets files or path, multiple files separated by commas, you can also set the environment variable SNIPPETS_FILE to set this argument
    #[argh(option, short = 'f')]
    file: Option<String>,

    /// select one model for semantic search, valid for -e, support: 1(granite-embedding-small-english-r2), 2(granite-embedding-english-r2), 3(granite-embedding-107m-multilingual), 4(granite-embedding-278m-multilingual), 5(all-MiniLM-L6-v2), 6(all-MiniLM-L12-v2), 7(paraphrase-multilingual-MiniLM-L12-v2), 8(mxbai-embed-large-v1), 9(mxbai-embed-xsmall-v1), 10(e5-base-v2), 11(multilingual-e5-small), you can also set the environment variable SNIPPETS_MODEL to set this argument
    #[argh(option, short = 'm')]
    model: Option<usize>,

    /// path of the model folder, valid for -m, default: ./embedding_models/, you can also set the environment variable SNIPPETS_MODEL_PATH to set this argument
    #[argh(option, short = 'p')]
    model_path: Option<String>,

    /// force the use of cpu, otherwise prioritize using the gpu, valid for -m, you can also set the environment variable SNIPPETS_CPU to set this argument
    #[argh(switch, short = 'C')]
    cpu: bool,

    /// the number of most similar results, valid for -m, default: 5, you can also set the environment variable SNIPPETS_NUM to set this argument
    #[argh(option, short = 'n')]
    num: Option<usize>,

    /// print selected snippets summary, support all and categories, multiple categories separated by commas
    #[argh(option, short = 'u')]
    summary: Option<String>,

    /// save -i, -t, -e selected snippets to files, you can also set the environment variable SNIPPETS_SAVE="true" to set this argument
    #[argh(switch, short = 's')]
    save: bool,

    /// copy to clipboard, you can also set the environment variable SNIPPETS_CLIPBOARD="true" to set this argument
    #[argh(switch, short = 'c')]
    clipboard: bool,

    /// print all supported tags
    #[argh(switch, short = 'T')]
    show_tags: bool,

    /// output path, default: ./saved_snippets/, you can also set the environment variable SNIPPETS_OUTPATH to set this argument
    #[argh(option, short = 'o')]
    outpath: Option<String>,
}

/// model info
#[derive(Clone)]
pub struct ModelInfo {
    pub model:          Model,
    pub model_type:     ModelType,
    pub model_path:     PathBuf,
    pub config_path:    PathBuf,
    pub tokenizer_path: PathBuf,
    pub top_num:        usize, // the number of most similar results
    pub use_cpu:        bool,
}

/// parsed paras
pub struct ParsedParas {
    pub ids:       Vec<usize>,        // get snippets by id, multiple ids separated by commas
    pub tags:      Vec<SnipTag>,      // get snippets by tag, supported tags were written in snippets files or default 11 tags, multiple categories separated by commas
    pub search:    Option<String>,    // get snippets by search keyword
    pub files:     Vec<PathBuf>,      // if use -f specify *.snippets files, will not search from current path and binary file path
    pub model:     Option<ModelInfo>, // selected model, model.safetensors, config.json, tokenizer.json, the number of most similar results
    pub save:      bool,              // save -i or -t or -s to files, if not use -s, will print to terminal
    pub clipboard: bool,              // copy -i or -t or -s to clipboard
    pub show_tags: bool,              // print all supported tags
    pub summary:   Vec<SnipTag>,      // print selected snippets summary
    pub outpath:   PathBuf,           // save to this path, default: ./saved_snippets/
}

/// 解析参数
pub fn parse_para() -> Result<ParsedParas, MyError> {
    let para: Paras = argh::from_env();
    let top_num = match para.num {
        Some(n) => {
            if n == 0 {
                return Err(MyError::ParaError{para: "-n must > 0".to_string()})
            }
            n
        },
        None => match EnvVarValue::Usize(5).get_env_var("SNIPPETS_NUM")? {
            EnvVarValue::Usize(n) => n,
            _ => unreachable!(),
        },
    };
    let use_cpu = if para.cpu {
        true
    } else {
        match EnvVarValue::Bool(false).get_env_var("SNIPPETS_CPU")? {
            EnvVarValue::Bool(c) => c,
            _ => unreachable!(),
        }
    };
    let out: ParsedParas = ParsedParas{
        ids: match para.id {
            Some(ids) => {
                let mut ids_vec: Vec<usize> = Vec::new();
                for i in ids.split(",") {
                    match i.parse::<usize>() {
                        Ok(idx) => ids_vec.push(idx),
                        Err(e) => return Err(MyError::ParseStringError{from: i.to_string(), to: "usize".to_string(), error: e}),
                    }
                }
                ids_vec
            },
            None => Vec::new(),
        },
        tags: match para.tag {
            Some(c) => {
                let mut category_vec: Vec<SnipTag> = Vec::new();
                for i in c.split(",") {
                    let tmp_category = i.to_lowercase();
                    if let Some(t) = SnipTag::string_to_tag(&tmp_category) {
                        category_vec.push(t);
                    } else {
                        return Err(MyError::ParaError{para: format!("snippet tag only support: {}, not {}", SnipTag::supported_tags(), i)})
                    }
                }
                category_vec
            },
            None => Vec::new(),
        },
        search: para.search,
        files: match para.file {
            Some(f) => get_snippet_files(&f)?,
            None => match EnvVarValue::VecPath(Vec::new()).get_env_var("SNIPPETS_FILE")? {
                EnvVarValue::VecPath(f) => f,
                _ => unreachable!(),
            },
        },
        model: match para.model {
            Some(m) => {
                let p = match &para.model_path {
                    Some(p) => p.clone(),
                    None => {
                        // get model from path of the current running executable
                        let binary_path = match current_exe() {
                            Ok(mut binary_path) => {
                                if binary_path.pop() { // Truncates binary_path to parent
                                    binary_path.join("embedding_models").to_str().unwrap().to_string()
                                } else {
                                    "./embedding_models/".to_string()
                                }
                            },
                            Err(_) => "./embedding_models/".to_string(),
                        };
                        // get model from env
                        let current_path_or_env = match EnvVarValue::Str("./embedding_models/".to_string()).get_env_var("SNIPPETS_MODEL_PATH")? {
                            EnvVarValue::Str(p) => p,
                            _ => unreachable!(),
                        };
                        // check modle exist, priority order: ./embedding_models/ > embedding_models in path of the current running executable > SNIPPETS_MODEL_PATH
                        match (check_model_exist("./embedding_models/", m).is_ok(), check_model_exist(&binary_path, m).is_ok(), check_model_exist(&current_path_or_env, m).is_ok()) {
                            (true, _, _) => "./embedding_models/".to_string(),
                            (false, true, _) => binary_path,
                            (false, false, true) => current_path_or_env,
                            (false, false, false) => return Err(MyError::ParaError{para: format!(r#"couldn't find model in "./embedding_models/", "{}" and "SNIPPETS_MODEL_PATH""#, binary_path)}),
                        }
                    },
                };
                let (model, model_type, model_path, config_path, tokenizer_path) = check_model_exist(&p, m)?;
                Some(ModelInfo{
                    model,
                    model_type,
                    model_path,
                    config_path,
                    tokenizer_path,
                    top_num,
                    use_cpu,
                })
            },
            None => None,
        },
        summary: match para.summary {
            Some(s) => {
                let lowercase_str = s.to_lowercase();
                if lowercase_str == "all" {
                    SnipTag::all_tags()
                } else {
                    let mut selected_snippets: Vec<SnipTag> = Vec::new();
                    for i in lowercase_str.split(",") {
                        if let Some(t) = SnipTag::string_to_tag(i) {
                            selected_snippets.push(t);
                        } else {
                            return Err(MyError::ParaError{para: format!("snippet category only support: {}, not {}", SnipTag::supported_tags(), i)})
                        }
                    }
                    selected_snippets
                }
            },
            None => Vec::new(),
        },
        save: if para.save {
            true
        } else {
            match EnvVarValue::Bool(false).get_env_var("SNIPPETS_SAVE")? {
                EnvVarValue::Bool(s) => s,
                _ => unreachable!(),
            }
        },
        clipboard: if para.clipboard {
            true
        } else {
            match EnvVarValue::Bool(false).get_env_var("SNIPPETS_CLIPBOARD")? {
                EnvVarValue::Bool(c) => c,
                _ => unreachable!(),
            }
        },
        show_tags: para.show_tags,
        outpath: match para.outpath {
            Some(o) => PathBuf::from(&o),
            None => match EnvVarValue::Path(PathBuf::from("./saved_snippets/")).get_env_var("SNIPPETS_OUTPATH")? {
                EnvVarValue::Path(p) => p,
                _ => unreachable!(),
            },
        },
    };
    if !out.show_tags {
        // cannot use -i, -t, -e, -u simultaneously
        // -t and -e can be used simultaneously
        match (out.ids.is_empty(), out.tags.is_empty(), out.search.is_none(), out.summary.is_empty()) {
            (true,  true,  true,  true)  => return Err(MyError::ParaError{para: "you must specify one of -i, -t, -e, -u".to_string()}),
            (true,  true,  true,  false) => (),
            (true,  true,  false, true)  => (),
            (true,  true,  false, false) => return Err(MyError::ParaError{para: "cannot use -e and -u simultaneously".to_string()}),
            (true,  false, true,  true)  => (),
            (true,  false, true,  false) => return Err(MyError::ParaError{para: "cannot use -t and -u simultaneously".to_string()}),
            (true,  false, false, true)  => (), // -t and -e
            (true,  false, false, false) => return Err(MyError::ParaError{para: "cannot use -t, -e and -u simultaneously".to_string()}),
            (false, true,  true,  true)  => (),
            (false, true,  true,  false) => return Err(MyError::ParaError{para: "cannot use -i and -u simultaneously".to_string()}),
            (false, true,  false, true)  => return Err(MyError::ParaError{para: "cannot use -i and -e simultaneously".to_string()}),
            (false, true,  false, false) => return Err(MyError::ParaError{para: "cannot use -i, -e and -u simultaneously".to_string()}),
            (false, false, true,  true)  => return Err(MyError::ParaError{para: "cannot use -i and -t simultaneously".to_string()}),
            (false, false, true,  false) => return Err(MyError::ParaError{para: "cannot use -i, -t and -u simultaneously".to_string()}),
            (false, false, false, true)  => return Err(MyError::ParaError{para: "cannot use -i, -t and -e simultaneously".to_string()}),
            (false, false, false, false) => return Err(MyError::ParaError{para: "cannot use -i, -t, -e and -u simultaneously".to_string()}),
        }
        // if not use embedding feature, ignore -m, -p, -C, -n
        if !cfg!(feature = "embedding") {
            if out.model.is_some() {
                println!("Warning - -m is only valid for embedding feature");
            }
            if para.model_path.is_some() {
                println!("Warning - -p is only valid for embedding feature");
            }
            if para.cpu {
                println!("Warning - -C is only valid for embedding feature");
            }
            if para.num.is_some() {
                println!("Warning - -n is only valid for embedding feature");
            }
        } else {
            // -m is only valid for -e
            if out.search.is_none() && out.model.is_some() {
                println!("Warning - -m is only valid for -e");
            }
            if out.model.is_none() {
                if para.model_path.is_some() {
                    println!("Warning - -p is only valid for -m");
                }
                if para.cpu {
                    println!("Warning - -C is only valid for -m");
                }
                if para.num.is_some() {
                    println!("Warning - -n is only valid for -m");
                }
            }
        }
        // if save, create output path
        if out.save && !(out.outpath.exists() && out.outpath.is_dir()) {
            if let Err(err) = create_dir_all(&out.outpath) {
                return Err(MyError::CreateDirAllError{dir_name: out.outpath.to_str().unwrap().to_string(), error: err})
            }
        }
    }
    Ok(out)
}

/// check path, model.safetensors, config.json, tokenizer.json exist
/// return (model.safetensors path, config.json path, tokenizer.json path)
fn check_model_exist(path_str: &str, id: usize) -> Result<(Model, ModelType, PathBuf, PathBuf, PathBuf), MyError> {
    let base_path = PathBuf::from(path_str);
    let model = Model::new(id).map_err(|e| MyError::EmbeddingError{error: e})?;
    let (model_type, model_dir) = model.model_type_and_path(&base_path);
    let (model_path, config_path, tokenizer_path) = Model::check_model_files(&model_dir).map_err(|e| MyError::EmbeddingError{error: e})?;
    Ok((model, model_type, model_path, config_path, tokenizer_path))
}

/// environment variable value
enum EnvVarValue {
    Usize(usize),
    Str(String),
    Bool(bool),
    Path(PathBuf),
    VecPath(Vec<PathBuf>),
}

impl EnvVarValue {
    /// get environment variable
    fn get_env_var(self, var: &str) -> Result<Self, MyError> {
        match env::var(var) {
            Ok(str_var) => match &self {
                EnvVarValue::Usize(_) => {
                    match str_var.parse::<usize>() {
                        Ok(n) => {
                            if n == 0 {
                                return Err(MyError::ParaError{para: "environment variable SNIPPETS_NUM must > 0".to_string()})
                            }
                            Ok(EnvVarValue::Usize(n))
                        },
                        Err(e) => return Err(MyError::ParseStringError{from: str_var.to_string(), to: "usize".to_string(), error: e}),
                    }
                },
                EnvVarValue::Str(_) => Ok(EnvVarValue::Str(str_var)),
                EnvVarValue::Bool(_) => if str_var == "true" {
                    Ok(EnvVarValue::Bool(true))
                } else {
                    Ok(EnvVarValue::Bool(false))
                },
                EnvVarValue::Path(_) => Ok(EnvVarValue::Path(PathBuf::from(&str_var))),
                EnvVarValue::VecPath(_) => Ok(EnvVarValue::VecPath(get_snippet_files(&str_var)?)),
            },
            Err(e) => if let VarError::NotUnicode(s) = e {
                return Err(MyError::EnvVarError{info: format!("couldn't interpret {}: {:?}", var, s)})
            } else {
                Ok(self) // default value
            },
        }
    }
}

/// get *.snippets files from commas seperated string or one path
fn get_snippet_files(file: &str) -> Result<Vec<PathBuf>, MyError> {
    let mut files: Vec<PathBuf> = Vec::new();
    let tmp_dir = PathBuf::from(file);
    if tmp_dir.exists() && tmp_dir.is_dir() {
        if let Ok(dirs) = tmp_dir.read_dir() {
            for i in dirs {
                if let Ok(entry) = i {
                    let tmp_file_path = entry.path();
                    if tmp_file_path.is_file() {
                        if let Some(ext) = tmp_file_path.extension() {
                            if ext == "snippets" {
                                files.push(tmp_file_path);
                            }
                        }
                    }
                }
            }
        }
    } else {
        for i in file.split(",") {
            let tmp_file = PathBuf::from(i);
            if !(tmp_file.exists() && tmp_file.is_file()) {
                return Err(MyError::FileNotExistError{file: i.to_string()})
            }
            files.push(tmp_file);
        }
    }
    Ok(files)
}

