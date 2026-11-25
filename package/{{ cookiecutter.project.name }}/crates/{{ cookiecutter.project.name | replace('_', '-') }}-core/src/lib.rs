//! Domain library for `{{ cookiecutter.project.name }}`.
{%- if cookiecutter.rust_bindings == 'bin' %}

mod path_error;
pub mod skill;

pub use crate::path_error::IoResultExt;
pub use crate::path_error::PathError;
{%- endif %}
