# supermodel
[![https://img.shields.io/crates/v/supermodel](https://img.shields.io/badge/crates.io-supermodel-red)](https://crates.io/crates/supermodel)
[![https://img.shields.io/crates/v/supermodel__macros](https://img.shields.io/badge/crates.io-supermodel--macros-orange)](https://crates.io/crates/supermodel-macros)
[![https://img.shields.io/crates/v/supermodel__sqlx](https://img.shields.io/badge/crates.io-supermodel--sqlx-yellow)](https://crates.io/crates/supermodel-sqlx)

Supermodel is an abstract data-modeling library. It provides an interface for creating, editing, and modifying data in various types of databases and APIs.

## Goals
- **Be generic.** The library tries be as abstract as possible. This allows the same code to be used for multiple purposes. Moreover, this means prioritizing the Rust type system over the type system of a database implementation.
- **Be intuitive.** Supermodel is centered around the `Model` and `Dialect` trait. With an implementation of a Dialect (typically through a crate like `supermodel-sqlx`) and an implementation of a Model (typically through the `model` proc-macro), all of Supermodel's features can be accessed.
- **Be fast.** While I would not consider myself skilled at high-performance Rust, this library aims to be only a thin layer over its implementations when it comes to runtime performance.
 
## Examples

```rs
#[model(Postgres)]
#[name = "users"]
pub struct User {
    username: String,
    password: String
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().expect("initializing .env file");

    let url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL in env");
    let mut connection = Postgres::create_connection(&url).await?;

    User::register().execute(&mut connection).await?;

    Ok(())
}
```

Generated SQL:
```sql
create table if not exists users
(
    id       bigserial
             primary key,
    username varchar(30),
    password varchar(30)
);
```

Check out some more example programs written with Supermodel [here](https://github.com/peterhenryd/supermodel/tree/main/packages/supermodel/examples).

## Usability

Supermodel currently compiles, but is largely unusable. It only supports a handful of data types and many operations are unimplemented. Moreover, the API is very unstable as I haven't yet decided the best way to go about certain things.

## Roadmap

Check out the plans for the future of this library [here](https://github.competerhenryd/supermodel/tree/main/ROADMAP.md).
