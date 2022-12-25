# SvelteKit-Rust

This is a template for creating a SvelteKit app with Rust backend.

## Architecture

The goal of this project is to provide a template for a WebApp that can run under specific conditions:

The end users connection is not perfect. As such, requests to the app should be kept to a minimum.

Users are situated in multiple locations. As such, the frontend of the app should be able to run with multiple instances, so that users can be served from the closest instance.
We also want to be able to run the frontend on "[serverless](https://en.wikipedia.org/wiki/Serverless_computing)" platforms, so we cannot expect to store any state on the frontend, or expect a single session to be served by a single instance.

Users also may also restrict JavaScript execution. As such, the frontend of the app should be able to run (in a limited fashion) without JavaScript.

The backend of the app should be able to run on a single instance, though advanced applications may want to run multiple instances of the backend, with considerations for data consistency.

Both the frontend of the application and the backend of the application are avaliable on the internet, and the frontend may call the backend.

The backend of the application may not call the frontend.

## Development

Install and set up the project:

```bash
./setup-project.sh
```

Start the entire application with:

```bash
./start-dev.sh
```

This will start the frontend and backend in development mode.

## Deployment

### Frontend

In the current configuration, the frontend is built as a node application:

```bash
cd frontend
npm run build
```

This will build the frontend into the `frontend/build` directory. You may then deploy this directory to your webserver. And start the frontend with:

```bash
node build
```

### Backend

The backend is built as a Rust application:

```bash
cd backend
cargo build --release
```

This will build the backend into the `backend/target/release` directory. You may then deploy the resulting binary `backend/target/release/sveltekit-rust-backend` to your server. And start the backend with:

```bash
./svetlekit-rust-backend
```
