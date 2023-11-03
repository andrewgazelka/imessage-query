use anyhow::Context;
use clap::Parser;
use client::Client;

mod apple;
mod client;
mod split;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The handle id of the person you're chatting with
    handle_id: Option<i64>,

    /// The name of the person you're chatting with
    #[arg(short, long, default_value = "Friend")]
    to: String,

    /// The name of yourself
    #[arg(short, long, default_value = "Me")]
    me: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let home_dir = dirs::home_dir().context("failed to get home dir")?;
    let chat_db_path = home_dir.join("Library").join("Messages").join("chat.db");

    let url = format!("file:{}", chat_db_path.to_str().unwrap());

    let pool = sqlx::sqlite::SqlitePool::connect(&url).await?;

    let client = Client::new(pool, args.to, args.me);

    if let Some(handle_id) = args.handle_id {
        client.print_messages_with_handle_id(handle_id).await?;
    } else {
        client.print_all_messages().await?;
    };

    Ok(())
}
