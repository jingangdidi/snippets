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

  This precompiled file only supports the default 11 classification tags, which means that only these 11 tags can be used in your `.snippets` files:
  ```
  Code, Command, Doc, Git, Manual, Note, Other, Python, Rust, Shell, Tool
  ```
  If you want to customize tags, you need to compile from source. During compilation, the tags used will be extracted from your `.snippets` files under the `snippets_database` path and compiled into the program.

**2. prepare your snippets file**

- Use `.snippets` as the format suffix to distinguish it from other files
- `tags`: Fill in classification tags, which can be multiple with capitalized first letters, such as Code, Command, Note, Python, Rust.
- `description`: Fill in a brief description information, and semantic search will calculate similarity with the description information.
- `content`: Fill in specific content, such as code blocks, placed between `r##` and `"##` without escaping.
- If using a pre-built binary, place all prepared `.snippets` files in the current path, or in the same path as the program, or in the path specified by `-f`.
- 如果自己编译，则将自己准备的所有`.snippets`文件放到`snippets_database`路径下，编译时会整合到`default.snippets`中编译到程序内，使用时不再依赖`.snippets`文件
- If you compile it yourself, place all the `.snippets` files you have prepared in the `snippets_database` path. During compilation, they will be integrated into the `default.snippets` file and compiled into the program. When used, they will no longer depend on the `.snippets` files.

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

Support the following 11 models, multiple can be downloaded and placed in the `embedding_madels` path:
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

The following example is downloaded to three models in the local `embedding_madels` path. During compilation, each model in the `embedding_madels` path will be used to calculate the embedding of the description information for each snippet in the `snipets_database` path. When compiled into the program, semantic search does not require runtime calculation.
```
embedding_models # The local model path is fixed to "./embedding_madels" during compilation. When using, it can be specified through "-p", or the environment variable "SNIPPETS_MODEL_PATH" can be set, or it can be placed in the "embedding_madels" folder in the same path as the program. The default is "./embedding_madels"
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

1. 查看所有snippets（id、描述信息、分类），并统计每种分类的数量
    ```
    snippets -u all
    ```
    命令行显示：
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

2. 根据id选择（多个id用`,`间隔），打印具体内容
    ```
    snippets -i 27,29
    ```
    命令行显示：
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

3. 根据分类标签选择，标签不区分大小写，打印具体内容
    ```
    snippets -t r
    ```

4. 多个标签用`,`间隔，选取同时含有指定的多个标签的snippets
    ```
    snippets -t r,command
    ```

5. 使用关键词搜索，会在discription描述信息和content具体内容中搜索，不区分大小写
    ```
    snippets -e "pandas"
    ```

6. -t和-e可以联合使用，缩小搜索范围
    ```
    snippets -t code -e "pandas"
    ```

7. 搜索时使用`-m`指定embedding模型，则进行语义搜索，默认打印前5个（可以使用`-n`指定数量，或设置环境变量`SNIPPETS_NUM`）相似度最高的snippets。`-m 1`表示使用`granite-embedding-small-english-r2`模型。可使用`-p`指定模型路径，不指定`-p`则在当前路径`./embedding_models`文件夹、程序同路径下`embedding_models`文件夹、环境变量`SNIPPETS_MODEL_PATH`搜索，都不存在则报错
    ```
    snippets -e "python pandas usage" -m 1
    ```

8. 如果编译时使用了`cuda`或`metal`，则优先使用GPU计算embedding，可使用`-C`强制使用CPU，或设置环境变量`SNIPPETS_CPU=true`
    ```
    snippets -e "python pandas usage" -m 1 -C
    ```

9. 使用`-s`或设置环境变量`SNIPPETS_SAVE=true`，将获取的snippets保存至本地（以id为文件名，主体内容、描述信息、分类标签写入到文件中）
    ```
    snippets -i 27,29 -s
    ```
    生成以下2个文件：
    ```
    saved_snippets
     ├─ 27.r # 分类标签含有R，格式后缀为`.r`，描述信息前加上`# `作为注释
     └─ 29.r # 分类标签含有R，格式后缀为`.r`，描述信息前加上`# `作为注释
    ```

10. 指定`-c`或设置环境变量`SNIPPETS_CLIPBOARD=true`，将获取的snippets复制到剪切板（只复制主体内容，不包含id、描述信息、分类标签）
    ```
    snippets -i 27,29 -c
    ```

## 🛠 从源码编译
- 默认使用CPU，不使用GPU，不使用embedding语义搜索
  ```
  git clone https://github.com/jingangdidi/snippets.git
  cd snippets
  cargo build --release
  ```
- 使用CPU，使用embedding语义搜索
  ```
  cargo build --release --features embedding
  ```
- Windows和Linux使用CUDA，使用embedding语义搜索
  ```
  cargo build --release --features cuda
  ```
- MacOS使用Metal，使用embedding语义搜索
  ```
  cargo build --release --features metal
  ```

## 🚥 命令行参数
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

## 💡 注意
- 编译时会读取`./snippets_database`路径下所有`*.snippets`文件（除了`exmaple.snippets`和`default.snippets`），并用`./embedding_models`路径下所有模型计算discription描述信息的embedding（如果编译时指定了`--features embedding`），合并保存为`default.snippets`，然后编译到程序中作为默认库，使用时就不需要依赖`.snippets`文件了。如果编译时`./snippets_database`路径不存在，或其中不含有`*.snippets`文件，则仅支持默认的11个tag标签
- 使用语义搜索时如果不指定`-p`，则会依次在当前路径`./embedding_models`文件夹、程序同路径下`embedding_models`文件夹、环境变量`SNIPPETS_MODEL_PATH`搜索模型文件，都不存在则报错
- 可以通过`-f`参数指定`.snippets`文件（多个之间`,`间隔），或含有`.snippets`文件的路径（读取该路径下所有`.snippets`文件），覆盖编译在程序内的snippets
- 如果不指定`-f`，会自动在当前路径下搜索`.snippets`文件，没有搜索到则在程序所在路径下搜索，还没有搜索到则会使用默认编译在程序内的`default.snippets`
- 含有中文时，Windows下Cmder显示的表格会对不齐，可修改设置：
  ```
  General --> Fonts --> 去掉勾选的“Compress long  string to fit space”
  ```
- `-i`, `-t`, `-e`, `-u`不能同时使用，每次最多使用一个，同时使用则显示报错
- `-t`和`-e`可以同时使用，在指定分类中搜索

## ⏰ 更新记录
- [2025.10.16] release v0.1.0
