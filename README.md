# HTTP Server in Rust Workshop

<div style="display: flex; justify-content: center; width: 100vw;">
  <img src="https://rustacean.net/assets/rustacean-orig-noshadow.svg" alt="Rust crab :)" width=500 />
</div>

# Table of Contents

- [About](#about)
- [Setup](#setup)
- [Rust](#rust)
- [Rust](#rust)
- [Server](#server)

# About

## A simple http server written in rust to be used to teach a workshop for Layer Zero at UNLV

# Setup

### Install rust, cargo, and rustup

**Linux/MacOS**

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

**Windows**

go to the [rust website](https://forge.rust-lang.org/infra/other-installation-methods.html)
download and run [rustup-init.exe](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)

### Create your cargo project

With everything now installed, you can use cargo to create a project

```sh
cargo new rust_http
cd rust_http
```

# Rust

Since we have our project created, let's take a look around

We have the following file structure

```sh
rust_http/
├── Cargo.toml
└── src/
    └── main.rs
```

We already have a `main.rs` file so let' see what's going on there

First go into the src directory

```sh
cd src
```

We can run this main file by doing the following command anywhere inside of the project

```sh
cargo run
```

We can see it says `Hello, World!`

### Rust Fundamentals

Now let's get into some rust fundamentals.

This won't go over all Rust fundamentals so if you want to go deeper into the language, make sure to check out [The Rust Book](https://doc.rust-lang.org/reference/introduction.html).

Open up main.rs in your code editor of choice.

We have some boilerplate code in here already to print out "Hello World"

```rust
fn main() {
    println!("Hello, world!");
}
```

Let's go over what each of these are.

**Main function**

```rust
fn main()
```

Similar to C/C++ the entry point for the program will be the main function. Any code you want to run directly will be run in here

```rust
println!("Hello, World!");
```

`println!` is a macro that will send an output to standard out, similar to `printf()` and `cout` in C/C++ respectively.
It's important that `println!` is a macro, but we'll get into that in a bit.

Now let's go over some concepts

#### **Variables**

Rust is a statically and strongly typed language like C/C++, meaning we have to define our types and all types are evaluated on compile time.

We declare variables using the `let` keyword and all variables are immutable by default. This means variables can not be changed once assigned a value and needs to be initialized and can not be assigned a variable after declaration.

**Valid**

```rust
let foo: i32 = 100;
```

**Invalid**

```rust
let foo: i32 = 100;
foo = 55;
```

We can make variable mutable by using the `mut` keyword.

**Invalid**

```rust
let mut foo: i32 = 100;
foo = 55;
```

#### **Functions**

So we saw how there is the main function that looks like

```rust
fn main() {
  // Code here
}
```

We can declare other functions the same way.

```rust
fn foo() {
  // Code here
}
```

How about parameters?

```rust
fn foo(param1: i32) {
  // Code here
}
```

Now let's return a value

```rust
fn foo(param1: i32) {
  // Code here
  return 15;
}
```

You can also do the following to return

```rust
fn foo(param1: i32) -> i32 {
  // Code here
  15
}
```

Curly braces both define a scope and also evaluates a value to be returned from that scope

```rust
let x = { 51 };
```

#### **Conditions and Loops**

Now let's do conditions and loops

We can do if statements like the following

```rust
let x = 5;
if x > 0 {
  println!("positive")
} else if x < 0 {
  println!("negative")
} else {
  println!("???")
}
```

Rust is unique in the sense that it includes a keyword for an infinite loop.

```rust
loop {
  println!("???")
}
```

Now this we'll actually be using this, but you might want to not loop forever. If so, you can use the `break` keyword for that. There's also a`continue` keyword too.

```rust
let x = 0;
loop {
  x += 1;
  if x == 4 {
    break;
  }
  println!("{}", x)
}
```

Of course while and for loops too

```rust
let x = 0;
while x < 4 {
  x += 1;
  println!("{}", x)
}
```

For loops in rust are used for iterators however by using the `for in` keywords, so you can use iterators to loop through iterable objects or data types.

```rust
for i in 0..4 {
  println!("{}", i)
}
```

Now time for one of my favorites, `match`. This works like `switch case` in C/C++.

```rust
let x = 5;
match x {
  1 => println!("Hello"),
  2 | 3 | 4 | 5 => println!("World"),
  _ => println!("Nothing"),
}
```

#### **Ownership and Borrowing**

**Ownership**

Every piece of allocated data has an owner. The owner is a variable that references that piece of memory and there can only be a single owner at any given time.

Say we have the following code

```rust
let s = String::from("Hello");

println!("{}", s);
```

In this example, `s` owns the value returned from `String::from("hello")` which is a `String` object with the value `"Hello"`.

You can pass ownership of that allocated `String` object from s to another variable.

```rust
let s = String::from("Hello");
let s1 = s;

println!("{}", s);
```

However, this transfers the ownership of the data originally stored in `s` and gives it to `s1`. This means, `s` has no value it is pointing to and can not be used anymore. So the `println!("{}", s)` macro will no longer work.

Ownership follows 3 rules.

1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

This allows us to avoid memory leaks, dangling pointers, and other memory related issues.

Now the issue here is that when you pass a variable into a function, you're transferring ownership of the value from the variable to the function's parameter. According to rule 3 then, once the function ends, the parameter will no longer be in scope and the value it owns will be dropped. Now the variable that passed the object will no longer have anything in it.

```rust
fn main() {
    let s = String::from("Hello");

    foo(s);

    println!("{}", s);
}

fn foo(s: String) {
    println!("Printing: {}", s)
}
```

**Borrowing**

We can avoid this by using a feature called Borrowing. Borrowing is what is sounds like. When you have a value, you can temporarily allow another variable or function to use that data but once that temporary owner is out of scope, the ownership will be returned to its actual owner. This is indicated by `&`.

```rust
let s = String::from("Hello");
let s1 = &s;

println!("{}", s);
```

In a function this would look like

```rust
fn main() {
  let s = String::from("Hello");
  foo(&s);

  println!("{}", s);
}

fn foo(s: &String) {
    println!("Printing: {}", s)
}
```

Borrowing follows these 2 rules.

1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.

## Use

So in C/C++ we have the `#include` preprocessor directive to bring in external code, typically from header files which will then be compiled and linked with the implementation files. In rust you would use the `use` keyword in path.

```rust
use std
```

Now let's say you don't want the full std library and use specific parts of it. You would then use `::` to specify that

```rust
use std::io
```

What if we want to use multiple modules, or methods?

```rust
use std::{io, fs}
```

Let's get deeper into this

```rust
use std::{
  io::{Write, BufReader},
  fs::File,
  collections::HashMap
}
```

# HTTP

Now let's go over the basics of HTTP.

HTTP is an application layer protocol on the OSI model.

HTTP is an application layer protocol on the OSI model. This protocol is the foundation of data communication on the internet, defining how messages are formatted and transmitted between clients and servers.

## HTTP Methods

HTTP defines a set of request methods (also known as verbs) that indicate the desired action to be performed on a resource:

- **GET:**  
  Retrieves data from the server.

- **POST:**  
  Submits data to the server, often resulting in a change in state or side effects on the server.

- **PUT:**  
  Updates or creates a resource on the server.

- **DELETE:**  
  Removes a resource from the server.

Other methods like **HEAD**, **OPTIONS**, **PATCH**, and more provide additional control and flexibility when interacting with web resources.

## HTTP Versions and Transport

**Versions:**  
HTTP has evolved over time:

- **HTTP/1.0 & HTTP/1.1:**  
  Introduced features like persistent connections and chunked transfer encoding.
- **HTTP/2:**  
  Enhances performance through multiplexing, header compression, and server push.
- **HTTP/3:**  
  Built on the QUIC protocol, further reducing latency and improving reliability.

**Transport Protocol:**  
Typically, HTTP uses TCP as its transport layer protocol, ensuring reliable delivery. When security is a concern, HTTP is used over TLS (HTTPS), which encrypts the communication between client and server.

Understanding these core principles of HTTP is crucial for building and maintaining an effective HTTP server. With these concepts in mind, you'll be better equipped to work with HTTP requests, responses, and the overall architecture of web communications in your Rust-based server.

Let's try this out

Go into your terminal and do the following command

```sh
curl https://www.google.com
```

So there's a lot we just got, but that's just the google website.

Essentially what happened here was, we sent an HTTP get request asking for the contents of the html file being served by google and it comes back to us as an HTTP response. This is what happens in your browser when you put an url in, disregarding what happens with DNS resolution.

What we're sending would look something like this.

```http
GET /index HTTP/1.1
... Header stuff
```

And what we get in response would look something like this.

```http
HTTP/1.1 200 ok
Content-Type: text/html
<!DOCTYPE html>
<html lang="en">
// More html here
```

# Server

So let's now take what we learned about both rust and http and build ourselves an HTTP server.

We'll be using HTTP/1.1 for this.

Let's start with the main function

```rust
fn main() {}
```

Now we need a way to listen to incoming requests. For that we'll be using the TcpListner built into the net module and have it listen to localhost on port 8080.

```rust
use std::net::TcpListener,

fn main() {
  let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
}
```

Now we need to actually get the incoming requests then do something with it.

```rust
...
use std::net::TcpListener;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

  for stream in listener.incoming() {}
}
```

Now let's create a helper function to handle the request. We need to add onto the use statement at the top of the file with the `TcpStream` struct and pass a parameter of the struct type into the function.

```rust
use std::net::{TcpListener, TcpStream};

// Main function

fn handle_request(mut stream: TcpStream) {}
```

Now let's import some more stuff that we'll be using

```rust
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};
```

With that done, we can now work on the `handle_request` function. We'll first create a `String` and a `BufReader` so we can take the contents of the http request and convert it from a `TcpStream` into a `String`.

```rust
fn handle_request(mut stream: TcpStream) {
  let mut request = String::new();
  let mut reader = BufReader::new(&mut stream);

  reader.read_line(&mut request).unwrap();
}
```

Now let's make a match statement to get the status line and the content that we will return to the client based on the request. As stated above, a value can be returned by a scope, we can do the same with a match statement. When we get the case, we can return a value from the executed code into a variable or set of variables. We will also use the `as_str()` to convert the String object to a `str` which is the text itself. After this we will use `trim()` to remove any white space in front of or after the text.

```rust
fn handle_request(mut stream: TcpStream) {
  let mut request = String::new();
  let mut reader = BufReader::new(&mut stream);

  reader.read_line(&mut request).unwrap();

  let (status, content) = match request.as_str().trim() {
    "GET / HTTP/1.1" => (
      "HTTP/1.1 200 OK",
      "<html><head><title>200 Ok</title></head><body><h1>Hello, World!</h1></body></html>"
      ),
    _ => (
      "HTTP/1.1 404 Not Found",
      "<html><head><title>404 Not Found</title></head><body><h1>404 Not Found</h1></body></html>"
    )
  }
};
```

Next, let's take the returned values and write them into the stream and then return the updated stream.

```rust
fn handle_request(mut stream: TcpStream) {
  let mut request = String::new();
  let mut reader = BufReader::new(&mut stream);

  reader.read_line(&mut request).unwrap();

  let (status, content) = match request.as_str().trim() {
    "GET / HTTP/1.1" => (
      "HTTP/1.1 200 OK",
      "<html><head><title>200 Ok</title></head><body><h1>Hello, World!</h1></body></html>"
      ),
    _ => (
      "HTTP/1.1 404 Not Found",
      "<html><head><title>404 Not Found</title></head><body><h1>404 Not Found</h1></body></html>"
    )
  };

  stream
      .write_all(
          format!(
              "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
              status,
              content.len(),
              content,
          )
          .as_bytes(),
      )
      .unwrap();
}
```

Finally, let's update the for loop in the main function then run the program.

```rust
fn main() {
  let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

  for stream in listener.incoming() {
    let str = stream.unwrap();
    handle_request(str);
  }
}
```

Now in the terminal do.

```sh
cargo run
```

Copyright &copy; 2025 Austin Choi

