use rcon::Connection;
use serde::Deserialize;
use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt::Debug;
use tokio::sync::Mutex;
use std::str::FromStr;

struct Handler {
    prefix: String,
    connection: Mutex<Connection>,
    role: Option<RoleId>,
    channel: Option<ChannelId>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if message.author.bot {
            return
        }
        let permission = true;
        let permission = permission && if let Some(channel) = self.channel {
            message.channel_id == channel
        } else {
            true
        };
        let permission = permission && if let Some(guild) = message.guild_id {
            if let Some(expected_role) = self.role {
                message.author.has_role(&ctx.http, guild, expected_role).await.unwrap_or(false)
            } else {
                true
            }
        } else {
            false
        };
        if !permission { 
            return
        };
        if let Some(command) = message.content.strip_prefix(&self.prefix) {
            let command = command.trim();
            println!("run command: {}", command);

            match {
                (*self.connection.lock().await).cmd(command).await 
            } {
                Ok(res) => {
                    println!("response: {}", res);
                    print_err(
                        message
                            .channel_id
                            .say(&ctx.http, format!("response from server: \n{}", res))
                            .await,
                    );
                }
                Err(err) => {
                    println!("error    : {}", err);
                    print_err(
                        message
                            .channel_id
                            .say(
                                &ctx.http,
                                format!("error during sending message: \n{}", err),
                            )
                            .await,
                    );
                }
            }
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn print_err<T, R: Debug>(result: Result<T, R>) -> () {
    if let Some(err) = result.err() {
        println!("error: {:?}", err)
    }
}

#[tokio::main]
async fn main() {
    // Discord Bot Token を設定
    let options = read_options().await;

    let mut builder = Connection::builder();
    match options.server_kind {
        ServerKind::Normal => {}
        ServerKind::Minecraft => builder = builder.enable_minecraft_quirks(true),
        ServerKind::Factorio => builder = builder.enable_factorio_quirks(true),
    }
    let connection = builder
        .connect(options.rcon.address, &options.rcon.pass)
        .await
        .expect("failed to connect");

    let handler = Handler {
        prefix: options.prefix,
        role: options.role.map(RoleId),
        channel: options.channel.map(ChannelId),
        connection: Mutex::new(connection),
    };

    let mut client = Client::builder(&options.token)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    } else {
        println!("Client started");
    }
}

async fn read_options() -> Options {
    if let Some(options) = read_options_from_env() {
        return options
    }
    let config = tokio::fs::read_to_string("config.toml")
        .await
        .expect("failed to read config.toml");
    toml::from_str(&config).expect("failed to read config.toml")
}

fn read_options_from_env() -> Option<Options> {
    use std::env;

    Some(Options {
        token: env::var("DISCORD_TOKEN").ok()?,
        prefix: env::var("DISCORD_PREFIX").ok()?,
        role: env::var("DISCORD_ROLE").ok()
            .map(|x| x.parse::<u64>().expect("parsing DISCORD_ROLE")),
        channel: env::var("DISCORD_CHANNEL").ok()
            .map(|x| x.parse::<u64>().expect("parsing DISCORD_CHANNEL")),
        server_kind: env::var("SERVER_KIND").ok()
            .map(|x| x.parse::<ServerKind>().expect("parsing SERVER_KIND"))
            .unwrap_or_else(Default::default),
        rcon: RconOptions {
            address: env::var("RCON_ADDRESS").ok()?,
            pass: env::var("DISCORD_CHANNEL").ok().unwrap_or_else(Default::default),
        }
    })
}

#[derive(Deserialize)]
struct Options {
    token: String,
    prefix: String,
    role: Option<u64>,
    channel: Option<u64>,
    #[serde(default)]
    server_kind: ServerKind,
    rcon: RconOptions,
}

#[derive(Deserialize)]
struct RconOptions {
    address: String,
    #[serde(default)]
    pass: String,
}

#[derive(Deserialize)]
enum ServerKind {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "minecraft")]
    Minecraft,
    #[serde(rename = "factorio")]
    Factorio,
}

impl Default for ServerKind {
    fn default() -> Self {
        ServerKind::Normal
    }
}

#[derive(Debug)]
struct ParseServerKindErr(());

impl std::fmt::Display for ParseServerKindErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value is invalid. oneof `normal`, `minecraft`, or `factorio` is required.")
    }
}

impl std::error::Error for ParseServerKindErr {}

impl FromStr for ServerKind {
    type Err = ParseServerKindErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(ServerKind::Normal),
            "minecraft" => Ok(ServerKind::Minecraft),
            "factorio" => Ok(ServerKind::Factorio),
            _ => Err(ParseServerKindErr(())),
        }
    }
}
