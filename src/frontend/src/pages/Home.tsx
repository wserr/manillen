import { useKeycloak } from "@react-keycloak/web";

export default () => {
    const {keycloak} = useKeycloak();
    return(
    <>
        <h1>Welcome to the manillen app!</h1>
        <button onClick={() => keycloak?.login()}>Login</button>
    </>
    )
}
