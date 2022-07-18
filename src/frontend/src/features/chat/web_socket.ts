
export class WebSocketWrapper {
    webSocket: WebSocket;

    constructor(url: string) {
        this.webSocket = new WebSocket(url);
        this.webSocket.onopen = this.onOpen;
        this.webSocket.onerror = this.onError;
        this.webSocket.onclose = this.onClose;
    }
    sendMessage(message: string): void {
        this.webSocket.send(message);
    }

    //indicates that the connection is ready to send and receive data
    onOpen(event: Event): void {
        console.log("connected");
    }

    //An event listener to be called when an error occurs. This is a simple event named "error".
    onError(event: Event): void {


    }

    //An event listener to be called when the WebSocket connection's readyState changes to CLOSED.
    onClose(event: any): void {

        console.log(JSON.stringify(event.data));

    }
}