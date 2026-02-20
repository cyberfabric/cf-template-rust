{% if module_type == "http-fetcher" %}mod domain;
mod infra;
{% endif %}mod module;

pub use module::*;
