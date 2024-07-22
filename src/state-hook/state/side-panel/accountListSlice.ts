import { createSlice } from "@reduxjs/toolkit";

interface SidePanelState{
    open: boolean;
}

const initialState: SidePanelState = {
    open: false
}

const sidePanelSlice = createSlice({
    name: 'sidePanel',
    initialState,
    reducers: {
        openSidePanel: (state) => {
            state.open = true;
        },
        closeSidePanel: (state) => {
            state.open = false;
        }   
    }
})

export const { openSidePanel, closeSidePanel } = sidePanelSlice.actions;
export default sidePanelSlice.reducer;