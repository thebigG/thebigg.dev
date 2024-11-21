[![CI to Docker Hub](https://github.com/thebigG/thebigg.dev/actions/workflows/main.yml/badge.svg)](https://github.com/thebigG/thebigg.dev/actions/workflows/main.yml)
# thebigg.dev
Source for thebigg.dev

## Environment Setup
```
rustup target add wasm32-unknown-unknown
# See https://trunkrs.dev/#install for further details
cargo install trunk --version 0.21.4
```
  
## Run Inside Docker  
```
make run_container
Go to http://0.0.0.0:8080/
```  


