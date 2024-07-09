# Gradekeeper API Server 
The Rust and Axum-based API server for [Gradekeeper](github.com/jacksonrakena/gradekeeper), a free, open-source web app that simplifies grade tracking and projections for university students across the globe.  

If you're looking for the React/TypeScript-based client, check out the [gradekeeper](https://github.com/jacksonrakena/gradekeeper) repository.

## About
Gradekeeper Nova (the codename for this project) was written in 2023 to replace the legacy Next.js—PlanetScale (MySQL)—based system that had served Gradekeeper well for years. Planetscale deprecated their free tier in early 2024, and before that was announced I had already written most of the code in this project so I could migrate it to self-managed PostgreSQL.

At the same time, I took this opportunity to significantly improve the database architecture, adding more constraints, changing data types, and optimising some database behaviour. The migration was messy, involving 510 users, 596 trimesters, 1,309 courses, 6,739 course components, and 12,214 course subcomponents.

The end result is a much faster, leaner, and type-safe API server that's far easier for me to maintain, and a significantly trimmed down and more lightweight React/Vite—based frontend that yields significant improvements in frontend experience.

## Deployment
Follow the instructions in the [Gradekeeper client repo](https://github.com/jacksonrakena/gradekeeper) to configure and deploy the server.

## Project layout
### Routes (`/routes`)
- Authentication service
  - `/api/auth/login` - handles incoming login requests and redirects to Google OAuth2 service
  - `/api/auth/callback`  
  Handles the Google callback, verifies the token, establishes the session, and sends it back to the frontend
- User route
  - `/api/users/me` - returns all user data, including components, subcomponents, courses, and blocks
- Block route
  - `/api/block/*`  
  All routes for updating and retrieving all entities
### Middleware
**Authentication middleware** (`/middleware/auth.rs`)  
Provides a few key authentication-related functions

- `check_authorization` is injected in every request. It checks and decodes the Authorization header,
ensures it is valid, and then injects the `Arc<Session>` middleware so route handlers are able to use the user's 
session information
- `validate_ownership_of_route_assets` is called on every protected route (to any entities), and ensures that the user is valid,
and that the user actually owns any assets they are trying to access.  
This middleware uses the `RouteAssetIdentifiers` struct with optional bound fields, so it will automatically bind to any matching route identifiers, and ensure
that they are protected.
### Other
- `errors.rs` provides the `AppResult<R>` (`Result<R, AppError>`) and `AppError` struct, which provide structured error responses to API requests
- `models.rs` defines the Diesel structs, which also serve as API response objects (this is a useful file to consult for implementing API clients)

## Architecture
- [Diesel ORM](https://diesel.rs/) for accessing and managing database objects
  - Fairly typical PostgreSQL setup for storing data
- Uses Google OAuth2 tokens for session management (implemented in [middleware/auth.rs](https://github.com/jacksonrakena/gradekeeper-server/blob/main/src/middleware/auth.rs) and routes [api/auth/callback](https://github.com/jacksonrakena/gradekeeper-server/blob/main/src/routes/api/auth/callback.rs) and [api/auth/login](https://github.com/jacksonrakena/gradekeeper-server/blob/main/src/routes/api/auth/login.rs))
- Uses Axum for HTTP routing, with Axum `Extension<Arc<T>>` to pass around authorised user state from middleware into routes
  - File-based routing convention, see [src/routes/api](https://github.com/jacksonrakena/gradekeeper-server/tree/main/src/routes/api)

## Copyright
&copy; 2022-2024 Jackson Rakena
