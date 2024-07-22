use serde::Serialize;
use tauri::State;

use crate::utils::accounts::{AccountPayload, NLAccounts};
use crate::utils::result::CommandResult;

#[derive(Debug, Serialize, Clone)]
pub struct AccountNowPayload{
    id:Option<String>
}


#[tauri::command]
pub async fn accounts_list(
    accounts:State<'_,NLAccounts>
) -> CommandResult<Vec<AccountPayload>>{

    let accounts = accounts.list().await;
    let accounts_payload = accounts.into_iter().map(|account|{
        AccountPayload{
            id:account.profile.id.clone(),
            name:account.profile.name.clone(),
            skin:account.profile.skins.iter().find(|x| x.state == "ACTIVE").unwrap().url.clone()
        }
    }).collect();    

    Ok(accounts_payload)   
}

#[tauri::command]
pub async fn accounts_now(
    accounts:State<'_,NLAccounts>
) -> CommandResult<AccountNowPayload>{
    let account = accounts.me().await;

    let me_payload = match account {
        Some(account) => AccountNowPayload{
            id:Some(account.profile.id.clone())
        },
        None => AccountNowPayload{
            id:None
        }
    };

    Ok(me_payload)
}