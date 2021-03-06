# discord-links-transfer-bot

[![Test](https://github.com/tktcorporation/discord-links-transfer-bot/actions/workflows/test.yml/badge.svg)](https://github.com/tktcorporation/discord-links-transfer-bot/actions/workflows/test.yml)

## Prerequirements

- Docker, docker-compose
- Discord Bot Token

## Get Started

### Env Vars

1. `cp -p .envrc.sample .envrc` and set variables.
1. Install [direnv](https://github.com/direnv/direnv).
1. `direnv allow`

### Run

1. Invite your bot to your server.
1. `docker-compose run app`
1. Post `~join` in your server.
1. The bot talk in voice chat.

### Develop

1. `docker-compose run app /bin/bash`

#### Test

```bash
RUST_BACKTRACE=1 cargo test
```

#### Linter, Formatter

- Lint

```bash
cargo clippy --all
```

- Format

```bash
cargo fmt --all
```

##### Task Runner

[act](https://github.com/nektos/act) can use as a task runner in this project.  
But, it takes longer than `cargo` commands.

```bash
# lint, format(check), test
act
```

```bash
# deploy to heroku
act release
```

## Deploying to Heroku

### Prerequirements

- Heroku Account

### Deploy

```bash
docker-compose run heroku /bin/bash
```

```bash:in_container
heroku create <APP_NAME>
heroku login
apk add docker
heroku container:login
heroku container:push app -a <APP_NAME>
heroku container:release app -a <APP_NAME>
heroku ps:scale app=1 -a <APP_NAME>
```

## To invite sample bot
https://discord.com/api/oauth2/authorize?client_id=866822680485429259&permissions=2147822672&scope=bot

## LICENCE
MIT
