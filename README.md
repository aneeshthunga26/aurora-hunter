# Aurora Hunter - Discord bot to track Aurora Borealis and Aurora Australis
Aurora data provided by [Auroras Live](http://api.auroras.live)

## Installing and running the bot
The bot is built using the shuttle runtime, which lets you run the bot locally or host it on [Shuttle](https://shuttle.dev).

### Prequisites
You need to have the following installed before you can install and run the bot
- rust toolchain ([I recommend using rustup to install this](https://rust-lang.org/learn/get-started/))
- shuttle cli ([I recommend using cargo to install this](https://docs.shuttle.dev/getting-started/installation#cargo-binstall))
- discord developer account ([here is a good starting point](https://discord.com/developers/))

After you have followed the previous steps, clone this repository and create `Secrets.toml` in the root directory of the project with the following contents.

```toml
DISCORD_TOKEN="discord_api_token"
DISCORD_GUILD_ID="server_id_you_want_to_install_the_bot_to"
```

also create a `.env` file in the root directory of the project with the following contents.

```
BASE_API_URL="http://api.auroras.live/v1/"
```

install and run the project with the command `shuttle run` while in the project's root directory.

