import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { invoke } from "@tauri-apps/api/core";


export interface Account{
    id: string;
    name: string;
    skin: string;
}

interface AccountsState{
    userNow: string | undefined;
    accounts: Map<string, Account>; // id -> Account
}

const initialState: AccountsState = {
    userNow: undefined,
    accounts: new Map<string, Account>()
}


const accountSlice = createSlice({
    name: 'account',
    initialState,
    reducers: {
        switchAccount: (state, action:PayloadAction<string>) => { // string is id of account
            invoke("switch").catch(console.error)
            state.userNow = action.payload;
        },
        addAccount: (state, action:PayloadAction<Account>) => {
            state.accounts.set(action.payload.id, action.payload);
        },
        logoutAccount: (state, action:PayloadAction<string>) => { // string is id of account

            invoke('logout',{payload:action.payload}).catch(console.error);
            state.accounts.delete(action.payload);

        },
        initAccount: (state,action:PayloadAction<Account[]>) => {
            for (let account of action.payload){
                state.accounts.set(account.id, account);
            }
        }
    }
})

export const { switchAccount, addAccount, logoutAccount, initAccount } = accountSlice.actions;

export default accountSlice.reducer;