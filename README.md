# Project Title

todo-api-rust

## Description

Simple TODO REST API written in rust a.k.a rust boilerplate

## Getting Started

### Dependencies

- Cargo
- MongoDB 5

### Installing

- Just clone the repo

### Executing program

- Cd into project directory
- Edit `mongodb.rs`, change into your mongodb credential
- Execute `cargo run`
- visit http://localhost:8000

### TODO

- [x] implement partial update
- [x] implement efficient update
- [x] implement request validation
- [x] add better error handling
- [ ] make sure perfomance ok with load test
- [ ] find way to abstract controller without actix_web dependency
