use crate::translation::tr;
use crate::utils::*;
use poise::ChoiceParameter;
use rand::Rng;

#[derive(ChoiceParameter, Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Heads,
    Tails,
}

impl Side {
    fn tr(&self, ctx: Context<'_>) -> String {
        match self {
            Side::Heads => tr!(ctx, "Heads").to_string(),
            Side::Tails => tr!(ctx, "Tails").to_string(),
        }
    }
}

#[poise::command(prefix_command, slash_command, category = "Fun")]
pub async fn coin(ctx: Context<'_>, side: Side) -> Result<(), Error> {
    let correct = {
        let mut rng = rand::rng();
        if rng.random_bool(0.5) {
            Side::Heads
        } else {
            Side::Tails
        }
    };

    let mut response = tr!(ctx, "coin-msg",
        side: side.tr(ctx),
        correct: correct.tr(ctx)
    );

    response = format!(
        "{response}\n{}",
        if side == correct {
            tr!(ctx, "coin-right")
        } else {
            tr!(ctx, "coin-wrong")
        }
    );

    reply_without_ping(ctx, response).await?;
    Ok(())
}
