mod info;
mod logout;

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
                let accounts = NLAccounts::load(&config_path).await.unwrap_or(NLAccounts::default());
                app.manage(accounts);
                app.manage(config_path);
                Ok::<(), anyhow::Error>(())
            }).expect("WTF");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![


        ])
    
}