import { createSlice, PayloadAction } from "@reduxjs/toolkit"
import { RootState } from "../../app/store";

export enum WebSocketStatusEnum {
    Connected,
    Disconnected,
    Connecting,
    Error
}

export interface WebSocketStatusState {
    status: WebSocketStatusEnum
}

const initialState: WebSocketStatusState = {
    status: WebSocketStatusEnum.Disconnected
}

export const webSocketStatusSlice = createSlice({
    name: 'webSocketStatus',
    initialState,
    reducers: {
        updateState: (state, action: PayloadAction<WebSocketStatusEnum>) => {
            state.status = action.payload;
        }
    }
});

export const { updateState } = webSocketStatusSlice.actions;

export const currentState = (state: RootState) => state.webSocketStatus.status;

export default webSocketStatusSlice.reducer;