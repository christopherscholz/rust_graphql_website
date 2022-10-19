Personal Multi-Page-Website with client server separation using a GraphQL API for Christopher Scholz build with [Rocket](https://rocket.rs/).
* Using `React`, `ReactRouter` to create the different pages
* GraphQL API via `Apollo CLient` and `Juniper`/`Rócket` (Server, including CORS Headers)

The app can be run locally or via docker.

For the `local` setup run
* server
    ```
    cd server
    cargo run
    ```
* client
    ```
    cd client
    npm start
    ```

For the `docker` setup run
```
docker-compose up
```

GraphiQL is also available under `http://127.0.0.1:8000/`. In order to get the page `home`, you could query like this
```
{
  page(name: "home") {
    name
    time
    blocks {
      id
      type
      ... on ParagraphBlock {
        data {
          text
        }
      }
      ... on HeaderBlock {
        data {
          text
          level
        }
      }
      ... on ListBlock {
        data {
          style
          items
        }
      }
    }
    version
  }
}
```