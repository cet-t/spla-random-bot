#[derive(Debug, Clone, Copy, poise::ChoiceParameter)]
pub enum BoolChoice {
    #[name_localized("ja", "はい")]
    True,
    #[name_localized("ja", "いいえ")]
    False,
}

impl From<BoolChoice> for bool {
    fn from(value: BoolChoice) -> Self {
        match value {
            BoolChoice::True => true,
            BoolChoice::False => false,
        }
    }
}
