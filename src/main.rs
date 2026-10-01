use teloxide::{
    prelude::*,
    types::{MenuButton, WebAppInfo},
};

const MINI_APP_URL: &str = "https://boon-code.github.io/telegram-mini-app-test/index.html";

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    println!("Bot started...");

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if msg.text() == Some("/start") {
            bot.set_chat_menu_button()
                .chat_id(msg.chat.id)
                .menu_button(MenuButton::WebApp {
                    text: "Open Mini App".to_string(),
                    web_app: WebAppInfo {
                        url: MINI_APP_URL.parse().unwrap(),
                    },
                })
                .await?;

            bot.send_message(
                msg.chat.id,
                "Open the Mini App from the menu button next to the message field.",
            )
            .await?;

            return Ok(());
        }

        if let Some(data) = msg.web_app_data() {
            bot.send_message(msg.chat.id, format!("Mini App sent: {}", data.data))
                .await?;

            return Ok(());
        }

        Ok(())
    })
    .await;
}
