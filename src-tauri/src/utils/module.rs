use tauri::{Builder, Runtime};


pub trait ModuleExtend<R:Runtime>{
    fn module<F>(self,func:F) -> Builder<R> where F:Fn(Builder<R>) -> Builder<R>;
}

impl <R:Runtime> ModuleExtend<R> for Builder<R>{
    fn module<F>(self, func: F) -> Builder<R>
    where
        F: Fn(Builder<R>) -> Builder<R>
    {
        func(self)
    }
}