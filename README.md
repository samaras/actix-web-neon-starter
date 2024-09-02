# Actix-Web Blog Starter Kit

This is a starter kit for a simple blog API built with Actix-web and Neon using Diesel as the ORM. The application includes basic authentication and internationalization (i18n).

## Features
- **User Authentication**: Register and login with JWT tokens.
- **CRUD for Posts**: Create, read, update, and delete blog posts.
- **Internationalization (i18n)**: Multi-language support out of the box.
- **Extendable**: Clear separation of concerns makes it easy to extend and scale the application.

## Crates Used
- `actix-web`: Web framework for Rust.
- `diesel`: ORM and query builder for database interactions.
- `jsonwebtoken`: For handling JWT-based authentication.
- `argon2`: For password hashing.
- `i18n-embed`: For localization and internationalization.
- `serde`: For serializing/deserializing data.
- `dotenv`: For environment variable management.

## Setup Instructions

1. **Clone the Repository**
   ```bash
   git clone https://github.com/samaras/actix-web-neon-starter
   cd actix-web-neon-starter

2. **Install Diesel CLI**

```bash
cargo install diesel_cli --no-default-features --features postgres
```
3. **Setup Environment varialbles**

Create .env file on the root folder and set the DATABASE_URL from neon, and your JWT_SECRET.
```env
DATABASE_URL=postgres://user:password@localhost/blog_db
JWT_SECRET=your_jwt_secret
```
4. **Run migrations with diesel**

```bash
diesel migration run
``` 
5. **Run the server**
```bash
cargo run
```

