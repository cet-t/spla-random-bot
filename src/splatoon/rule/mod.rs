#[derive(Debug, Clone, Copy, poise::ChoiceParameter)]
pub enum Rule {
    #[name = "ナワバリバトル"]
    TurfWar,
    #[name = "ガチエリア"]
    SplatArea,
    #[name = "ガチヤグラ"]
    TowerControl,
    #[name = "ガチホコバトル"]
    Rainmaker,
    #[name = "ガチアサリ"]
    ClamBlitz,
}

impl Rule {
    pub fn all() -> Vec<Self> {
        vec![
            Self::TurfWar,
            Self::SplatArea,
            Self::TowerControl,
            Self::Rainmaker,
            Self::ClamBlitz,
        ]
    }

    pub fn gachi() -> Vec<Self> {
        vec![
            Self::SplatArea,
            Self::TowerControl,
            Self::Rainmaker,
            Self::ClamBlitz,
        ]
    }

    pub fn get(contains_turfwar: bool) -> Vec<Self> {
        if contains_turfwar {
            Self::all()
        } else {
            Self::gachi()
        }
    }
}
