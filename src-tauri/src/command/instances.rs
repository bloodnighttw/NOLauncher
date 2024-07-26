use tauri::{App, Manager, Runtime};

use crate::utils::{base_store::InstanceStorePoint, module::BuilderWrapper, settings::instances::NLInstanceList};

pub mod control;
pub mod info;


pub fn init<R>(wrapper: BuilderWrapper<R>) -> BuilderWrapper<R>
where
    R:Runtime{
    wrapper
        .setup(|app:&&mut App<R>|{
            let path:InstanceStorePoint = app.try_into()?;
            app.manage(NLInstanceList::new(path));
            Ok(()) as Result<(), Box<dyn std::error::Error>>
        })
    
}