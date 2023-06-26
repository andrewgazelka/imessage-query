use anyhow::Context;
use clap::Parser;
use sqlx::Row;

fn split_by(input: &[u8], separator: &[u8]) -> Vec<Vec<u8>> {
    let mut result = Vec::new();

    let mut start = 0;
    while let Some(end) = input[start..]
        .windows(separator.len())
        .position(|window| window == separator)
    {
        result.push(input[start..start + end].to_vec());
        start += end + separator.len();
    }

    result.push(input[start..].to_vec());

    result
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The handle id of the person you're chatting with
    handle_id: Option<i64>,

    /// The name of the person you're chatting with
    #[arg(short, long, default_value = "Bob")]
    to: String,

    /// The name of yourself
    #[arg(short, long, default_value = "Andrew")]
    me: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let home_dir = dirs::home_dir().context("failed to get home dir")?;
    let chat_db_path = home_dir.join("Library/Messages/chat.db");

    let url = format!("file:{}", chat_db_path.to_str().unwrap());

    let pool = sqlx::sqlite::SqlitePool::connect(&url).await?;

    if let Some(handle_id) = args.handle_id {
        println!("handle id");
        let res = sqlx::query("SELECT attributedBody, is_from_me FROM message WHERE handle_id = ?")
            .bind(handle_id)
            .fetch_all(&pool)
            .await?;

        for row in res {
            let blob_data: Vec<u8> = row.get(0);
            let text = blob_to_text(&blob_data).unwrap_or_default();

            let is_from_me: bool = row.get(1);

            if text.is_empty() {
                continue;
            }

            let text = text.replace('\n', " ");

            if is_from_me {
                println!("{}: {text}", args.me);
            } else {
                println!("{}: {text}", args.to);
            }
        }
    } else {
        println!("starting...");
        let res = sqlx::query("SELECT handle_id, attributedBody FROM message")
            .fetch_all(&pool)
            .await?;

        for row in res {
            let handle_id: i64 = row.get(0);
            let blob_data: Vec<u8> = row.get(1);
            let text = blob_to_text(&blob_data).unwrap_or_default();

            if text.is_empty() {
                continue;
            }
            println!("{handle_id}\t{text}");
        }
    };

    //
    Ok(())
}

/// <https://github.com/niftycode/imessage_reader/blob/c021d1a4169fe5357d6f95e6dd4d615b378e88bf/imessage_reader/fetch_data.py#L42>
fn blob_to_text(blob_data: &[u8]) -> Option<String> {
    let blob_data = split_by(blob_data, b"NSString");

    let Some(blob_data) = blob_data.get(1) else {
        return None;
    };

    let blob_data = blob_data.get(5..).unwrap();
    let text = if blob_data[0] == 129 {
        let len = blob_data.get(1..3).unwrap();
        let len = u16::from_le_bytes([len[0], len[1]]) as usize;
        let blob_data = blob_data.get(3..).unwrap();
        let blob_data = blob_data.get(..len).unwrap();
        String::from_utf8(blob_data.to_vec()).unwrap()
    } else {
        let len = blob_data[0] as usize;
        let blob_data = blob_data.get(1..).unwrap();
        let blob_data = blob_data.get(..len).unwrap();

        // text = text[3:length + 3]
        String::from_utf8(blob_data.to_vec()).unwrap()
    };

    Some(text)
}
