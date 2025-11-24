# Secret Santa Generator

This is a simple web service built in Rust to generate Secret Santa pairings. It provides a single API endpoint that accepts a list of participants and returns a randomly shuffled list of giver-receiver pairs.

## Project Structure

This project is a Rust workspace composed of two crates:

-   `crates/secret_santa_lib`: This is the core logic library. It contains the business logic for shuffling the list of participants (using the Fisher-Yates algorithm) and generating the pairings in a cyclical manner to ensure everyone gives and receives one gift.
-   `crates/server`: This crate contains the web server built with the [Axum](https://github.com/tokio-rs/axum) framework. It exposes the HTTP API and uses `secret_santa_lib` to perform the generation.

The main binary entry point is located in `bin/secret_santa/src/main.rs`, which starts the server.

## How to Run

You can build and run the project using standard Cargo commands.

1.  **Build the project:**
    ```sh
    cargo build --release
    ```

2.  **Run the server:**
    ```sh
    cargo run --bin secret_santa
    ```
    The server will start and listen on `0.0.0.0:8080`.

## API Usage

There is one primary endpoint available: `/generate`.

### `/generate`

-   **Method:** `POST`
-   **Description:** Accepts a list of names and returns a randomly generated list of Secret Santa pairs.
-   **Request Body:** A JSON object with a `users` key containing an array of strings.
-   **Success Response:** A JSON object with a `pairs` key containing an array of `[giver, receiver]` tuples.
-   **Error Response:** If the list of users is empty, it returns a `400 Bad Request` with a plain text error message.

#### Example: Successful Request

Here is an example `curl` command to generate pairings for four people.

**Request:**
```sh
curl -X POST \
  http://localhost:8080/generate \
  -H "Content-Type: application/json" \
  -d '{"users":["Alice","Bob","Charlie","David"]}'
```

**Example Success Response:**
```json
{
  "pairs": [
    ["Charlie", "Alice"],
    ["David", "Charlie"],
    ["Alice", "Bob"],
    ["Bob", "David"]
  ]
}
```
*(Note: The order of pairs will be random each time you make a request)*

#### Example: Error Request

Here is an example of what happens when you send an empty list of users.

**Request:**
```sh
curl -X POST \
  http://localhost:8080/generate \
  -H "Content-Type: application/json" \
  -d '{"users":[]}'
```

**Error Response (Status `400`):**
```
The list of users cannot be empty.
```
