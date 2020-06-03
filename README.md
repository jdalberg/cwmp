cwmp, an implementation of the CWMP protocol in Rust
====================================================

[![Build Status][build-status-img]](https://github.com/jdalberg/cwmp/actions?query=workflow%3ACI)


Building and using
------------------
*not on crates.io yet

Parsing
-------

```rust,no_run
extern crate cwmp;

use cwmp::parse;

fn main() {
    let s: String = "<some cwmp xml document>";

    match cwmp::parse(s) {
        Ok(parsed) => println!("{:?}", parsed),
        Err(e) => println!("Error [{:?}] occured", e)
    }
}
```