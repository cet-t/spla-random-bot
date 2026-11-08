mod common;
mod splatoon;

pub fn commands() -> Vec<crate::Command> {
    vec![
        common::ping(),
        common::info(),
        splatoon::weapon(),
        splatoon::stage(),
        splatoon::rule(),
    ]
}
