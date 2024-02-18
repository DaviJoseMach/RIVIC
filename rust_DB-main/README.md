# Rust database

This repository implements a database with Rust using 2 different databases, Mongo and SQLite.

[Readme Brazil (pt-br)](https://github.com/LeviAckr/rust_DB/blob/main/pt.md)


## How to use

To use this database you will need to install the following dependencies:

- Rust
- MongoDB
- SQLite

After installing the dependencies, you can build the database using the following command:

```
cargo build

```

Once the database is built, you can start it using the following command:

```
cargo run

```

The database will now be available at `localhost:8080`.

## How to use with Mongo

To use the database with Mongo, you will need to create a collection in the Mongo database called `users`. You will also need to create a document in the `users` collection with the following fields:

- `id`: the user ID
- `name`: the username
- `email`: the user's email address

After creating the collection and document, you can start using the database to store and retrieve user data.

## How to use with SQLite

To use the database with SQLite, you will need to create a database called `users.sqlite`. You will also need to create a table in the `users.sqlite` database with the following fields:

- `id`: the user ID
- `name`: the username
- `email`: the user's email address

After creating the table, you can start using the database to store and retrieve user data.

## Resources

This database offers the following features:

-   Data storage
-   Data recovery
-   Data update
- Deletion of data

## Future

I plan to add the following features to the database in the future:

- Support for multiple databases
- Support for authentication and authorization
- Support for distribution

## Contributions

I am grateful for any contribution to the database. If you have any suggestions or corrections, feel free to submit a pull request.

## License

This database is licensed under the MIT license.
