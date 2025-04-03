# axum
A simple web server written in Rust using the axum framework.

# Components
- axum
- sea-orm
- redis 

# generate entity
```shell
sea generate entity -u mysql://root:123456@127.0.0.1:3306/warlock -o src/models/entity --with-serde=both --date-time-crate=time --expanded-format
```