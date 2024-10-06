# Tsumego Practice


## How to build

As the backend is written in Rust, you'll need [Cargo](https://doc.rust-lang.org/stable/cargo/) to build it, and you need to be in the `backend/` directory:

```
cd backend
cargo build
cd ..
```

Before building the database, clone [sanderland/tsumego](https://github.com/sanderland/tsumego.git), which has a library of about 15,000 tsumego problems.

```
git clone https://github.com/sanderland/tsumego.git sanderland-tsumego
```

Now run the Python script to set up the backend database. This script extracts the problem data, converts it into the format used by this application, and initialises the database. This might not work if you haven't already built the backend. You must also install [SQLite 3](https://www.sqlite.org/) if you don't have it already.

```
./init_db.sh
```

Now build the frontend: you'll need the [TypeScript](https://www.typescriptlang.org/) compiler.

```
./build_frontend.sh
```

The frontend build will also be copied into the `backend/static/` directory, ready to be served.


## How to run the tests

There are some unit tests for model types and utility functions in the backend and frontend projects.

- To run the tests for the backend, run `cargo test` from the `backend/` directory.
- To run the tests for the frontend, open `frontend/tests.html` in a browser.
