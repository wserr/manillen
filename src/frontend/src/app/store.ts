import { configureStore, ThunkAction, Action } from '@reduxjs/toolkit';
import counterReducer from '../features/counter/counterSlice';
import webSocketStatusReducer from '../features/webSocketStatus/webSocketStatusSlice';
import chatReducer from '../features/chat/chatSlice';

export const store = configureStore({
  reducer: {
    counter: counterReducer,
    webSocketStatus: webSocketStatusReducer,
    chat: chatReducer

  },
});

export type AppDispatch = typeof store.dispatch;
export type RootState = ReturnType<typeof store.getState>;
export type AppThunk<ReturnType = void> = ThunkAction<
  ReturnType,
  RootState,
  unknown,
  Action<string>
>;
