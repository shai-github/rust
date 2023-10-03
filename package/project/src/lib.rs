/*
If lib.rs is defined in root of source directory, then Rust will automatically create
a library crate with same name as package and lib.rs will be the crate root.

Some more rules
1. A package must have at least one crate
2. A package can have either zero library crates or one library crate
3. A package can have multiple binary crates by placing files in src/bin
*/