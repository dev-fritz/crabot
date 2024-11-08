use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Contact,
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Contact => {
            bot.send_message(
                msg.chat.id,
                format!("Github: {}\n\n Whatsapp: {}\n\n Email: {}",
                        "https://www.github.com/dev-fritz",
                        "https://wa.me/5595991561987",
                        "fritzhenrique.dev@gmail.com")
            ).await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(msg.chat.id, format!("Your username is @{username} and age is {age}."))
                .await?
        }
    };

    Ok(())
}