mod config;
pub use config::DiffConfig;
pub mod cli;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExtraArgs{
    headers: Vec<(String, String)>,
    query: Vec<(String, String)>,
    body: Vec<(String, String)>,
}