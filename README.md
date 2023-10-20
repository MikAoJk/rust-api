# rust api
Simple application written in rust that serves an api

## Technologies used
* Rust
* Cargo
* Docker

### Prerequisites
Make sure you have the rust installed using this command:
#### Rust
```bash script
rustc --version
```

#### Cargo
Make sure you have cargo installed using this command:
```bash script
cargo --version
```

### Build
Build the code without running it
```bash script
cargo build
```

### Test
Build the code and run all the tests
```bash script
cargo test
```

### Run
Run the code
```bash script
cargo run
```

#### Running the application locally

#####  Create docker image of app
Creating a docker image should be as simple as
``` bash
docker build -t rust-api .
```

#### Running a docker image
``` bash
docker run --rm -it -p 8080:8080 rust-api
```

##### 🧪 Testing the applications endpoints

Request to is_alive
```bash script
curl --location --request GET 'http://0.0.0.0:8080/is_alive'
```

Request to is_ready
```bash script
curl --location --request GET 'http://0.0.0.0:8080/is_ready'
```

Request to cars
```bash script
curl --location --request GET 'http://0.0.0.0:8080/cars'
```

Root endpoint
With your browser go to http://0.0.0.0:8080 to see the staic html file


### Contact
This project is maintained by [CODEOWNERS](CODEOWNERS)