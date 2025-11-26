cat = cat
    .description = 😺 Show a cat image
    .help = Sends a random cat picture.

cat-not-found = 😿 No cat found.

rule34 = rule34
    .description = 🔞 Show a random r34 image by tags
    .tags = tags
    .tags-description = Tags separated by space
    .help = Find a random image from Rule34 based on the tags you provide.
    Tags are separated by spaces.

safebooru = safebooru
    .description = 🖼️ Show a random anime image by tags
    .tags = tags
    .tags-description = Tags separated by space
    .help = Find a random image on SafeBooru based on the tags you provide.
    Tags are separated by spaces.

coin = coin
    .description = 🪙 Flip a coin
    .side = side
    .side-description = Choose a side
    .help = Flip a coin.

coin-msg = You chose `{$side}`... It came up `{$correct}`!
coin-right = 🏅 You guessed right!
coin-wrong = ⛔ You guessed wrong!
Heads = Heads
Tails = Tails

ping = ping
    .description = 😴 Check my response time
    .help = Check my response speed.
    The response is a message saying "Pong!".

ping-pong = Pong!

help = help
    .description = 🆘 Show a help menu
    .command = command
    .command-description = A command name
    .help = Get help on available commands.
    Can show general help with a list of available commands,
    as well as detailed information on commands.

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
    .description = 💬 Talk to me
    .text = text
    .text-description = Text to send
    .help = Talk to me.
    You can also reply to my messages to trigger this command,
    or write 'Sylvie' in any message.

ask-error = 🤖 No response received. The response may have been filtered, or a network error occurred.

forget = forget
    .description = 🗑️ Remove entries from memory
    .indexes = indexes
    .indices-description = Entry indexes from /memory
    .help = Delete one or more messages from my memory.
    Accepts one or more message indexes, separated by space.
    Indexes can be obtained from the `s.memory` command.
    Requires message management permission.

forget-forgot = 🚮 Entries with indexes {$indices} have been removed from memory.

forgetall = forgetall
    .description = 🗑️ Forget guild memory
    .confirmation = confirmation
    .confirmation-description = Type "yes" to confirm.
    .help = Completely clears guild's persistent memory.
    I will forget all messages in the guild.
    Requires administrator permissions.

forgetall-confirm = ⛔ Action not confirmed.
forgetall-forgot = 🚮 All memory has been erased.

restart = restart
    .description = 🔄 Restart the chatbot
    .help = Restarts the AI chatbot, clearing short-term memory and starting a new conversation.
    Permanent memory is not cleared.
    Requires message management permission.

restart-restarted = 🔄 Restarted successfully.
restart-not = ⛔ No restart needed.

memory = memory
    .description = 💾 View persistent memory
    .index = index
    .index-description = Index of a message
    .help = View a list of messages in my persistent memory.
    You can specify a message index to view single message.

memory-title = Messages in persistent memory:
memory-single-title = Message with index {$index}
memory-single-not-found = ⛔ A message with this index doesn't exist.
memory-empty = Memory is empty.
memory-footer = Use `s.memory ID` to get specific message.
    Use `s.forget ID` to delete a memory entry.
    Page {$page} of {$max_page}

clear = clear
    .description = 🗑️ Delete last messages in a channel
    .amount = amount
    .amount-description = Amount of messages to delete
    .help = Delete up to 255 recent messages in a text channel.
    Requires message management permission.

clear-last-msg-fail = ⛔ Couldn't fetch recent messages.
clear-deleted = 🚮 Deleted {$amount} messages.

hug = hug
    .description = 🤗 Give someone a hug
    .target = target
    .target-description = A member to hug
    .help = Hug someone.
    You can provide a mention as an argument,
    or reply to person's message with the command, in which case you don't need to provide an argument.

hug-target = {$author} gives {$target} a warm hug
hug-no-target = Sylvie gives {$author} a warm hug

kiss = kiss
    .description = 😘 Kiss someone
    .target = target
    .target-description = A member to kiss
    .help = Kiss someone.
    You can provide a mention as an argument,
    or reply to person's message with the command, in which case you don't need to provide an argument.

kiss-target = {$author} kissed {$target}
kiss-no = Sorry, Sylvie is not interested in that kind of things.

bite = bite
    .description = 🧛 Bite someone
    .target = target
    .target-description = A member to bite
    .help = Bite someone.
    You can provide a mention as an argument,
    or reply to person's message with the command, in which case you don't need to provide an argument.

bite-target = {$author} bites {$target}
bite-no-target = Sylvie bites {$author}

lick = lick
    .description = 👅 Lick someone
    .target = target
    .target-description = A member to lick
    .help = Lick someone.
    You can provide a mention as an argument,
    or reply to person's message with the command, in which case you don't need to provide an argument.

lick-target = {$author} licks {$target}
lick-no-target = Sylvie licks {$author}

pat = pat
    .description = 🫳 Pat someone on the head
    .target = target
    .target-description = A member to pat
    .help = Pat someone.
    You can provide a mention as an argument,
    or reply to person's message with the command, in which case you don't need to provide an argument.

pat-target = {$author} gently pats {$target}
pat-no-target = Sylvie pats {$author}

sleep = sleep
    .description = 😴 Fall asleep
    .target = target
    .target-description = A member to sleep with
    .help = Allows you to sleep alone or with someone.
    You can provide a mention as an argument,
    or reply to person's message with the command, in which case you don't need to provide an argument.

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
