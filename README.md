# scost (simple cos table)

A simple command line tool to manage objects with same names across multiple buckets (for Tencent Cloud COS only).

## Usage

Run

```shell
scost # (default: -c $HOME/.scost.toml)
```

or

```shell
scost -c config.toml
```

into an interactive shell.

## Commands

- `cp <from_bucket> [<to_bucket>|*] <path>`. Copy an existing object from one bucket to another bucket (or all buckets).
- `ls [<bucket>|*] <path>`. List objects under a path in a bucket (or all buckets).
- `rm [<bucket>|*] <path>`. Remove an object from a bucket (or all buckets).
- `sign [<bucket>|*] <path>`. Sign a url for an object in a bucket (or all buckets).

P.S. '*' represents all buckets.

## Configuration

Just put your authentication information (secret id and secret key) and used buckets in an toml file.

An example is in [config/config.toml](config/config.toml):

```toml
[auth]
secret_id = "secret id"
secret_key = "secret key"

[[buckets]]
alias = "b1"
bucket = "bucket1"
region = "ap-shanghai"

[[buckets]]
alias = "b2"
bucket = "bucket2"
region = "ap-beijing"
```

## TODO

- `cp` supports copying a directory recursively.