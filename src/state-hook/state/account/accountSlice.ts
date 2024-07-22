import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { invoke } from "@tauri-apps/api/core";


export interface Account{
    id: string;
    name: string;
    skin: string;
}

interface AccountsState{
    userNow: string | undefined | null;
    accounts: Account[]; // id -> Account
}

const initialState: AccountsState = {
    userNow: undefined,
    accounts: []
}


const accountSlice = createSlice({
    name: 'account',
    initialState,
    reducers: {
        switchAccount: (state, action:PayloadAction<string>) => { // string is id of account
            invoke("switch",{payload:action.payload}).catch(console.error)
            state.userNow = action.payload;
        },
        addAccount: (state, action:PayloadAction<Account>) => {
            let arr = []            
            for (let old of state.accounts){
                if (old.id !== action.payload.id){ // to prevent duplicate account
                    arr.push(old);
                }
            }

            arr.push(action.payload);

            state.accounts = arr;

        },
        logoutAccount: (state, action:PayloadAction<string>) => { // string is id of account

            invoke('logout',{payload:action.payload}).catch(console.error);
            let arr = []
            for (let account of state.accounts){
                if (account.id !== action.payload){
                    arr.push(account);
                }
            }
            
            state.accounts = arr;

        },
        initAccount: (state,action:PayloadAction<Account[]>) => {
            state.accounts = action.payload;
        },
        initUserNow: (state,action:PayloadAction<string|undefined|null>) => {
            state.userNow = action.payload;
        }
    }
})

export const { switchAccount, addAccount, logoutAccount, initAccount, initUserNow } = accountSlice.actions;

export default accountSlice.reducer;