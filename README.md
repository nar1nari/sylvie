> [!WARNING]  
> This repository contains an old version of Sylvie; the current version was rewritten and currently have closed source code.
> 
> This code is no longer maintained and may stop working with newer versions of the crates. However, you can still use it as an example of how to create a Discord bot in Rust. If you create a fork, please retain the original license.

# Sylvie - Multifunctional Discord Bot in Rust

**Sylvie** is a public, completely free and open-source Discord bot written in Rust. 

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

## AI Chatbot (Core Feature)

This is the whole point of Sylvie.

- Powered by Gemini-2.5-Flash
- Remembers conversation context across messages
- Fine control over what is stored in memory
- Designed for long-term, natural interaction in servers

## Languages

Currently supported:

- 🇬🇧 English
- 🇷🇺 Russian

Translation system is simple and contributor-friendly
New languages are always welcome.

## Credits

Built using excellent Rust libraries:

- [Serenity](https://github.com/serenity-rs/serenity)
- [Poise](https://github.com/serenity-rs/poise)
