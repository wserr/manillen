import { useContext } from "react"
import { useAppSelector } from "../../app/hooks";
import { WebSocketContext } from "../webSocket/WebSocket"
import { messages } from "./chatSlice";

export const ChatComponent = () => {
    let ws = useContext(WebSocketContext)!;

    const sendMessage = () => {
        ws.socket.send("testing");
    }

    let chatMessages = useAppSelector(messages);

    return (
        <>
            <li>
                {
                    chatMessages.map(c => <ul>{c}</ul>)
                }
            </li>
            <button onClick={sendMessage}>Send</button>
        </>
    )
}