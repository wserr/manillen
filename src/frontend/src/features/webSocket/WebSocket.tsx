import { createContext } from "react";
import { useDispatch } from "react-redux";
import { useAppDispatch } from "../../app/hooks";
import { WebSocketUrl } from "../../constants";
import { updateMessages } from "../chat/chatSlice";
import { updateState, WebSocketStatusEnum } from "../webSocketStatus/webSocketStatusSlice";

const WebSocketContext = createContext<SocketContext | undefined>(undefined);

export { WebSocketContext };

export interface SocketContext {
    socket: WebSocket
}

export interface WebSocketProviderProps {
    children: JSX.Element | JSX.Element[]
}

export const WebSocketProvider = (props: WebSocketProviderProps) => {
    console.log(WebSocketUrl);
    let webSocket = new WebSocket(WebSocketUrl);
    const dispatch = useDispatch();

    webSocket.onopen = () => {
        dispatch(updateState(WebSocketStatusEnum.Connected));
    }

    webSocket.onmessage = (ev: MessageEvent) => {
        dispatch(updateMessages(ev.data));
    }

    webSocket.onclose = () => {
        dispatch(updateState(WebSocketStatusEnum.Disconnected));
    }
    webSocket.onerror = () => {
        dispatch(updateState(WebSocketStatusEnum.Error));
    }
    let ws: SocketContext = {
        socket: webSocket
    }

    return (
        <WebSocketContext.Provider value={ws}>
            {props.children}
        </WebSocketContext.Provider>
    )
}