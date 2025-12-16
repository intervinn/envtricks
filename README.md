</br>
<h1 align="center">envtricks</h1>
</br>

[![Crates.io](https://img.shields.io/crates/v/envtricks)](https://crates.io/crates/envtricks)

envtricks (`envtricks`) is a small set of helpful commands to work with .env files.

This project features powerful manipulation with selectively copying fields from one file to another,
generating .env.example's and primitive commands for getting and setting values of fields, which is useful
for shell scripts.

## Installing
From source:
```bash
git clone https://github.com/intervinn/envtricks && cd envtricks && cargo install --path .
```

## Commands
Run `envtricks help` to view similar information.

### `envtricks move`
* Arguments: `source`, `destination`, `fields`
* Flags: `-a (--all)`

Copies `fields` of `source` into `destination`, keeping all of `destination` fields that weren't present in `source`. 

If `-a` is present and `fields` is empty, all fields all fields of `source` are copied into `destination`

If `-a` is present and `fields` if present, all fields except for the ones present in `fields` are copied into `destination`

If the `destination` file doesn't exist, it gets automatically created.

Examples:

`envtricks move -a .env .env.prod` - move all fields of .env into .env.prod, create .env.prod if doesn't exist.

`envtricks move -a .env .env.prod SECRET SOMETHING` - move all fields of .env into .env.prod, EXCEPT for `SECRET` and `SOMETHING`

`envtricks move .env .env.prod SECRET SOMETHING` - move ONLY `SECRET` and `SOMETHING` from .env into .env.prod

### `envtricks example`
* Arguments: `source`, `destination`

Create or overwrite `destination`, containing `source` with empty values.

### `envtricks set`
* Arguments: `source`, `key`, `value`

Set `key` to `value` in `source`

### `envtricks get`
* Arguments: `source`, `key`

Writes value of `key` to stdout.

### `envtricks remove`
* Arguments: `source`, `fields`

Removes values of `fields` in `source`
