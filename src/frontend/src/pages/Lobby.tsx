import { useKeycloak } from "@react-keycloak/web";

export default () => {
    const {keycloak} = useKeycloak();
    return(
    <>
        <h1>To view this page, you need to be authenticated</h1>
        <button onClick={() => keycloak?.logout()}>Log out</button>
    </>
    );
}