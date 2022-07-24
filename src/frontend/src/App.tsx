import './App.css';
import { WebSocketProvider } from './features/webSocket/WebSocket';
import { ChatComponent } from './features/chat/Chat';
import { WebSocketStatus } from './features/webSocketStatus/WebSocketStatus';
import { ReactKeycloakProvider } from '@react-keycloak/web';
import keycloak from './features/auth/keycloak';


function App() {
  return (
    <div className="App">

      <ReactKeycloakProvider authClient={keycloak} initOptions={{ onLoad: "login-required" }}>
        <WebSocketProvider>
          <WebSocketStatus />
          <ChatComponent />
        </WebSocketProvider>
      </ReactKeycloakProvider>

    </div>
  );
}

export default App;
