import {configureStore} from "@reduxjs/toolkit";

import accountReducer from "./state/account/accountSlice";
import accountListReducer from "./state/side-panel/accountListSlice";
// @ts-ignore
import reactron from "../ReactotronConfig"

const __DEV__ = import.meta.env.DEV

export const store = configureStore({
    reducer: {
        account: accountReducer,
        accountPanel: accountListReducer
    },
    // middleware: (getDefaultMiddleware) => getDefaultMiddleware().concat(logger),
    enhancers: (defaultEnhancer) => __DEV__ ? defaultEnhancer().concat(reactron.createEnhancer!()) : defaultEnhancer(),


})


export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch
