pub mod info;
pub mod control;

use tauri::{App, Manager, Runtime};

use crate::utils::{accounts::NLAccounts, base_store::ConfigStorePoint, module::BuilderWrapper};

pub fn init<R>(wrapper: BuilderWrapper<R>) -> BuilderWrapper<R>
where
    R:Runtime{
    wrapper
        .setup(|app:&&mut App<R>|{
                // to init the device code data in the app
            tauri::async_runtime::block_on(async {    // added this line
                let config_path = ConfigStorePoint::try_from(app).unwrap();
                let accounts = NLAccounts::load(&config_path).await.unwrap_or(NLAccounts::default(config_path));
                app.manage(accounts);
                Ok::<(), anyhow::Error>(())
            }).expect("WTF");
            Ok(())
        })
    
}