use color_eyre::{eyre::WrapErr, Result};
use discord::model::ChannelId;
use discord::Discord;
use std::env;

fn main() -> Result<()> {
    color_eyre::install()?;
    let token = env::var("DISCORD_TOKEN").context("No Discord token specified")?;
    let channel_id = env::var("DISCORD_CHANNEL_ID")
        .context("No Discord channel id specified")?
        .parse::<u64>()
        .context("Provided Discord channel id is not of type u64")?;
    let message = env::var("DISCORD_MESSAGE").context("No Discord message specified")?;

    let discord = Discord::from_bot_token(&token)
        .context("Failed to login to Discord with the provided token")?;

    let m = discord.send_message(ChannelId(channel_id), &message, "", false)?;

    println!("Message sent: {m:?}");

    Ok(())
}
