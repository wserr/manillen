import { useContext } from "react"
import { useAppSelector } from "../../app/hooks";
import { WebSocketContext } from "../webSocket/WebSocket"
import { messages } from "./chatSlice";
import axios from 'axios';


export const ChatComponent = () => {
    let ws = useContext(WebSocketContext)!;

    const sendMessage = () => {
        ws.socket.send("testing");
    }

    const login = async () => {
        let response = await axios.get("http://localhost:5000/login");
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
            <button onClick={login}>Send</button>
        </>
    )
}