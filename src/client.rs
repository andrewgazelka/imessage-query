use std::{fmt::Display, io::Write};

use colored::Colorize;
use derive_more::Constructor;
use once_cell::sync::Lazy;
use regex::Regex;
use sqlx::{sqlite::SqliteRow, Row};

use crate::{apple, apple::copy_to_clipboard};

#[derive(Constructor)]
pub struct Client {
    pool: sqlx::SqlitePool,
    to: String,
    me: String,
}

fn handle_query_thread(
    row: &SqliteRow,
    to: &str,
    me: &str,
    writer: &mut impl Write,
) -> anyhow::Result<()> {
    let text: Vec<u8> = row.get(0);

    if text.is_empty() {
        return Ok(());
    }

    let text = apple::blob_to_text(&text).unwrap();

    // if the entire message is whitespace, don't print it
    static ALL_WHITESPACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s+$").unwrap());

    if ALL_WHITESPACE.is_match(&text) {
        return Ok(());
    }

    let from_me: bool = row.get(1);

    if from_me {
        let message = format!("{me}: {text}");
        writeln!(writer, "{message}")?;
    } else {
        let message = format!("{to}: {text}");
        writeln!(writer, "{message}")?;
    }

    Ok(())
}

fn handle_query_all(row: &SqliteRow) {
    let handle_id = row.get(0);
    let text: Vec<u8> = row.get(1);

    if text.is_empty() {
        return;
    }

    let text = apple::blob_to_text(&text).unwrap_or_default();

    let handle_id = Colored(handle_id);

    println!("{handle_id} {text}");
}

impl Client {
    pub async fn print_messages_with_handle_id(&self, handle_id: i64) -> anyhow::Result<()> {
        let query =
            sqlx::query("SELECT attributedBody, is_from_me FROM message WHERE handle_id = ?")
                .bind(handle_id)
                .fetch_all(&self.pool)
                .await?;

        let mut output: Vec<u8> = Vec::new();

        for row in query {
            handle_query_thread(&row, &self.to, &self.me, &mut output)?;
        }

        let output = String::from_utf8(output)?;

        print!("{output}");
        copy_to_clipboard(&output)?;

        Ok(())
    }

    pub async fn print_all_messages(&self) -> anyhow::Result<()> {
        let query = sqlx::query("SELECT handle_id, attributedBody FROM message")
            .fetch_all(&self.pool)
            .await?;

        for row in query {
            handle_query_all(&row);
        }

        Ok(())
    }
}

struct Colored(i64);

impl Display for Colored {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = self.0 % 10;

        let s = format!("{:04}", self.0);

        // colorized crate
        match color {
            0 => write!(f, "{}", s.red()),
            1 => write!(f, "{}", s.green()),
            2 => write!(f, "{}", s.yellow()),
            3 => write!(f, "{}", s.blue()),
            4 => write!(f, "{}", s.magenta()),
            5 => write!(f, "{}", s.cyan()),
            6 => write!(f, "{}", s.white()),
            7 => write!(f, "{}", s.bright_red()),
            8 => write!(f, "{}", s.bright_green()),
            9 => write!(f, "{}", s.bright_yellow()),
            _ => write!(f, "{s}"),
        }
    }
}
