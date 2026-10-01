Telegram Mini-App Example
=========================

This shall show a small example how to interact: HTML UI and JavaScript <-> Telegram Bot (`teloxide`)

- Static Website via Github Pages: https://boon-code.github.io/telegram-mini-app-test/index.html
- No HTTP server, only `teloxide`
- Minimal dependencies
- Launch the Mini App from the chat menu button (left of the message field)
- Adjust an on-screen counter with +1 and -1 buttons

# Resources

- https://core.telegram.org/bots/webapps
- https://github.com/teloxide/teloxide
- https://docs.rs/teloxide/latest/teloxide

# Behavior

- In a private chat, `/start` configures the chat menu button to open the Mini App.
- In a group, `/start` sends a URL button that opens the bot's Main Mini App. Configure the Main Mini App URL in @BotFather (Bot Settings > Main App) for this link to work.
- The Mini App counter changes locally when +1 or -1 is pressed.
- Enable inline mode for the bot with `/setinline` in @BotFather.
- Pressing **Send value to bot** switches to an inline query containing the counter. Choose the result to send it, then use `/show` in the bot chat to see the value saved by the bot.
- The bot stores the latest submitted value in memory using an atomic counter; it resets when the bot process restarts.
