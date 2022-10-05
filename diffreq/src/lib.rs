mod config;
pub use config::{DiffConfig, DiffProfile};
pub mod cli;
pub mod req;
pub mod util;
pub use req::{RequestProfile, ResponseProfile};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ExtraArgs {
    headers: Vec<(String, String)>,
    query: Vec<(String, String)>,
    body: Vec<(String, String)>,
}
