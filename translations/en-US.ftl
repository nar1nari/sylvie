cat = cat
    .description = 😺 Show a cat image

cat-not-found = 😿 No cat found.

rule34 = rule34
    .description = 🔞 Show a random r34 image by tags
    .tags = tags
    .tags-description = Tags separated by space

safebooru = safebooru
    .description = 🖼️ Show a random anime image by tags
    .tags = tags
    .tags-description = Tags separated by space

coin = coin
    .description = 🪙 Flip a coin
    .side = side
    .side-description = Choose a side

coin-msg = You chose `{$side}`... It came up `{$correct}`!
coin-right = 🏅 You guessed right!
coin-wrong = ⛔ You guessed wrong!
Heads = Heads
Tails = Tails

ping = ping
    .description = 😴 Check bot response time
ping-pong = Pong!

help = help
    .description = 🆘 Show a help menu
    .command = command
    .command-description = A command name

help-title = ℹ️ Help
help-usage = ℹ️ Usage:
help-optional =  (optional)
help-parameters = *️⃣ Parameters
help-not-found = ⛔ Command not found
help-no-description = No description.
help-aliases = 🔣 Aliases
help-not-found-description = The `{$command}` command does not exist.
help-footer = Commands can be invoked with `/` or `s.`
    Use `/help [command]` for detailed information.

ask = ask
    .description = 💬 Ask the chatbot
    .text = text
    .text-description = Text to send
ask-error = 🤖 No response received. The response may have been filtered, or a network error occurred.

forget = forget
    .description = 🗑️ Remove entries from memory
    .indexes = indexes
    .indices-description = Entry indexes from /memory

forget-forgot = 🚮 Entries with indexes {$indices} have been removed from memory.

forgetall = forgetall
    .description = 🗑️ Forget chat history
    .confirmation = confirmation
    .confirmation-description = Type "yes" to confirm.

forgetall-confirm = ⛔ Action not confirmed.
forgetall-forgot = 🚮 All chat history has been erased.

restart = restart
    .description = 🔄 Restart the chatbot

restart-restarted = 🔄 Restarted successfully.
restart-not = ⛔ No restart needed.

memory = memory
    .description = 💾 View persistent memory

memory-title = Messages in persistent memory:
memory-empty = Memory is empty.
memory-footer = Use `s.forget ID` to delete a memory entry.
    Page {$page} of {$max_page}

clear = clear
    .description = 🗑️ Delete last messages in a channel
    .amount = amount
    .amount-description = Amount of messages to delete

clear-last-msg-fail = ⛔ Couldn't fetch recent messages.
clear-deleted = 🚮 Deleted {$amount} messages.

hug = hug
    .description = 🤗 Give someone a hug
    .target = target
    .target-description = A member to hug
hug-target = {$author} gives {$target} a warm hug
hug-no-target = Sylvie gives {$author} a warm hug

kiss = kiss
    .description = 😘 Kiss someone
    .target = target
    .target-description = A member to kiss
kiss-target = {$author} kissed {$target}
kiss-no = Sorry, Sylvie is not interested in that kind of things.

bite = bite
    .description = 🧛 Bite someone
    .target = target
    .target-description = A member to bite
bite-target = {$author} bites {$target}
bite-no-target = Sylvie bites {$author}

lick = lick
    .description = 👅 Lick someone
    .target = target
    .target-description = A member to lick
lick-target = {$author} licks {$target}
lick-no-target = Sylvie licks {$author}

pat = pat
    .description = 🫳 Pat someone on the head
    .target = target
    .target-description = A member to pat
pat-target = {$author} gently pats {$target}
pat-no-target = Sylvie pats {$author}

sleep = sleep
    .description = 😴 Fall asleep
    .target = target
    .target-description = A member to sleep with
sleep-target = {$author} sleeps with {$target}
sleep-no-target = {$author} falls asleep

rp-need-target = 👤 You must specify a target for this action.
booru-not-found = 🗨️ No images found for the given tags.
went-wrong = Something went wrong, try again.
error = Error: {$error}
argument-parse-error = **Invalid command arguments.**
    Check the command usage in the help menu.
nsfw-error = **This command can only be used in NSFW channels.**
user-permissions-error = **Missing permissions:** `{$permissions}`
bot-permissions-error = "**Missing bot permissions:** `{$permissions}`
guild-only-error = **This command can only be used in guild channels.**
error-no-reference = **Reference message is unknown.**
