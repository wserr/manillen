import { useKeycloak } from "@react-keycloak/web";
import { Navigate, Route, useLocation } from "react-router-dom"

export function RequireAuth({ children} : {children: JSX.Element }) {
    const { keycloak } = useKeycloak();
    const location = useLocation();

    return keycloak?.authenticated ? children : <Navigate to="/login" state={{ from: location}} replace /> 
}