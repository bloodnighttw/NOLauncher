//! This module is used to extend the tauri builder to make the code more readable and maintainable
//! It's make us be able to run setup, manage, invoke_handler in module file.
//! And without the need to write a large amount of import code in the main file.

use std::error::Error;
use std::ops::Deref;
use std::sync::Arc;
use tauri::{App, Builder, Runtime};
use tauri::ipc::Invoke;

type SetupExpandFn<R> = dyn Fn(&&mut App<R>) -> Result<(), Box<dyn Error>> + Send + Sync + 'static;

pub struct BuilderWrapper<R> where R:Runtime{
    builder: Builder<R>,
    expands: Vec<Arc<SetupExpandFn<R>>> // we need to store in arc
}

/// Rust orphans rule required a trait in this scope to implement the this trait
pub trait ModuleExtend<R> where R:Runtime{
    fn module<F>(self,func:F) -> BuilderWrapper<R> where F:Fn(BuilderWrapper<R>) -> BuilderWrapper<R>;
}

/// Implement the ModuleExtend trait for Builder
impl <R> ModuleExtend<R> for Builder<R> where R:Runtime{
    fn module<F>(self, func: F) -> BuilderWrapper<R>
    where
        F: Fn(BuilderWrapper<R>) -> BuilderWrapper<R>
    {
        let wrapper = self.into();
        func(wrapper)
    }
}

/// Implement the From trait for BuilderWrapper
/// This is used to convert the Builder to BuilderWrapper
impl <R> From<Builder<R>> for BuilderWrapper<R> where R:Runtime{
    fn from(builder: Builder<R>) -> Self{
        Self{
            builder,
            expands: Vec::new()
        }
    }
}

impl <R> Deref for BuilderWrapper<R> where R:Runtime{
    type Target = Builder<R>;

    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}

/// Implement the BuilderWrapper
impl <R> BuilderWrapper<R> where R:Runtime{

    /// this should always be the last function to be called before build
    /// Note this function is actually run setup inner, so the setup you call outside the wrapper will not run!
    pub fn expand(self) -> Builder<R>{
        self.builder.setup(move |app|{
            for func in self.expands {
                func(&app)?;
            }
            Ok(())
        })
    }

    /// this function is used to setup the app
    /// Note the function argument is &&mut App<R>, not &mut App<R>
    pub fn setup<F>(mut self, func: F) -> Self
    where F: Fn(&&mut App<R>) -> Result<(), Box<dyn Error>> + Send + Sync + 'static{
        self.expands.push(Arc::from(func));
        Self{
            builder: self.builder,
            expands: self.expands
        }
    }

    /// to manage the data in the app
    pub fn manage<T>(self, data:T) -> Self
    where T:Send+Sync+'static{
        let builder = self.builder.manage(data);
        Self{
            builder,
            expands: self.expands
        }
    }

    /// to register command
    pub fn invoke_handler<F>(self,handlers:F) -> Self
    where F:Fn(Invoke<R>) -> bool + Send + Sync + 'static{
        let builder = self.builder.invoke_handler(handlers);
        Self{
            builder,
            expands: self.expands
        }
    }

    /// to register another module
    pub fn module<F>(self, func: F) -> Self
    where F: Fn(BuilderWrapper<R>) -> BuilderWrapper<R>
    {
        func(self)
    }

}