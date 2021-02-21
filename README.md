## storagenv - v0.1.2

## Description
`storagenv` is a simple .env file manager, which you can retrieve env's to clipboard \
or simply print them in order to make life for local development, easier.

## How to install ? ( using Rust )

```sh
    git clone https://github.com/edicury/storagenv
    cd storagenv
    cargo build --release
    mkdir $HOME/storagenv
    mkdir $HOME/storagenv/bin
    cp /target/release/storagenv $HOME/storagenv/bin
    export PATH="$HOME/storagenv/bin:$PATH"
    source ~/.zshrc
```


## Available Commands

- `help`
    - lists all available commands inside binary
- `list`
    - lists all stored environments
- `show ENV_NAME`
    - prints specified env on console
- `add ENV_NAME ENV_STR`
    - stores new env inside binary
- `pick ENV_NAME`
    - copies env to clipboard