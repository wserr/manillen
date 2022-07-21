import {MessageBar} from "./messageBar";
import {Overview} from "./overview";
import {useState} from "react";
import {WebSocketWrapper} from "./web_socket";

export const ChatComponent = () => {
    const [messages, set_messages] = useState<string[]>([]);
    const add_message = (input: string) => {
        set_messages([...messages, input]);
    }
    let socketWrapper = new WebSocketWrapper(process.env.REACT_APP_CHAT_URL ?? '');
    socketWrapper.webSocket.onmessage = (message: MessageEvent) => {
        add_message(message.data);
    }
    const send_message = (input: string) => {
        socketWrapper.sendMessage(input);
    }
    return (<>
        <Overview Messages={messages}></Overview>
        <MessageBar onSend={send_message}></MessageBar>
    </>)


}
