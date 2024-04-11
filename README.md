# Gradekeeper API Server 
The Rust and Axum-based API server for [Gradekeeper](github.com/jacksonrakena/gradekeeper), a free, open-source web app that simplifies grade tracking and projections for university students across the globe.  

## About
Gradekeeper Nova (the codename for this project) was written in 2023 to replace the legacy Next.js—PlanetScale (MySQL)—based system that had served Gradekeeper well for years. Planetscale deprecated their free tier in early 2024, and before that was announced I had already written most of the code in this project so I could migrate it to self-managed PostgreSQL.  

At the same time, I took this opportunity to significantly improve the database architecture, adding more constraints, changing data types, and optimising some database behaviour. The migration was messy, involving 510 users, 596 trimesters, 1,309 courses, 6,739 course components, and 12,214 course subcomponents.  

The end result is a much faster, leaner, and type-safe API server that's far easier for me to maintain, and a significantly trimmed down and more lightweight React/Vite—based frontend that yields significant improvements in frontend experience.

## Architecture
- [Diesel ORM](https://diesel.rs/) for accessing and managing database objects
  - Fairly typical PostgreSQL setup for storing data
- Uses Google OAuth2 tokens for session management (implemented in [middleware/auth.rs](https://github.com/jacksonrakena/gradekeeper-server/blob/main/src/middleware/auth.rs) and routes [api/auth/callback](https://github.com/jacksonrakena/gradekeeper-server/blob/main/src/routes/api/auth/callback.rs) and [api/auth/login](https://github.com/jacksonrakena/gradekeeper-server/blob/main/src/routes/api/auth/login.rs))
- Uses Axum for HTTP routing, with Axum `Extension<Arc<T>>` to pass around authorised user state from middleware into routes
  - File-based routing convention, see [src/routes/api](https://github.com/jacksonrakena/gradekeeper-server/tree/main/src/routes/api)

## Copyright
&copy; 2022-2024 Jackson Rakena
