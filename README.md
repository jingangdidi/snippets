# snippets
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jingangdidi/snippets/blob/main/LICENSE)

[中文文档](https://github.com/jingangdidi/snippets/blob/main/README_zh.md)

**A lightweight, cross-platform command-line tool for storing and retrieving code snippets, commands, installation notes, usage instructions, and text-based information, all within a single executable.**

**轻量级命令行代码块工具，无需安装，仅一个可执行文件，记录自己常用的代码块、软件安装说明、常用命令等，在命令行快速检索**

## 👑 Features
- 💪 Single-file executable - no installation required
- 🎨 Multiple search methods: ID, category tags, keyword search, semantic search
- 💡 Support command-line printing, copying to clipboard, and saving to file
- 🤖 Semantic search supports multiple embedding models
- 💻​ Semantic search supports CPU, CUDA, Metal, and Windows, Linux, MacOS

## 🚀 Quick-Start
**1. download a pre-built binary**

  [latest release](https://github.com/jingangdidi/snippets/releases)

  This pre-compiled file only supports the default 11 classification tags, which means that only these 11 tags can be used in your `.snippets` files:
  ```
  Code, Command, Doc, Git, Manual, Note, Other, Python, Rust, Shell, Tool
  ```
  If you want to customize tags, you need to compile from source. During compilation, the tags used will be extracted from your `.snippets` files under the `snippets_database` path and compiled into the program.

**2. prepare your snippets file**

- Use `.snippets` as the format suffix to distinguish it from other files.
- `tags`: Fill in classification tags, which can be multiple with capitalized first letters, such as Code, Command, Note, Python, Rust.
- `description`: Fill in a brief description information, and semantic search will calculate similarity with the description information.
- `content`: Fill in specific content, such as code blocks, placed between `r##` and `"##` without escaping.
- If using a pre-built binary, place all prepared `.snippets` files in the current path, or in the same path as the program, or in the path specified by `-f`.
- If you compile it yourself, place all the `.snippets` files you have prepared in the `snippets_database` path. During compilation, they will be integrated into the `default.snippets` file and compiled into the program. When used, they will no longer depend on these `.snippets` files.

Example files can be found in [example.snippets](https://github.com/jingangdidi/snippets/blob/main/snippets_database/example.snippets)
```
[
    SingleSnippet(
        tags:        ["Code", "Python"],
        discription: "Python hello world",
        content:     "print('hello world')",
    ),
    SingleSnippet(
        tags:        ["Code", "Rust"],
        discription: "Rust hello world \n second line",
        content:     r##"
fn main() {
    println!("hello world");
}
"##,
    ),
    SingleSnippet(
        tags:        ["Command", "Manual"],
        discription: r##"
install python package
by "pip"
"##,
        content:     r##"
pip list
pip install package_name
pip uninstall package_name
"##,
    ),
]
```

**3. Download the embedding model (optional, to be used for semantic search)**

Support the following 11 models, multiple can be downloaded and placed in the `embedding_models` path:
1. [granite-embedding-small-english-r2](https://huggingface.co/ibm-granite/granite-embedding-small-english-r2)
2. [granite-embedding-english-r2](https://huggingface.co/ibm-granite/granite-embedding-english-r2)
3. [granite-embedding-107m-multilingual](https://huggingface.co/ibm-granite/granite-embedding-107m-multilingual)
4. [granite-embedding-278m-multilingual](https://huggingface.co/ibm-granite/granite-embedding-278m-multilingual)
5. [all-MiniLM-L6-v2](https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2)
6. [all-MiniLM-L12-v2](https://huggingface.co/sentence-transformers/all-MiniLM-L12-v2)
7. [paraphrase-multilingual-MiniLM-L12-v2](https://huggingface.co/sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2)
8. [mxbai-embed-large-v1](https://huggingface.co/mixedbread-ai/mxbai-embed-large-v1)
9. [mxbai-embed-xsmall-v1](https://hf-mirror.com/mixedbread-ai/mxbai-embed-xsmall-v1)
10. [e5-base-v2](https://huggingface.co/intfloat/e5-base-v2)
11. [multilingual-e5-small](https://huggingface.co/intfloat/multilingual-e5-small)

The following example is downloaded to three models in the local `embedding_models` path. During compilation, each model in the `embedding_models` path will be used to calculate the embedding of the description information for each snippet in the `snippets_database` path. When compiled into the program, semantic search does not require runtime calculation.
```
embedding_models # The local model path is fixed to "./embedding_models" during compilation. When using, it can be specified through "-p", or the environment variable "SNIPPETS_MODEL_PATH" can be set, or it can be placed in the "embedding_models" folder in the same path as the program. The default is "./embedding_models"
 ├─ granite-embedding-107m-multilingual
 │   ├─ config.json
 │   ├─ model.safetensors
 │   └─ tokenizer.json
 ├─ paraphrase-multilingual-MiniLM-L12-v2
 │   ├─ config.json
 │   ├─ model.safetensors
 │   └─ tokenizer.json
 └─ multilingual-e5-small
     ├─ config.json
     ├─ model.safetensors
     └─ tokenizer.json
```

## 📖 Usage example

1. View all snippets and count of each category
    ```
    snippets -u all
    ```
    Command line display:
    ```
    # ┌─────┬────────────────────┬────────────────┐
    # │ id  │ discription        │ categories     │
    # ├─────┼────────────────────┼────────────────┤
    # │ 0   │ rust argh usage    │ Code, Rust     │
    # ├─────┼────────────────────┼────────────────┤
    # │ 1   │ rust chrono time   │ Code, Rust     │
    # ├─────┼────────────────────┼────────────────┤
    # │ ... │ ...                │ ...            │
    # ├─────┼────────────────────┼────────────────┤
    # │ 125 │ shell awk command  │ Command, Shell │
    # ├─────┼────────────────────┼────────────────┤
    # │ 126 │ shell grep command │ Command, Shell │
    # └─────┴────────────────────┴────────────────┘
    # +------------+-------+
    # | categories | count |
    # +------------+-------+
    # | Code       | 84    |
    # +------------+-------+
    # | R          | 33    |
    # +------------+-------+
    # | Shell      | 30    |
    # +------------+-------+
    # | Rust       | 29    |
    # +------------+-------+
    # | Command    | 14    |
    # +------------+-------+
    # | Python     | 5     |
    # +------------+-------+
    ```

2. Select based on the id (multiple IDs separated by `,`) and print the content
    ```
    snippets -i 27,29
    ```
    Command line display:
    ```
    # ┌────┬─────────────────────────────┬────────────┐
    # │ id │ discription                 │ categories │
    # ├────┼─────────────────────────────┼────────────┤
    # │ 26 │ string start end            │ Code, R    │
    # ├────┼─────────────────────────────┼────────────┤
    # │ startsWith(result, 'cluster')                 │
    # │ endsWith(result, '.txt')                      │
    # ├────┼─────────────────────────────┼────────────┤
    # │ id │ discription                 │ categories │
    # ├────┼─────────────────────────────┼────────────┤
    # │ 29 │ string upper case lowercase │ Code, R    │
    # ├────┼─────────────────────────────┼────────────┤
    # │ str.upper <- toupper(str)                     │
    # │                                               │
    # │ str.lower <- tolower(str)                     │
    # └────┴─────────────────────────────┴────────────┘
    # +----+-----------------------------+------------+
    # | id | discription                 | categories |
    # +----+-----------------------------+------------+
    # | 26 | string start end            | Code, R    |
    # +----+-----------------------------+------------+
    # | 29 | string upper case lowercase | Code, R    |
    # +----+-----------------------------+------------+
    ```

3. Select based on tags, labels are not case sensitive
    ```
    snippets -t r
    ```

4. Multiple tags are separated by `,` to select snippets that contain multiple specified tags at the same time
    ```
    snippets -t r,command
    ```

5. Using keyword search will search in the description and content, without distinguishing between uppercase and lowercase letters
    ```
    snippets -e "pandas"
    ```

6. `-t` and `-e` can be used together to narrow down the search scope
    ```
    snippets -t code -e "pandas"
    ```

7. When searching, use `-m` to specify the embedding model for semantic search. By default, print the top 5 (you can use `-n` to specify the number, or set the environment variable `SNIPPETS_NUM`) most similar snippets. `-m 1` indicates the use of the `granite-embedding-small-english-r2` model. You can use `-p` to specify the model path. If you do not specify `-p`, you will search in the `./embedding_models` folder in the current path, the `embedding_models` folder in the same path as the program, or the environment variable `SNIPPETS_MODEL_PATH`. If none of them exist, an error will be reported.
    ```
    snippets -e "python pandas usage" -m 1
    ```

8. If `cuda` or `metal` is used during compilation, GPU computing for embeddings will be prioritized. You can use `-C` to force CPU usage, or set the environment variable `SNIPPETS_CPU=true`
    ```
    snippets -e "python pandas usage" -m 1 -C
    ```

9. Use `-s` or set the environment variable `SNIPPETS_SAVE=true` to save the obtained snippets locally (with id as the file name, and write the main content, description information to the file)
    ```
    snippets -i 27,29 -s
    ```
    Generate the following two files:
    ```
    saved_snippets
     ├─ 27.r # tags contain "R", file name with a format suffix of `.r`, and "#" is added before the description as a comment
     └─ 29.r # tags contain "R", file name with a format suffix of `.r`, and "#" is added before the description as a comment
    ```

10. Specify `-c` or set the environment variable `SNIPPETS_CLIPBOARD=true` to copy the obtained snippets to the clipboard (only copy the main content, excluding id, description, and tags)
    ```
    snippets -i 27,29 -c
    ```

## 🛠 Building from source
- By default, CPU will be used, GPU will not be used, and embedding semantic search will not be used
  ```
  git clone https://github.com/jingangdidi/snippets.git
  cd snippets
  cargo build --release
  ```
- Using CPU and embedding semantic search
  ```
  cargo build --release --features embedding
  ```
- Windows and Linux use CUDA and embedding semantic search
  ```
  cargo build --release --features cuda
  ```
- MacOS uses Metal and embedding semantic search
  ```
  cargo build --release --features metal
  ```

## 🚥 Arguments
```
Usage: snippets [-i <id>] [-t <tag>] [-e <search>] [-f <file>] [-m <model>] [-p <model-path>] [-C] [-n <num>] [-u <summary>] [-s] [-c] [-o <outpath>]

command line snippets

Options:
  -i, --id          get snippets by id, multiple ids separated by commas
  -t, --tag         get snippets by tag, supported tags were written in snippets files or default 11 tags, multiple categories separated by commas
  -e, --search      get snippets by keyword search or semantic search (need -m embedding model)
  -f, --file        specify *.snippets files or path, multiple files separated by commas, you can also set the environment variable SNIPPETS_FILE to set this argument
  -m, --model       select one model for semantic search, valid for -e, support:
                    1(granite-embedding-small-english-r2),
                    2(granite-embedding-english-r2),
                    3(granite-embedding-107m-multilingual),
                    4(granite-embedding-278m-multilingual),
                    5(all-MiniLM-L6-v2),
                    6(all-MiniLM-L12-v2),
                    7(paraphrase-multilingual-MiniLM-L12-v2),
                    8(mxbai-embed-large-v1),
                    9(mxbai-embed-xsmall-v1),
                    10(e5-base-v2),
                    11(multilingual-e5-small), you can also set the environment variable SNIPPETS_MODEL to set this argument
  -p, --model-path  path of the model folder, valid for -m, default: ./embedding_models/, you can also set the environment variable SNIPPETS_MODEL_PATH to set this argument
  -C, --cpu         force the use of cpu, otherwise prioritize using the gpu, valid for -m, you can also set the environment variable SNIPPETS_CPU to set this argument
  -n, --num         the number of most similar results, valid for -m, default: 5, you can also set the environment variable SNIPPETS_NUM to set this argument
  -u, --summary     print selected snippets summary, support all and categories, multiple categories separated by commas
  -s, --save        save -i, -t, -e selected snippets to files, you can also set the environment variable SNIPPETS_SAVE=true to set this argument
  -c, --clipboard   copy to clipboard, you can also set the environment variable SNIPPETS_CLIPBOARD=true to set this argument
  -o, --outpath     output path, default: ./saved_snippets/, you can also set the environment variable SNIPPETS_OUTPATH to set this argument
  -h, --help        display usage information
```

## 💡 Note
- During compilation, all `.snippets` files in the `snippets_database` path will be read (except for `exmaple.snippets` and `default.snippets`), and the embeddings of the description information will be calculated using all models in the `embedding_models` path (if `--features embedding` is specified during compilation), merged and saved as `default.snippets`, and then compiled into the program as the default database. When used, there is no need to rely on any `.snippets` files. If the `snippets_database` path does not exist at compile time or does not contain `.snippets` files, only the default 11 tags are supported.
- If `-p` is not specified when using semantic search, the model files will be searched in the current path `./embedding_models` folder, the same path as the program's `embedding_models` folder, and the environment variable `SNIPPETS_MODEL_PATH`. If none of them exist, an error will be reported.
- You can use the `-f` parameter to specify `.snippets` files (multiple files separated by commas), or a path containing `.snippets` files (which will read all `.snippets` files under that path), ignoring the compiled default snippets in the program.
- If `-f` is not specified, these `.snippets` files will be automatically searched in the current path. If it is not found, it will be searched in the path where the program is located. If it is not found yet, the default sniplets compiled in the program will be used.
- `-i`, `-t`, `-e`, `-u` cannot be used simultaneously. A maximum of one can be used at a time, and an error message will be displayed if used simultaneously.
- `-t` and `-e` can be used simultaneously to search within specified categories.

## ⏰ changelog
- [2025.10.16] release v0.1.0
