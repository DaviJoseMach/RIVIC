# Banco de dados Rust

Este repositório implementa um banco de dados com Rust usando 2 bancos de dados diferentes, Mongo e SQLite.

## Como usar

Para usar este banco de dados, você precisará instalar as seguintes dependências:

-   Rust
-   MongoDB
-   SQLite

Depois de instalar as dependências, você pode construir o banco de dados usando o seguinte comando:

```
cargo build

```

Depois que o banco de dados estiver construído, você pode iniciá-lo usando o seguinte comando:

```
cargo run

```

O banco de dados agora estará disponível no endereço `localhost:8080`.

## Como usar com Mongo

Para usar o banco de dados com Mongo, você precisará criar uma coleção no banco de dados Mongo chamada `users`. Você também precisará criar um documento na coleção `users` com os seguintes campos:

-   `id`: o ID do usuário
-   `name`: o nome do usuário
-   `email`: o endereço de e-mail do usuário

Depois de criar a coleção e o documento, você pode começar a usar o banco de dados para armazenar e recuperar dados de usuários.

## Como usar com SQLite

Para usar o banco de dados com SQLite, você precisará criar um banco de dados chamado `users.sqlite`. Você também precisará criar uma tabela no banco de dados `users.sqlite` com os seguintes campos:

-   `id`: o ID do usuário
-   `name`: o nome do usuário
-   `email`: o endereço de e-mail do usuário

Depois de criar a tabela, você pode começar a usar o banco de dados para armazenar e recuperar dados de usuários.

## Recursos

Este banco de dados oferece os seguintes recursos:

-   Armazenamento de dados
-   Recuperação de dados
-   Atualização de dados
-   Exclusão de dados

## Futuro

Planejo adicionar os seguintes recursos ao banco de dados no futuro:

-   Suporte para vários bancos de dados
-   Suporte para autenticação e autorização
-   Suporte para distribuição

## Contribuições

Sou grato por qualquer contribuição ao banco de dados. Se você tiver alguma sugestão ou correção, sinta-se à vontade para enviar uma solicitação de pull.

## Licença

Este banco de dados é licenciado sob a licença MIT.
