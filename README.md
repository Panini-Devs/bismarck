# Bismarck
A discord bot with a bunch of features.

## Featurelist
- Calculator
- Genshin Impact wish simulator
- And other stuff...

## Building
1. Create a sqlite database using the `migrations/0_initial.sql` file called `database.sqlite`
2. Run
```sh
DATABASE_URL=sqlite://database.sqlite cargo build
```

## Running
1. Get yourself a discord bot token
  1. Go to [https://discord.com/developers/applications](https://discord.com/developers/applications)
  2. Choose or create a new application
  3. Go to the bot tab and reset your token, make sure to save it (and not share it with anyone)
2. Create a copy of `sample.env` called `.env` and replace the value of `DISCORD_TOKEN` with your token
3. Run
```sh
cargo run
```

## License
See `LICENSE`

## Contributing
See `CONTRIBUTING.md` and `CODE_OF_CONDUCT.md`
