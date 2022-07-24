import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { RootState } from "../../app/store";

export interface ChatState {
    messages: string[]
}

const initialState: ChatState = {
    messages: []
}

export const chatSlice = createSlice({
    name: 'chat',
    initialState,
    reducers: {
        updateMessages: (state, action: PayloadAction<string>) => {
            state.messages.push(action.payload);
        }
    }
});

export const { updateMessages } = chatSlice.actions;

export const messages = (state: RootState) => state.chat.messages;

export default chatSlice.reducer;