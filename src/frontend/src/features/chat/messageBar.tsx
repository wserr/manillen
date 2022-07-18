import {useState} from "react";

interface MessageBarProps
{
    onSend: (message: string) => void;
}


export const MessageBar = (props: MessageBarProps) => {
    const [message, setMessage] = useState<string>('');
    return (<>
        <input type={"text"} onChange={(value) => setMessage(value.target.value)}/>
        <button onClick={() => props.onSend(message)}>Send Message</button>
    </>)
}
