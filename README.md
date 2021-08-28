discord-rcon
======

The discord bot to control game servers via rcon.
This bot supports Minecraft and Factorio by [rcon crate] 
but other servers may work.
This bot is tested with minecraft 1.7.10.

[rcon crate]: https://crates.io/crates/rcon

## how to enable rcon for minecraft server

<details>
<summary>how to enable rcon for minecraft server</summary>

1. replace `enable-rcon=false` to `enable-rcon=true` in `server.properties`
2. add `rcon.password=PASSWORD` (please replace `PASSWORD` with password 
   you want to use) to `server.properties`

</details>

## how to use this bot

### via docker

First, please [create config.toml]

Then run following command to start the bot.
```bash
docker run ghcr.io/anatawa12/discord-rcon
```

### install via cargo

Run ``cargo install discord-rcon`` to install discord-rcon.

Then, please [create config.toml].

Finally, run ``discord-rcon`` to start bot.

## configuring this bot

This bot uses [TOML] to write configurations.
Please replace each values to the value you want to use.
Commented property means optional configuration.

```toml
# the discord bot token
token = "<discord bot token>"

# the discord command prefix.
# if the value is "!", "!say hello" executes "say hello" in console
# If the value is empty, all messages posted to the channel by a person who has
# the role will be executed.
prefix = "<command prefix>"

# The id of role the users this bot can be used.
# if not specified, all users can use this bot. it's dangerous
#role = <role id>

# The id of channel this bot will listen.
# if not specified, all channels on guild/server the bot connected will be watched.
#channel = <channel id>

# The kind of your game server.
# This enables the some quirks for game servers.
# currently supports Minecraft and Factorio.
# If your game is not one of games above, you don't need to specify server_kind
#server_kind = "<one of minecraft, factorio, or normal>"

# The section about rcon connection
[rcon]
# the address of rcon server.
address = "<server ip>:<port>"
# the password of rcon server.
pass = "<password>"
```

[create config.toml]: #configuring-this-bot
