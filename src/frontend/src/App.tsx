import React from 'react';
import logo from './logo.svg';
import { Counter } from './features/counter/Counter';
import './App.css';
import { WebSocketProvider } from './features/webSocket/WebSocket';
import { ChatComponent } from './features/chat/Chat';
import { WebSocketStatus } from './features/webSocketStatus/WebSocketStatus';


function App() {
  return (
    <div className="App">
      <WebSocketProvider>
        <WebSocketStatus />
        <ChatComponent />
      </WebSocketProvider>
    </div>
  );
}

export default App;
