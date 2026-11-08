use moelle_a::Biski64;
use moelle_sample::Choice;
use poise::ChoiceParameter;

use crate::{
    Context, is_bot, reply,
    splatoon::{
        self,
        rule::Rule,
        weapon::{MainFilter, SpecialFilter, SubFilter},
    },
    toolkit::BoolChoice,
};

#[poise::command(
    slash_command,
    name_localized("ja", "武器抽選"),
    description_localized("ja", "武器抽選")
)]
pub async fn weapon(
    ctx: Context<'_>,
    #[name_localized("ja", "メインウェポン")] main: Option<MainFilter>,
    #[name_localized("ja", "サブウェポン")] sub: Option<SubFilter>,
    #[name_localized("ja", "スペシャルウェポン")] special: Option<SpecialFilter>,
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
            MainFilter::from_name(&w.wtype.name.ja_jp)
                .map(|f| f == main)
                .unwrap_or(false)
        });
    }

    if let Some(sub) = sub {
        weapons.retain(|w| {
            SubFilter::from_name(&w.sub.name.ja_jp)
                .map(|f| f == sub)
                .unwrap_or(false)
        });
    }

    if let Some(special) = special {
        weapons.retain(|w| {
            SpecialFilter::from_name(&w.special.name.ja_jp)
                .map(|f| f == special)
                .unwrap_or(false)
        })
    }

    if weapons.is_empty() {
        ctx.send(reply!("該当なし").ephemeral(true)).await?;
    } else {
        let mut rng = {
            let now = chrono::Utc::now();
            let id = ctx.author().id.get();
            Biski64::from(now.timestamp().wrapping_add(id as i64) as u64)
        };
        let Some(w) = rng.choice(&weapons) else {
            ctx.send(reply!("error: `{}:{}`", file!(), line!())).await?;
            return Ok(());
        };
        ctx.send(reply!("{}", w.name.ja_jp)).await?;
    }
    Ok(())
}

#[poise::command(
    slash_command,
    name_localized("ja", "ステージ抽選"),
    description_localized("ja", "ステージ抽選")
)]
pub async fn stage(ctx: Context<'_>) -> anyhow::Result<()> {
    is_bot!(ctx);

    ctx.defer().await?;

    let Ok(raw) = splatoon::stage::get_raw().await else {
        ctx.send(reply!("error: `{}:{}`", file!(), line!())).await?;
        return Ok(());
    };
    let Ok(stages) = splatoon::stage::parse_raw(raw) else {
        ctx.send(reply!("error: `{}:{}`", file!(), line!())).await?;
        return Ok(());
    };

    if stages.is_empty() {
        ctx.send(reply!("該当なし").ephemeral(true)).await?;
    } else {
        let mut rng = {
            let now = chrono::Utc::now();
            let id = ctx.author().id.get();
            Biski64::from(now.timestamp().wrapping_add(id as i64) as u64)
        };
        let Some(s) = rng.choice(&stages) else {
            ctx.send(reply!("error: `{}:{}`", file!(), line!())).await?;
            return Ok(());
        };
        ctx.send(reply!("{}", s.name.ja_jp)).await?;
    }
    Ok(())
}

#[poise::command(
    slash_command,
    name_localized("ja", "ルール抽選"),
    description_localized("ja", "ルール抽選")
)]
pub async fn rule(
    ctx: Context<'_>,
    #[name_localized("ja", "ナワバリバトルを含む")]
    #[description_localized("ja", "指定なしで除外")]
    contains_turfwar: Option<BoolChoice>,
) -> anyhow::Result<()> {
    is_bot!(ctx);

    let rules = Rule::get(contains_turfwar.unwrap_or(BoolChoice::False).into());

    let mut rng = {
        let now = chrono::Utc::now();
        let id = ctx.author().id.get();
        Biski64::from(now.timestamp().wrapping_add(id as i64) as u64)
    };

    let Some(rule) = rng.choice(&rules) else {
        ctx.send(reply!("error: `{}:{}`", file!(), line!())).await?;
        return Ok(());
    };
    ctx.send(reply!("{}", rule.name())).await?;

    Ok(())
}
