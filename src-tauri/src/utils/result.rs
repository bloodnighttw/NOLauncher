//! The code is edited from https://github.com/TDiblik/anyhow-tauri/blob/master/src/lib.rs
//! by bloodnighttw and relicense under Apache 2.0, which is same as Nolauncher Project.
//!
//! Here is original license.
//!
//! MIT License
//! 
//! Copyright (c) 2024 Tomáš Diblík
//! 
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//! 
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//! 
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.

use std::io::Error;
use serde::Serialize;

// Just extending the `anyhow::Error`
#[derive(Debug,Serialize)]
pub struct CommandError{
    pub status:String,
    #[serde(serialize_with="serialize")]
    pub error:anyhow::Error
}

impl std::error::Error for CommandError {}
impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.error)
    }
}

fn serialize<S>(x:&anyhow::Error,serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
{
    serializer.serialize_str(&format!("{:?}", x))
}


impl From<anyhow::Error> for CommandError {
    fn from(error: anyhow::Error) -> Self {
        Self {
            status: "error".to_string(),
            error
        }
    }
}

impl From<std::io::Error> for CommandError{
    fn from(value: Error) -> Self {
        Self {
            status: "error".to_string(),
            error: value.into()
        }
    }
}

impl From<tauri::Error> for CommandError {
    fn from(value: tauri::Error) -> Self {
        Self {
            status: "error".to_string(),
            error: value.into()
        }
    }
}

/// Use this as your command's return type.
///
/// Example usage:
/// ```
/// #[tauri::command]
/// fn test() -> anyhow_tauri::TAResult<String> {
///     Ok("No error thrown.".into())
/// }
/// ```
///
/// You can find more examples inside the library's repo at `/demo/src-tauri/src/main.rs`
pub type CommandResult<T> = std::result::Result<T, CommandError>;

pub trait IntoTAResult<T> {
    fn into_ta_result(self) -> CommandResult<T>;
}

impl<T, E> IntoTAResult<T> for std::result::Result<T, E>
    where
        E: Into<anyhow::Error>,
{
    /// Maps errors, which can be converted into `anyhow`'s error type, into `TACommandError` which can be returned from command call.
    /// This is a "quality of life" improvement.
    ///
    /// Example usage:
    /// ```
    /// #[tauri::command]
    /// fn test_into_ta_result() -> anyhow_tauri::TAResult<String> {
    ///     function_that_succeeds().into_ta_result()
    ///     // could also be written as:
    ///     // Ok(function_that_succeeds()?)
    /// }
    /// ```
    fn into_ta_result(self) -> CommandResult<T> {
        self.map_err(|e| CommandError{
            status: "error".to_string(),
            error:e.into()
        })
    }
}
impl<T> IntoTAResult<T> for anyhow::Error {
    /// Maps `anyhow`'s error type into `TACommandError` which can be returned from a command call.
    /// This is a "quality of life" improvement.
    ///
    /// Example usage:
    /// ```
    /// #[tauri::command]
    /// fn test_into_ta_result() -> anyhow_tauri::TAResult<String> {
    ///     function_that_succeeds().into_ta_result()
    ///     // could also be written as:
    ///     // Ok(function_that_succeeds()?)
    /// }
    /// ```
    fn into_ta_result(self) -> CommandResult<T> {
        Err(CommandError{
            status:"error".to_string(),
            error:self
        })
    }
}

pub trait IntoEmptyTAResult<T> {
    /// Usefull whenever you want to create `Result<(), TACommandError>` (or `TAResult<()>`)
    ///
    /// Example usage:
    /// ```
    /// #[tauri::command]
    /// fn test_into_ta_empty_result() -> anyhow_tauri::TAResult<()> {
    ///     anyhow::anyhow!("Showcase of the .into_ta_empty_result()").into_ta_empty_result()
    /// }
    /// ```
    fn into_ta_empty_result(self) -> CommandResult<T>;
}
impl IntoEmptyTAResult<()> for anyhow::Error {
    fn into_ta_empty_result(self) -> CommandResult<()> {
        Err(CommandError{
            status:"error".to_string(),
            error:self
        })
    }
}

/// Mirrors the `anyhow::bail!` implementation, but calls `.into_ta_result()` afterwards.
///
/// Example usage:
/// ```
/// #[tauri::command]
/// fn test_bail() -> anyhow_tauri::TAResult<String> {
///     anyhow_tauri::bail!("Showcase of the .bail!()")
/// }
/// ```
#[macro_export]
macro_rules! bail {
    ($msg:literal $(,)?) => {
        return Err(anyhow::anyhow!($msg)).into_ta_result()
    };
    ($err:expr $(,)?) => {
        return Err(anyhow::anyhow!($err)).into_ta_result()
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err(anyhow::anyhow!($fmt, $($arg)*)).into_ta_result()
    };
}

/// Mirrors the `anyhow::ensure!` implementation, but calls `.into_ta_result()` afterwards.
///
/// Example usage:
/// ```
/// #[tauri::command]
/// fn test_ensure() -> anyhow_tauri::TAResult<String> {
///     anyhow_tauri::ensure!(1 == 2); // this should throw
///     Ok("this should never trigger".to_owned())
/// }
/// ```
#[macro_export]
macro_rules! ensure {
    ($cond:expr $(,)?) => {
        if !$cond {
            return Err(anyhow::anyhow!(concat!(
                "Condition failed: `",
                stringify!($cond),
                "`"
            ))).into_ta_result();
        }
    };
    ($cond:expr, $msg:literal $(,)?) => {
        if !$cond {
            return Err(anyhow::anyhow!($msg)).into_ta_result()
        }
    };
    ($cond:expr, $err:expr $(,)?) => {
        if !$cond {
            return Err(anyhow::anyhow!($err)).into_ta_result()
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !$cond {
            return Err(anyhow::anyhow!($fmt, $($arg)*)).into_ta_result()
        }
    };
}