use crate::splatoon::weapon::raw::Weapon;

pub use filter::*;

mod filter;
mod raw;

const URL: &str = "https://stat.ink/api/v3/weapon";

pub async fn get_raw() -> anyhow::Result<String> {
    Ok(reqwest::get(URL).await?.text().await?)
}

pub fn parse_raw(raw: String) -> anyhow::Result<Vec<Weapon>> {
    let weapons: Vec<Weapon> = serde_json::from_str(&raw)?;
    Ok(weapons)
}
