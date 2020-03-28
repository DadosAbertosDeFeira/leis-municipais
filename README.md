# leis-municipais

CLI para parsear arquivos HTML disponibilizados pelo [Leis Municipais](leismunicipais.com.br) e gerar um único arquivo 
em formato `json` com as mesmas.

# Rodando a aplicação (se você já tem o Rust instalado)

* vá para a pasta do projeto
* `cargo run <absolute_path_to_the_folder>` (e cria um arquivo `leis.json` na pasta raíz do projeto)

# Instalando o Rust e rodando a aplicação

* `curl https://sh.rustup.rs -sSf | sh` (deve demorar um pouco)
* vá para a pasta do projeto
* `cargo run <absolute_path_to_the_folder>` (e cria um arquivo `leis.json` na pasta raíz do projeto)

# Contribuindo

* para rodar todos testes: `cargo test`
* para rodar somente os testes unitários: `cargo test --bin ranbumfy`
* para rodar somente os testes de integração: `cargo test --test integration`
* para rodar somente um teste: `cargo test <TESTNAME>`
