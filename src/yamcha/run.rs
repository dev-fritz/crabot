use teloxide::prelude::*;
use crate::yamcha::commands::{Command, answer};

pub async fn start_bot() {
    let bot = teloxide::Bot::from_env();
    Command::repl(bot, answer).await;
}