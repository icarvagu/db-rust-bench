# Database Performance Tester

Este projeto é uma aplicação Rust para testar a performance de diferentes bancos de dados, incluindo MongoDB, MySQL, PostgreSQL e Redis. A aplicação executa operações de inserção, consulta, atualização e exclusão em cada banco de dados e mede o tempo de execução de cada operação.

## Índice

- [Requisitos](#requisitos)
- [Configuração e Execução](#configuração-e-execução)
- [Estrutura do Projeto](#estrutura-do-projeto)
- [Exemplos de Uso](#exemplos-de-uso)
- [Contribuição](#contribuição)
- [Licença](#licença)

## Requisitos

- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)
- [Rust](https://www.rust-lang.org/)

## Configuração e Execução

### 1. Subir os containers com Docker Compose

Certifique-se de que o Docker e Docker Compose estão instalados. Em seguida, execute:

```sh
docker-compose up -d  
```

## Funcionalidades

A aplicação executa as seguintes operações em cada banco de dados:

- `insert`: Insere registros no banco de dados.
- `queryAll`: Consulta todos os registros do banco de dados.
- `updateAll`: Atualiza todos os registros onde `UF` é igual a "SP" ou "SÃO".
- `deleteAll`: Deleta todos os registros do banco de dados.

## Estrutura do Projeto

- `src/main.rs`: Ponto de entrada da aplicação.
- `src/db/`: Contém os módulos para cada banco de dados (MongoDB, MySQL, PostgreSQL, Redis).
- `src/modal/csvStruct.rs`: Definição da estrutura `Record`.

## Compilar e Rodar a Aplicação

```sh
cargo build
cargo run
```

## Exemplos de Uso
![image](https://github.com/icarvagu/db-rust-bench/assets/171522394/ccab6e30-166f-43fa-87f0-d6a6b9312a3c)

## Contribuição

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues e pull requests.

# Licença

[![License: GPL v2](https://img.shields.io/badge/License-GPL%20v2-blue.svg)](https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html)
