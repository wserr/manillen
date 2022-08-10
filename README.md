# Manillen

This project is a manillen card game.

The rules can be found [here](https://nl.wikipedia.org/wiki/Manillen)

## Running the project

### Prerequisites

The following software needs to be installed on your PC in order to build the project

- Docker
- Docker-compose
- Visual Studio Code (or any other IDE) with the rust plugin installed
- Cargo
- npm (latest version, recommended to install with nvm)
- Insomnia

### Running the backend

The backend can be found under `/src/chat`.

Enter `cargo run` in order to run the backend

If you now test the following request: `http://localhost:8080/userinfo`

It should return 

```json
{
	"user_name": "Willem"
}
```




