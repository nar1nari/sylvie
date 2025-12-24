# Sylvie - Multifunctional Discord Bot in Rust

**Sylvie** is a public, completely free and open-source Discord bot written in Rust. 

---

## What Sylvie Can Do

### Commands

- Supports both prefix and slash commands
- Clean agnostic command handling powered by **Poise**

### Moderation

- Clear messages in chat
- Lightweight moderation utilities (expanding over time)

### Fun & Roleplay

- Simple fun commands (coin flip, cat, etc.)
- Roleplay interaction commands

### Utilities

- Download and send media from YouTube, TikTok, etc, via **yt-dlp**

### Music playback

- Coming soon

---

## AI Chatbot (Core Feature)

This is the whole point of Sylvie.

- Powered by Gemini-2.5-Flash
- Remembers conversation context across messages
- Fine control over what is stored in memory
- Designed for long-term, natural interaction in servers

---

## Languages

Currently supported:

- 🇬🇧 English
- 🇷🇺 Russian

Translation system is simple and contributor-friendly
New languages are always welcome.

---

## ⚠️ Major rewrite still in progress ⚠️

Original version was written in Python and current version is a full rewrite in Rust. Some parts are messy, experimental, or incomplete. APIs and structure may and will change.

---

## Contributing

Contributions are welcome and appreciated.

### If you know Rust you can

- Improve architecture
- Refactor messy code
- Suggest better patterns where things are ugly
- Optimize or clean up existing logic 

### If you don't know Rust, you can still help

- Add new translations
	 See: [translations](./translations/)
- Improve existing language files
- Fix wording or UX issues

If you see something dumb, feel free to fix it.

---

## Credits

Built using excellent Rust libraries:

- [Serenity](https://github.com/serenity-rs/serenity)
- [Poise](https://github.com/serenity-rs/poise)
