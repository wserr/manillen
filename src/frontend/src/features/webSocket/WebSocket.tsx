import { DefaultEventsMap } from "@socket.io/component-emitter";
import { createContext } from "react";
import { useDispatch } from "react-redux";
import { useAppDispatch } from "../../app/hooks";
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
    let webSocket = new WebSocket("ws://localhost:8080/ws/");
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