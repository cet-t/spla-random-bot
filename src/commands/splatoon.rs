use std::time::SystemTime;

use moelle_a::Jsf32;
use moelle_sample::Choice;
use poise::ChoiceParameter;

use crate::{
    Context, is_bot, reply,
    splatoon::{
        self,
        weapon::{MainFilter, SpecialFilter, SubFilter},
    },
};

#[poise::command(slash_command)]
pub async fn weapon(
    ctx: Context<'_>,
    main: Option<MainFilter>,
    sub: Option<SubFilter>,
    special: Option<SpecialFilter>,
) -> anyhow::Result<()> {
    is_bot!(ctx);

    ctx.defer().await?;

    let Ok(raw) = splatoon::weapon::get_raw().await else {
        ctx.send(reply!("error: `{}:{}`", file!(), line!())).await?;
        return Ok(());
    };
    let Ok(mut weapons) = splatoon::weapon::parse_raw(raw) else {
        ctx.send(reply!("error: `{}:{}`", file!(), line!())).await?;
        return Ok(());
    };

    if let Some(main) = main {
        weapons.retain(|w| {
            MainFilter::from_name(&w.wtype.name.ja_JP)
                .map(|f| f == main)
                .unwrap_or(false)
        });
    }

    if let Some(sub) = sub {
        weapons.retain(|w| {
            SubFilter::from_name(&w.sub.name.ja_JP)
                .map(|f| f == sub)
                .unwrap_or(false)
        });
    }

    if let Some(special) = special {
        weapons.retain(|w| {
            SpecialFilter::from_name(&w.special.name.ja_JP)
                .map(|f| f == special)
                .unwrap_or(false)
        })
    }

    if weapons.is_empty() {
        ctx.send(reply!("該当なし").ephemeral(true)).await?;
    } else {
        let mut rng = {
            let now = SystemTime::now().elapsed()?;
            Jsf32::from(now.as_nanos().rotate_left(12) as u32)
        };
        let Some(w) = rng.choice(&weapons) else {
            ctx.send(reply!("error: `{}:{}`", file!(), line!())).await?;
            return Ok(());
        };
        ctx.send(reply!("{}", w.name.ja_JP)).await?;
    }
    Ok(())
}
