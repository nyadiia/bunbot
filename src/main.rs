use std::env;

use serenity::Client;
use slashy::{
    framework::{
        Framework,
        CommandContext,
    },
    commands::CommandResult,
    settings::Settings,
    command,
    subcommand
};

command!{
    ping,
    "ping pong",
    pong,
    [
        optional String text | "text to echo"
    ]
}

#[subcommand]
async fn pong(ctx: &CommandContext) -> CommandResult {
    ctx.send_str(&format!("pong {}", ctx.get("text").unwrap_or("")))
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("token");
    let app_id = std::env::var("APPLICATION_ID").expect("app_id").parse().expect("app_id parse");

    // Create the slashy framework
    let settings = Settings {
        prefixes: vec!["!"],
        auto_register: true,
        auto_delete: true,
        slash_command_guilds: vec![]
    };
    let framework = Framework::new(settings, app_id, token)
        .command(PING_COMMAND);

    // Login with a bot token from the environment
    let mut client = Client::builder(token)
        .event_handler(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}