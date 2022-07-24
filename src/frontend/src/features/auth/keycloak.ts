import Keycloak from "keycloak-js";
const keycloak = Keycloak(process.env.REACT_APP_KEYCLOAK_FILE);
keycloak.redirectUri = process.env.REACT_APP_REDIRECT_URI;
export default keycloak;