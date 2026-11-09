mod splatoon;

pub fn commands() -> Vec<crate::Command> {
    vec![splatoon::weapon(), splatoon::stage(), splatoon::rule()]
}
