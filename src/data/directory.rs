use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Directory {
    Markdown,
}

impl Directory {
    pub fn get_name(&self) -> &str {
        use Directory::*;

        match self {
            Markdown => "markdown",
        }
    }

    pub fn get_path(&self) -> String {
        format!("./store/{}", self.get_name())
    }
}
