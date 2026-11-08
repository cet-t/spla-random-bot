mod raw;

pub use raw::*;

const URL: &str = "https://stat.ink/api/v3/stage";

pub async fn get_raw() -> anyhow::Result<String> {
    Ok(reqwest::get(URL).await?.text().await?)
}

pub fn parse_raw(raw: String) -> anyhow::Result<Vec<Stage>> {
    let weapons: Vec<Stage> = serde_json::from_str(&raw)?;
    Ok(weapons)
}
