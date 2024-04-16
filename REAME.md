# Rust Web Development

API Rest em Rust utilizando
- warp: framework web 
- tokio: runtime async (para servidor multithreading e 
  I/O não bloqueante)
- reqwest: para chamadas HTTP
- SQLx: driver para conectar Rust ao banco de dados 
  (sem ORM)

## Abrir a documentação do projeto
Rode o comando: `cargo doc --open`

O Rust cria documentação de tudo, todas as structs e funções
![](/assets/allitems.png)

Não só dos arquivos em que colocamos comentários especiais
![](/assets/question.png)

E nossos comentários especiais aparecem na documentação
![](/assets/pagination.png) 
![](/assets/extract.png)

## Buscar code smells no código com Clippy
instale o Clippy: `rustup component add clippy`

adicione esse trecho de código no topo do seu `main.rs` ou `lib.rs`: 

```
#![warn(clippy::all,)]
```

e rode o comando `cargo clean` 

e em seguida `cargo clippy`

## Formatar base de código com Rustfmt

instale o Rustfmt: `rustup component add rustfmt`

e rode o comando `cargo fmt` 

## Modularização do projeto
![](/assets/mods.png)