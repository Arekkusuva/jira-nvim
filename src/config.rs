use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use dirs;
use mlua::prelude::{FromLua, Lua, LuaError, LuaResult, LuaValue};

#[derive(Debug)]
pub struct Config {
    host: String,
    token: String,
}

impl Config {
    #[inline]
    pub fn host(&self) -> &str {
        &self.host
    }

    #[inline]
    pub fn token(&self) -> &str {
        &self.token
    }
}

impl<'lua> FromLua<'lua> for Config {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let table = match value {
            LuaValue::Table(table) => table,
            _ => {
                return Err(LuaError::FromLuaConversionError {
                    from: value.type_name(),
                    to: "config::Config",
                    message: Some("Expected table".into()),
                })
            }
        };

        // Host
        let host = match table.get::<_, String>("host") {
            Ok(host) => host,
            Err(_) => {
                return Err(LuaError::FromLuaConversionError {
                    from: "table",
                    to: "config::Config",
                    message: Some("Missed `host` in config".into()),
                });
            }
        };

        // Path to file with token
        let token_path = match table.get::<_, String>("token_path") {
            Ok(path) => get_absolute_path(&path),
            Err(_) => {
                let mut default_path = dirs::home_dir().unwrap_or("/".into());
                default_path.push(".config/jira-nvim/token.txt");
                default_path
            }
        };

        // Read token from file
        let mut file = File::open(&token_path)?;
        let mut buf = [0_u8; 1024];
        let mut read = file.read(&mut buf)?;
        if read < 2 {
            return Err(LuaError::FromLuaConversionError {
                from: "table",
                to: "config::Config",
                message: Some("Short or empty token".into()),
            });
        }
        if buf[read - 1] == '\n' as u8 {
            read -= 1;
            if buf[read - 1] == '\r' as u8 {
                read -= 1;
            }
        }

        Ok(Config {
            host,
            token: std::str::from_utf8(&buf[..read]).unwrap().into(),
        })
    }
}

fn get_absolute_path(path: &str) -> PathBuf {
    if path.starts_with("~") {
        let mut home = dirs::home_dir().unwrap_or("".into());
        if home.as_path() == Path::new("/") {
            path.strip_prefix("~").unwrap().into()
        } else {
            home.push(path.strip_prefix("~/").unwrap());
            home
        }
    } else {
        path.into()
    }
}
