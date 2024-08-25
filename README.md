
[![Rust](https://github.com/JKI757/ollama-like-api-server/actions/workflows/rust.yml/badge.svg)](https://github.com/JKI757/ollama-like-api-server/actions/workflows/rust.yml)

Simple server to implement an ollama like endpoint.  

The concept is to integrate with a chatbot that provides a different kind of incompatible endpoint.

This is essentially *all* chatgpt generated.  I fed the ollama api document into chatgpt and asked it to make me an endpoint implementing the api in rust.  It's solely an experiment to help me learn Rust, take it as such.  I put the MIT license on it as the ollama project is MIT licensed, but I'm not really sure any copyright can attach to this at all due to its source.


- Build
- - cargo build

- Run
- - cargo run 
  
- Debug
- - RUST_LOG=debug cargo run 