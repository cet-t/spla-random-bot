/*
    {
        "key": "yunohana",
        "aliases": [
            "gorge",
            "scorch_gorge",
            "1"
        ],
        "name": {
            "en_US": "Scorch Gorge",
            "ja_JP": "ユノハナ大渓谷"
        },
        "short_name": {
            "en_US": "Gorge",
            "ja_JP": "ユノハナ"
        },
        "area": 2144,
        "release_at": {
            "time": 1662649200,
            "iso8601": "2022-09-08T15:00:00+00:00"
        }
    },
*/

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Stage {
    pub name: Name,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Name {
    #[serde(rename = "ja_JP")]
    pub ja_jp: String,
}
