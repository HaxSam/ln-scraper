use once_cell::sync::OnceCell;
use surf::Client;

pub const LIGHTNOVEL_SITE: &str = "https://readlightnovels.net";

pub static CLIENT: OnceCell<Client> = OnceCell::new();
