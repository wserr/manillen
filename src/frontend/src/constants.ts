export const WebSocketUrl: string = GetSetting("WebsocketUrl", process.env.REACT_APP_WEBSOCKET_URL, true);

function GetSetting(name: string, input: string | undefined, required: boolean): string | never
{
    if (required && !input || input === '')
    {
        throw new Error(`${name} is not defined.`);
    }

    return input!;
}


