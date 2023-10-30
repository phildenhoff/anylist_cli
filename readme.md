# anylist_cli

A CLI utilty for interacting with AnyList. Uses [anylist_rs](https://github.com/phildenhoff/anylist_rs).

## running

To get an signed user id:

```fish
cargo run -- login
```

To view all your list names:

```fish
cargo run -- lists "<signed_user_id>"
```
