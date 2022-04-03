use mlua::prelude::{Lua, LuaResult, LuaTable};

mod config;
mod jira;
mod nvim;

use config::Config;
use jira::client::JiraClient;
use jira::error::JiraResult;
use jira::models::Issue;

/// Sets config and creates Jira client.
/// Merges new config with default inside [`Config::from_lua`].
fn setup(lua: &Lua, config: Config) -> JiraResult<()> {
    let client = JiraClient::new(config.host(), config.token()).unwrap();
    lua.set_app_data(client);
    Ok(())
}

/// Executes jql query with Jira API and returns matched issues.
fn query(lua: &Lua, query: String) -> JiraResult<Vec<Issue>> {
    let client = lua.app_data_ref::<JiraClient>().unwrap();
    let issues = client.query(&query)?;
    Ok(issues)
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
    Ok(exports)
}
