## Creating the Project

To create a new lib project using Cargo, run the following command:
```
cargo new --lib image-cli
```

## Project Structure

The project structure should be set up as follows:

```
image-cli/
├── Cargo.toml
├── src/
│   ├── cli.rs
│   ├── imagix/
│   │   ├── error.rs
│   │   ├── mod.rs
│   │   ├── resize.rs
│   │   └── stats.rs
```

1. In the `src` folder, create a subfolder named `imagix` to host the code.
2. In the `imagix` subfolder, create the following files:
   - `error.rs`: This file will contain custom error types and error handling code.
   - `mod.rs`: This file is the entry point for the `imagix` library.
   - `resize.rs`: This file will host the code related to image resizing.
   - `stats.rs`: This file will host the code for image file statistics.
3. In the `src` folder, create a file named `cli.rs` which will contain the command-line interface code.