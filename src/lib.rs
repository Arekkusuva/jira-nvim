use mlua::prelude::{Lua, LuaResult, LuaTable};

mod config;
mod error;
mod formatters;
mod jira;
mod lua_models;
mod nvim;
mod utils;

use config::Config;
use error::{Error, Result};
use jira::client::JiraClient;
use jira::models::Issue;

/// Sets config and creates Jira client.
/// Merges new config with default inside [`Config::from_lua`].
fn setup(lua: &Lua, config: Config) -> Result<Config> {
    let client = JiraClient::new(config.host(), config.token())?;
    lua.set_app_data(client);
    lua.set_app_data(config.clone());
    Ok(config)
}

/// Executes jql query with Jira API and returns matched issues.
fn query(lua: &Lua, query: String) -> Result<Vec<Issue>> {
    let client = lua
        .app_data_ref::<JiraClient>()
        .ok_or_else(|| Error::SetupFailed)?;
    let issues = client.query(&query)?;
    Ok(issues)
}

/// Wraps text, does not break words.
fn wrap_text(lua: &Lua, (text, max_chars): (String, usize)) -> LuaResult<LuaTable> {
    let result = lua.create_table()?;
    let mut result_cursor = 1;
    let lines = utils::wrap_text(&text, max_chars);
    for line in lines {
        result.raw_insert(result_cursor, line)?;
        result_cursor += 1;
    }
    Ok(result)
}

/// Creates lua function from rust function that can retrurn any `Result` type.
macro_rules! export_fn {
    ($lua:expr, $exports:expr, $fn:expr) => {
        $exports.set(
            stringify!($fn),
            $lua.create_function(move |lua: &Lua, args| $fn(lua, args).map_err(|err| err.into()))?,
        )
    };
}

#[mlua::lua_module]
fn libjira_nvim(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    export_fn!(lua, exports, setup)?;
    export_fn!(lua, exports, query)?;
    export_fn!(lua, exports, wrap_text)?;
    Ok(exports)
}
