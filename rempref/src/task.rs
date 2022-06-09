use std::{fmt::Display, path::MAIN_SEPARATOR};

pub struct RemovePrefixTask {
    pub from: String,
    pub to: String,
}

impl From<(u8, String)> for RemovePrefixTask {
    fn from((prefix_len, from): (u8, String)) -> Self {
        let mut path = from.split(MAIN_SEPARATOR).collect::<Vec<_>>();
        let last_part = &path.remove(path.len() - 1)[prefix_len.into()..];
        path.push(last_part);
        let to = path.join(MAIN_SEPARATOR.to_string().as_str());
        RemovePrefixTask { from, to }
    }
}

impl Display for RemovePrefixTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}
