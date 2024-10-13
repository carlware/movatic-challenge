# Movatic Coding Challenge

Create a demo web application to be able to view station data from bike-share systems
from across the world by reading MDS / GBFS feeds data.

## Requirements
1. docker compose


## Launch the project
```shell
# build and run the containers
docker compose up -d

# open the weapp at http://localhost:8080

# destroy
docker compose down
```

## Dev requirements
1. [rust > 1.78 ](https://www.rust-lang.org/tools/install)
2. [asdf](https://asdf-vm.com/guide/getting-started.html)



## Development
The folder structure is a cargo project, inside the ui folder there is a React application.

### rust
`Configuration.yaml` file is used to set the application settings

```shell
# run the project
cargo run

# run test
cargo test
```

### react
```shell
# install node with asdf
asdf install

# run development project
npm start

# format and lint
npm run format
npm run lint 
```

## TODO
1. increase unit tests
2. add unit tests for react
3. use the status information instead of station id
4. add a search box
5. opaque the error responses from the API
6. Add retry pattern for the GBFS api
7. add a map instead of a list
