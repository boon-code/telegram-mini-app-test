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

- Sending `/start` configures the chat menu button to open the Mini App.
- The Mini App counter changes locally when +1 or -1 is pressed.
