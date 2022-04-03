use mlua::prelude::{Lua, LuaFunction, LuaResult, LuaTable};

// TODO: Rework to `nvim.notify`

#[inline]
pub fn vim_api(lua: &Lua) -> LuaResult<LuaTable> {
    lua.globals().get::<_, LuaTable>("vim")?.get("api")
}

pub fn log_err(lua: &Lua, msg: &str) -> LuaResult<()> {
    let log: LuaFunction = vim_api(lua)?.get("nvim_err_writeln")?;
    log.call(msg)
}
