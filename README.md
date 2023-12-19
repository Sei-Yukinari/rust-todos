# rust-todos
This Repository is using actix-web and async-graphql as a trial

## Requirements
- Rust 1.74
- Docker 24

## Usage
### start
```shell
$ make up
```

### logs
```shell
$ make logs web
```

### enter container
```shell
$ make exec web
```

### generate graphql schema
```shell
$ make gen
```

### test
```shell
$ make test
```

### DB
```shell
$ make exec web
## enter container
$ sqlx migtate add <name>
$ sqlx migtate run
```