# Minigrep

Minigrep created following the book `The Rust Programming Language`.

## Usage

Specify the string to search and them the file input:
```
cargo run -- Severino morte-e-vida-severina.txt

O meu nome é Severino
Como há muitos Severinos
Severino de Maria
Como há muitos Severinos
```

If file is `-` or no file specified, reads from the `stdin`.

Setting the environment variable `IGNORE_CASE` will make it ignore the case:
```
IGNORE_CASE=1 cargo run -- que morte-e-vida-severina.txt

Que é santo de romaria
Fiquei sendo o da Maria
Que se chamou Zacarias
E que foi o mais antigo
Como então dizer quem fala
```
