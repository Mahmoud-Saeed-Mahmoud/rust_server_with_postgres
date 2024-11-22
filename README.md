# Rust CRUD API with Actix-web and Sea-ORM

A modern, type-safe REST API built with Rust, featuring comprehensive CRUD operations for users, profiles, and posts. Created with [Windsurf](https://codeium.com/windsurf), the world's first agentic IDE.

## 🚀 Features

- **Modern Stack**: Built with Actix-web 4.x and Sea-ORM
- **Type Safety**: Leverages Rust's powerful type system
- **Database Integration**: PostgreSQL with Sea-ORM
- **Clean Architecture**: Repository pattern and modular design
- **RESTful API**: Full CRUD operations for all resources

## 📋 API Endpoints

### Users
```
GET    /api/users          # List all users
POST   /api/users          # Create a new user
GET    /api/users/{id}     # Get user by ID
PUT    /api/users/{id}     # Update user
DELETE /api/users/{id}     # Delete user
```

### Profiles
```
GET    /api/profiles          # List all profiles
POST   /api/profiles          # Create a new profile
GET    /api/profiles/{id}     # Get profile by ID
PUT    /api/profiles/{id}     # Update profile
DELETE /api/profiles/{id}     # Delete profile
```

### Posts
```
GET    /api/posts          # List all posts
POST   /api/posts          # Create a new post
GET    /api/posts/{id}     # Get post by ID
PUT    /api/posts/{id}     # Update post
DELETE /api/posts/{id}     # Delete post
```

## 🛠️ Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Cargo

## ⚙️ Configuration

1. Create a PostgreSQL database
2. Set your database URL in the environment:
```bash
export DATABASE_URL="postgres://user:password@localhost/dbname"
```

## 🚀 Getting Started

1. Clone the repository:
```bash
git clone <repository-url>
cd crud_from_prisma
```

2. Install dependencies:
```bash
cargo build
```

3. Run migrations:
```bash
cargo run --bin migration
```

4. Start the server:
```bash
cargo run
```

The server will start at `http://localhost:3333`

## 📦 Project Structure

```
src/
├── api/         # API handlers and routes
├── dto/         # Data Transfer Objects
├── entity/      # Database entities
├── repository/  # Database operations
└── server/      # Server configuration
```

## 🧪 Testing

Run the test suite:
```bash
cargo test
```

## 📚 Documentation

Generate and view the documentation:
```bash
cargo doc --open
```

## 🔧 Development

### Building
```bash
cargo build
```

### Running in development mode
```bash
cargo run
```

### Running in release mode
```bash
cargo run --release
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## ✨ Acknowledgments

- [Windsurf](https://github.com/codeium/windsurf) - The world's first agentic IDE
- [Actix-web](https://actix.rs/)
- [Sea-ORM](https://www.sea-ql.org/SeaORM/)
- [PostgreSQL](https://www.postgresql.org/)
