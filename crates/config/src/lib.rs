#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Frontend {
    #[default]
    Tui,
}

impl Frontend {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "tui" => Some(Self::Tui),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AppConfig {
    pub frontend: Frontend,
}

impl AppConfig {
    pub fn from_args(args: impl IntoIterator<Item = String>) -> Self {
        let mut frontend = Frontend::default();
        let mut iter = args.into_iter();

        while let Some(arg) = iter.next() {
            if arg == "--frontend" {
                if let Some(value) = iter.next().and_then(|value| Frontend::parse(&value)) {
                    frontend = value;
                }
            }
        }

        Self { frontend }
    }
}
