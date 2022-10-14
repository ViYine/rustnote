mod config;
pub use config::{
    get_body_text, get_header_text, get_status_text,
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
