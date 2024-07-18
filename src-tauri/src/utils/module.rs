use tauri::{Builder, Runtime};

pub trait Module<R:Runtime>{
    fn build(builder: Builder<R>) -> Builder<R>;
}

pub trait ModuleExtend<R:Runtime>{
    fn module<M:Module<R>>(self) -> Builder<R>;
}

impl <R:Runtime> ModuleExtend<R> for Builder<R>{
    fn module<M:Module<R>>(self) -> Builder<R>{
        M::build(self)
    }
}