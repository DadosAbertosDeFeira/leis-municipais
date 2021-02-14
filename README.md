![Continuous integration](https://github.com/DadosAbertosDeFeira/leis-municipais/workflows/Continuous%20integration/badge.svg)

# leis-municipais 📖

CLI para parsear arquivos HTML disponibilizados pelo [Leis Municipais](https://www.leismunicipais.com.br)
e gerar um único arquivo em formato `json` com as mesmas.

**Importante**: o site [Leis Municipais](https://www.leismunicipais.com.br) nos forneceu os arquivos das
leis da cidade de Feira de Santana após um pedido feito a Câmara de Vereadores da cidade.
A casa solicitou ao site e eles enviaram um arquivo compactado com vários HTMLs. Essa CLI
parseia essas informações e disponibiliza em um JSON. Leia mais sobre isso [aqui](https://medium.com/@dadosabertosdefeira/acessando-todas-as-leis-do-leismunicipais-com-br-235c6719fecf).

Para ter acesso ao arquivo do [Leis Municipais](https://www.leismunicipais.com.br), você deve solicitar
a Câmara da sua cidade via Lei de Acesso à Informação. O arquivo com os HTMLs da cidade de Feira
de Santana pode ser encontrado [aqui](https://drive.google.com/open?id=1TRFx3bMMT7y5IDQ_DkgMVXHG_MLva133).

## Como executar

Você encontrará a última versão da CLI na aba de [_releases_](https://github.com/DadosAbertosDeFeira/leis-municipais/releases) desse projeto.
Temos versões para Linux e MacOS. Faça o _download_ da última versão e
**dê permissões para o executável**:

```
chmox +x leis-municipais-linux-amd64
```

Em seguida, é só executar, passando o caminho da pasta de leis como argumento:

```
./leis-municipais-linux-amd64 LeisMunicipaisFeiraDeSantana/
```

_Voilà_! Um arquivo chamado `leis.json` foi criado na mesma pasta em que você
executou o comando.

## Desenvolvimento

Todas as contribuições são bem-vindas! Para sugerir uma melhoria, funcionalidade ou cadastrar
um bug, abra uma nova [_issue_](https://github.com/DadosAbertosDeFeira/leis-municipais/issues).

### Instalando o Rust e rodando a aplicação

* `curl https://sh.rustup.rs -sSf | sh` (deve demorar um pouco)
* execute `rustc --version` para verificar se a instalação ocorreu bem. se não,
configure o `PATH`: `export PATH="$HOME/.cargo/bin:$PATH"` no arquivo de configuração do seu
terminal
* vá para a pasta do projeto
* `cargo run <caminho_absoluto_para_a_pasta>`

Pronto! O arquivo `leis.json` será criado na pasta raiz do projeto.

### Rodando a aplicação (se você já tem o Rust instalado)

* vá para a pasta do projeto
* `cargo run <caminho_absoluto_para_a_pasta>`

Pronto! O arquivo `leis.json` será criado na pasta raiz do projeto.

### Hooks

Para rodar os hooks antes de algum commit:

`git config core.hooksPath .githooks`

### Testes

Antes de abrir um _pull request_, não esqueça de rodar os testes.

* para rodar todos testes: `cargo test`
* para rodar somente os testes unitários: `cargo test --bin leis-municipais`
* para rodar somente os testes de integração: `cargo test --test integration`
* para rodar somente um teste: `cargo test <nome_do_teste>`
