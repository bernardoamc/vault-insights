# vault-insights

A way to aggregate project information from your Vault. 

As of now we can infer which projects have been updated in the past X days and get an URL of the latest update.
Projects that have been especified but haven't been updated are listed as outdated.

## Setup

Add a file to `~/.config/vault-insights` with the content:

```toml
key = "your_key_from_vault"
token = "your_token_from_vault"
vault_url = "vault_url"
```

Run `vault-insights` specifying your project ids and the time period.

```sh
cargo run --  -p 9140,7927,1326 -s 5
```

## Interface

```

USAGE:
    vault-insights [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --projects <projects>...             Project IDs from the vault
    -s, --since_days_ago <since_days_ago>    Fetch projects updated since this amount of days [default: 14]
```
