import {configureStore} from "@reduxjs/toolkit";

import accountReducer from "./state/account/accountSlice";
import accountListReducer from "./state/side-panel/accountListSlice";

import logger from "redux-logger"

export const store = configureStore({
    reducer: {
        account: accountReducer,
        accountPanel: accountListReducer
    },
    middleware: (getDefaultMiddleware) => getDefaultMiddleware().concat(logger),

})

export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch
