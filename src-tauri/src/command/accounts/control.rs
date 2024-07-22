use tauri::State;

use crate::utils::{accounts::NLAccounts, result::CommandResult};


#[tauri::command]
pub async fn logout(
    accounts:State<'_,NLAccounts>,
    payload:String
) -> CommandResult<()>{

    accounts.logout(&payload).await?;    

    Ok(())
}

#[tauri::command]
pub async fn switch(
    accounts:State<'_,NLAccounts>,
    payload:String
) -> CommandResult<()>{
    
    accounts.switch(&payload).await?;

    Ok(())
}