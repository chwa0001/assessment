# Rust Backend with React Frontend

## Backend (Rust - Actix Web)
The backend is built using Rust with the Actix Web framework. It serves as a lightweight HTTP server that serves the static build files generated from the React frontend.

- The Rust backend is located in the `backend/` directory.
- Static files from the React frontend build are stored in `backend/build/` and served by the backend.
- The backend routes are defined in `backend/src/routes/`.

### Running the Backend
Ensure Rust and Cargo are installed, then run:

```sh
cd backend
cargo run
```

## Frontend (React + Vite)
The frontend is a React application built with Vite. After building the project, the static files are placed in `backend/build/` for the Rust server to serve.

### Building the Frontend
Ensure Node.js is installed, then run:

```sh
cd frontend
npm install
npm run build
```

This generates a `dist/` folder, which should be moved to `backend/build/`.

## Live Demo
View the deployed portfolio on Vercel:  
[**Live Portfolio**](https://your-vercel-deployment-url.vercel.app/)
