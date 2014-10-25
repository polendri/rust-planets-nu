rust-planets.nu
===============

A Rust wrapper for the planets.nu API. It is currently a work in progress and is in no way usable.

Prerequisites
-------------

  * **Cargo:** This project uses Cargo as its build system, and so it is intended to be used by projects which are also built using Cargo;
  * **Rust master:** This project is maintained such that it will build using the master branch of the Rust compiler. If your project is built
    using an older version of Rust, there are no guarantees that this library will be compatible.

Installation
--------

To start using this library in a Cargo-enabled project:

### Linux

  * Install Rust and Cargo by following the Rust Guide's [installation instructions](http://doc.rust-lang.org/guide.html#installing-rust);
  * Add the following to your project's `Cargo.toml` file:
    ```
    [dependencies.planets_nu]
    git = "https://github.com/pshendry/rust-planets-nu.git"
    ```

### Other Operating Systems

No idea, sorry! Hopefully the [Rust Guide](http://doc.rust-lang.org/guide.html) has instructions for using Cargo libraries in your build environment.

Usage
-----

This library comes in two main parts: a `request` module and a `parse` module. The `request` module contains functions for making direct queries to
the planets.nu server, whereas the `parse` module contains functions for directly interpreting the content of the responses (usually JSON). As such,
typically the `request` module should be used, but if for some reason you need to make requests manually or you need to parse stored responses,
the `parse` module is available for that.

For all examples that follow, it is necessary to import the crate and appropriate module(s):

```rust
extern crate planets_nu;

use self::planets_nu::request;
use self::planets_nu::parse;
```

  * **Login:**
    * Example:

        ```rust
        let result = request::login("username", "password");
        //let result = parse::login("[json string]");
        print!(result);
        ```
    * [Reference](http://www.rust-ci.org/pshendry/rust-planets-nu/doc/planets_nu/request/fn.login.html)

Reference Documentation
-----------------------

  * Reference documentation for rust-planets-nu is available at http://www.rust-ci.org/pshendry/rust-planets-nu/doc/planets_nu/.
  * Official documentation for the planets.nu API is available on the [planets.nu website](http://planets.nu/api-documentation), and unofficial documentation is available on the [VGA Planets Wiki](http://vgaplanets.org/index.php/Planets.Nu_API).
