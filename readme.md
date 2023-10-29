# anylist_cli

A CLI utilty for interacting with AnyList.

## running

To get an signed user id:

```fish
cargo run -- login --email <email> --password <password>
```

To view all your list names:

```fish
cargo run -- lists "<signed_user_id>"
```
