import { useAppSelector } from "../../app/hooks";
import { currentState, WebSocketStatusEnum } from "./webSocketStatusSlice";

export const WebSocketStatus = () => {
    const state = useAppSelector(currentState);

    const stateToReadable = (state: WebSocketStatusEnum): string => {
        switch (state) {
            case WebSocketStatusEnum.Connected:
                return "Connected";
            case WebSocketStatusEnum.Disconnected:
                return "Disconnected";
            case WebSocketStatusEnum.Connecting:
                return "Connecting";
            case WebSocketStatusEnum.Error:
                return "Error";
        }
    }
    return <p>{`Websocket status: ${stateToReadable(state)}`}</p>;
}