import './App.css';
import { WebSocketProvider } from './features/webSocket/WebSocket';
import { ChatComponent } from './features/chat/Chat';
import { WebSocketStatus } from './features/webSocketStatus/WebSocketStatus';
import { ReactKeycloakProvider } from '@react-keycloak/web';
import { Router } from './routes';


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
