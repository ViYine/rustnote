mod config;
pub use config::{
    xdiff::{DiffConfig, DiffProfile, ResponseProfile},
    xreq::RequestConfig,
    Action, Args, ConfigLoad, ConfigValidate, GetProfile, RequestProfile, RunArgs,
};
pub mod cli;
pub mod util;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ExtraArgs {
    headers: Vec<(String, String)>,
    query: Vec<(String, String)>,
    body: Vec<(String, String)>,
}
