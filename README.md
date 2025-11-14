

---

# binx

将任意 `.bin` 文件转换为可嵌入代码的 Shellcode 格式。

`binx` 支持输出 Rust 数组、C 数组、Base64、十六进制序列、十六进制 Dump，以及原始二进制格式，常用于逆向分析、漏洞利用、安全测试和嵌入式开发。

---

## ✨ Features

* 支持多种输出格式：

  * **Rust 数组 (`rs`)**
  * **C 数组 (`c`)**
  * **Hex 字节序列 (`hex`)**
  * **Hex Dump (`hex-dump`)**
  * **Base64 (`b64`)**
  * **Raw (`raw`)**
* 基于 Clap 的清晰命令行参数
* 友好的 ANSI 彩色日志：`[*]`、`[+]`、`[-]`
* 完全模块化，易于扩展更多格式

---

## 🚀 Installation

```bash
git clone https://github.com/yourname/binx
cd binx
cargo build --release
```

生成的可执行文件位于：

```
target/release/binx
```

建议加入 PATH。

---

## 🧰 Usage

`binx` 需要三个参数：

* `--input` / `-i`：输入二进制文件
* `--output` / `-o`：输出文件名
* `--format` / `-f`：输出格式

查看帮助：

```bash
binx -h
```

示例：

```
Usage: binx [OPTIONS] --input <INPUT> --output <OUTPUT> --format <FORMAT>

Options:
  -i, --input <INPUT>    Input binary file
  -o, --output <OUTPUT>  Output file name
  -f, --format <FORMAT>  Output format
      --help             Print help
      --version          Print version
```

---

## 📦 Formats

| 格式名        | 说明                               |
| ---------- | -------------------------------- |
| `rs`       | Rust 数组 `[u8; N]`                |
| `c`        | C 风格 `unsigned char shellcode[]` |
| `hex`      | `0xfc 0x48 ...` 样式字节序列           |
| `hex-dump` | 传统 `xxd` 风格十六进制 dump             |
| `b64`      | Base64 编码                        |
| `raw`      | 原样复制二进制                          |

---

## 📝 Examples

### 输出 Rust Shellcode 数组

```bash
binx -i payload.bin -o payload.rs -f rs
```

### 输出 C 版本 shellcode

```bash
binx -i payload.bin -o sc.c -f c
```

### 输出十六进制 dump

```bash
binx -i test.bin -o dump.txt -f hex-dump
```

### 输出 Base64

```bash
binx -i any.bin -o out.txt -f b64
```

---

## 📁 Project Structure

```
src/
├── main.rs            // 程序入口：流程控制、调度各模块
├── args.rs            // Clap 参数解析
├── util.rs            // ANSI 颜色日志 / 文件读写工具
├── format/            // 各种输出格式的模块
│   ├── rs.rs          // Rust shellcode 数组输出
│   ├── c.rs           // C unsigned char 数组输出
│   ├── hex.rs         // Hex 字节序列 & Hex Dump 输出
│   ├── base64.rs      // Base64 输出
│   └── raw.rs         // 原始二进制输出
```

