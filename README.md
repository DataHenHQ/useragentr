# useragentr
DataHen useragentr is a Rust package and standalone cli tool that generates a random combination of millions of user-agents strings. Currently used in production at DataHen to crawl/scrape through billions of pages.

# Usage

There are two ways to use this. Either as a Rust package, or as a standalone cli.

## As Rust package



Example code:

```rust
use lib::user_agent::{Type, UserAgent};

fn main() {
  // uses the default user agent configuration, by specifying None to the path parameter
    let ua = UserAgent::new(None).unwrap();
    for _ in 0..20 {
        let result = ua.generate_ua(&Type::Desktop).unwrap();
        println!("{}", result);
    }
}

```
You can see [more examples here](/examples)

## As standalone binary
To run as a standalone binary, first build the appropriate binary for your system, and install it.

You can then use it to generate a user agent string like so:

```shell
$ useragentr generate desktop                                                    

# will output:
Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.83 Safari/537.36 Edg/85.0.564.44
```

If you would like to generate multiple combination of user-agents, you can do the following:

```shell
$ useragentr generate desktop -n 2                                                    

# will output:
Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.83 Safari/537.36 Edg/85.0.564.44
Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/84.0.4147 Safari/537.36
```
