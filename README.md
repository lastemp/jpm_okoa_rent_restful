# jpm_okoa_rent_restful

This RESTful Actix Web API is a USSD system accessed by clients through their phones to request an advance for rent or mortgage payments. 
The system intergrates with [AfricasTalking USSD Gateway API](https://developers.africastalking.com/) and exchange messages.

The RESTful Actix Web API has below listed dependencies:
- [Actix Web](https://github.com/actix/actix-web) web framework for Rust
- [Serde](https://github.com/serde-rs/serde) for serializing and deserializing Rust data structures
- [serde_json](https://github.com/serde-rs/json) for serializing and deserializing Rust data structures
- [Reqwest](https://github.com/seanmonstar/reqwest) Rust HTTP Client
- [base64](https://github.com/marshallpierce/rust-base64) Decode from Base64 format or encode into it
- [tokio](https://github.com/tokio-rs/tokio) A runtime for writing reliable, asynchronous applications
- [mysql](https://github.com/blackbeam/rust-mysql-simple) MySql database driver

## Usage

All the following commands assume that your current working directory is _this_ directory. I.e.:

```console
$ pwd
.../jpm_okoa_rent_restful
```

1. Create database, tables and stored-procedures:

   The `sql` directory contains the SQL files used for database setup:
   
   Database
   ```sh
   mysql -u root -p < sql/0_create_database.sql
   ```
   
   Tables
   ```sh
   mysql -u root -p okoa_rent < sql/tables/*.sql
   ```
   
   Stored procedures
   ```sh
   mysql -u root -p okoa_rent < sql/stored-procedures/*.sql
   ```

   For each step you will be prompted for the root user's password. If there's no password set on the root use, just hit enter again.
   
   NB: The Database tables and stored-procedures have not been uploaded!

1. Create a `.env` file in this this directory:

   ```ini
   SERVER_ADDR=127.0.0.1:8080
   MYSQL_USER=root
   MYSQL_PASSWORD=<password>
   MYSQL_HOST=127.0.0.1
   MYSQL_PORT=3306
   MYSQL_DBNAME=okoa_rent
   ```

   Update "MYSQL_USER" and "MYSQL_PASSWORD" values with the correct MySQL user/password.

1. Run the server:

   ```sh
   cargo run
   ```

1. Using a different terminal send requests to the running server. For example, using [HTTPie]:

   ```sh
   http POST :8080/processussdactions sessionId=ikskjsysgayauua&phoneNumber=254700000000&text=
   ```

   See [the API documentation pages](./apis/) for more info.

[HTTPie]: https://httpie.io/cli