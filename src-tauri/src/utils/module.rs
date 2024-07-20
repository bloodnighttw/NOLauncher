use std::error::Error;
use std::sync::Arc;
use tauri::{App, Builder, Runtime};
use tauri::ipc::Invoke;

pub trait ModuleExtend<R> where R:Runtime{
    fn module<F>(self,func:F) -> BuilderWrapper<R> where F:Fn(BuilderWrapper<R>) -> BuilderWrapper<R>;
}

impl <R> ModuleExtend<R> for Builder<R> where R:Runtime{
    fn module<F>(self, func: F) -> BuilderWrapper<R>
    where
        F: Fn(BuilderWrapper<R>) -> BuilderWrapper<R>
    {
        let wrapper = self.into();
        func(wrapper)
    }
}

pub struct BuilderWrapper<R> where R:Runtime{
    builder: Builder<R>,
    expands: Vec<Arc<dyn Fn(&&mut App<R>) -> Result<(), Box<dyn Error>> + Send + Sync + 'static>> // we need to store in arc
}

impl <R> From<Builder<R>> for BuilderWrapper<R> where R:Runtime{
    fn from(builder: Builder<R>) -> Self{
        Self{
            builder,
            expands: Vec::new()
        }
    }
}

impl <R> BuilderWrapper<R> where R:Runtime{

    pub fn setup<F>(mut self, func: F) -> Self
    where F: Fn(&&mut App<R>) -> Result<(), Box<dyn Error>> + Send + Sync + 'static{
        self.expands.push(Arc::from(func));
        Self{
            builder: self.builder,
            expands: self.expands
        }
    }

    pub fn expand(self) -> Builder<R>{
        self.builder.setup(move |app|{
            for func in self.expands {
                func(&app)?;
            }
            Ok(())
        })
    }

    pub fn manage<T>(self, data:T) -> Self
    where T:Send+Sync+'static{
        let builder = self.builder.manage(data);
        Self{
            builder,
            expands: self.expands
        }
    }

    pub fn invoke_handler<F>(self,handlers:F) -> Self
    where F:Fn(Invoke<R>) -> bool + Send + Sync + 'static{
        let builder = self.builder.invoke_handler(handlers);
        Self{
            builder,
            expands: self.expands
        }
    }

    pub fn module<F>(self, func: F) -> Self
    where F: Fn(BuilderWrapper<R>) -> BuilderWrapper<R>
    {
        func(self)
    }

}