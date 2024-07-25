# Course Rust

## Project spec

The cargo project, `kvs`, builds a command-line key-value store client called
`kvs`, which in turn calls into a library called `kvs`.

The `kvs` executable supports the following command line arguments:

- `kvs set <KEY> <VALUE>`

  Set the value of a string key to a string

- `kvs get <KEY>`

  Get the string value of a given string key

- `kvs rm <KEY>`

  Remove a given key

- `kvs -V`

  Print the version
