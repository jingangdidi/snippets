# snippets
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jingangdidi/snippets/blob/main/LICENSE)

[English readme](https://github.com/jingangdidi/snippets/blob/main/README.md)

**A lightweight, cross-platform command-line tool for storing and retrieving code snippets, commands, installation notes, usage instructions, and text-based information, all within a single executable.**

**轻量级命令行代码块工具，无需安装，仅一个可执行文件，记录自己常用的代码块、软件安装说明、常用命令等，在命令行快速检索**

## 👑 特点
- 💪 单个可执行文件，无需安装，命令行调用
- 🎨 多种检索方式：id、分类标签、关键词搜索、语义搜索
- 💡 支持命令行打印、复制到剪切板、保存至文件
- 🤖 语义搜索支持多种embedding模型
- 💻​ 语义搜索支持CPU、CUDA、Metal，支持Windows、Linux、MacOS

## 🚀 使用说明
**1. 下载预编译的可执行文件**

  [latest release](https://github.com/jingangdidi/snippets/releases)

  这个预编译的文件只支持默认的11个分类标签，即你的`.snippets`文件中`tags`只能用这11个标签：
  ```
  Code, Command, Doc, Git, Manual, Note, Other, Python, Rust, Shell, Tool
  ```
  如果要自定义标签，需要自己编译，编译时会从`snippets_database`路径下的`.snippets`文件的tags中提取用到的标签编译到程序中。

**2. 准备自己的snippets文件**

- 以`.snippets`为格式后缀，以便与其他文件进行区分
- `tags`填写分类标签，可以有多个，首字母大写，例如：Code、Command、Note、Python、Rust
- `discription`填写简短的描述信息，语义搜索时会与该描述信息计算相似度
- `content`填写具体内容，比如代码块，放在`r##"`和`"##`之间，不需要转义
- 如果使用预编译好的程序，则将自己准备的所有`.snippets`文件放到当前路径下，或程序同路径下，或`-f`指定的路径下
- 如果自己编译，则将自己准备的所有`.snippets`文件放到`snippets_database`路径下，编译时会整合到`default.snippets`中编译到程序内，使用时不再依赖`.snippets`文件

示例文件见[example.snippets](https://github.com/jingangdidi/snippets/blob/main/snippets_database/example.snippets)
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

**3. 下载embedding模型（可选，语义搜索要用）**

支持以下11种模型，可下载多个，然后放到`embedding_models`路径下：
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

以下示例下载至本地`embedding_models`路径下的3个模型，编译时会使用`./embedding_models`路径下的每个模型计算`./snippets_database`路径下每个snippet的描述信息的embedding，编译到程序内，语义搜索时就不需要运行时计算
```
embedding_models # 本地模型路径，编译时固定为"./embedding_models"。调用时可通过-p指定，也可以设置环境变量"SNIPPETS_MODEL_PATH"，或者放到程序同路径下的"embedding_models"文件夹中，默认./embedding_models
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

## 📖 使用示例

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

9. 使用`-s`或设置环境变量`SNIPPETS_SAVE=true`，将获取的snippets保存至本地（以id为文件名，主体内容和描述信息写入到文件中）
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
