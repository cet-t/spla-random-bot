/*
   {
        "key": "52gal",
        "aliases": [
            "52_gal",
            "50"
        ],
        "type": {
            "key": "shooter",
            "aliases": [],
            "name": {
                "en_US": "Shooters",
                "ja_JP": "シューター"
            }
        },
        "name": {
            "en_US": ".52 Gal",
            "ja_JP": ".52ガロン"
        },
        "matching_range": 13.3,
        "main": "52gal",
        "sub": {
            "key": "splashshield",
            "aliases": [],
            "name": {
                "en_US": "Splash Wall",
                "ja_JP": "スプラッシュシールド"
            }
        },
        "special": {
            "key": "megaphone51",
            "aliases": [],
            "name": {
                "en_US": "Killer Wail 5.1",
                "ja_JP": "メガホンレーザー5.1ch"
            }
        },
        "reskin_of": "52gal"
    },
*/

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Weapon {
    #[serde(alias = "type")]
    pub wtype: Key,
    pub name: Name,
    pub sub: Key,
    pub special: Key,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Key {
    pub name: Name,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Name {
    #[serde(rename = "ja_JP")]
    pub ja_jp: String,
}
