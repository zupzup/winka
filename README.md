# winka

## Setup & Run

Simply run `RUST_LOG=info cargo run` to run the application and a simple GUI will open up, which you can interact with.

## TODO

* Create "success" notification on clicking submit with the entered text
* refactor to some pub fields etc., without accessors
* refactor clean up blinking logic
* refactor to not use booleans as fields and arguments, use enum types as arguments, or Option
* Refactor fontsystem to use global (once cell, arc, lock)
* Finish up code
