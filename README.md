# Meditation App - Rust Learning Project

## About
This project is part of my Rust 0-100 learning journey - building real applications to master Rust concepts from beginner to advanced.

## Learning Focus
- Ownership & Borrowing
- Async Programming with Tokio
- Web Development with Rocket
- Database Integration with SQLx
- JSON handling with Serde
- Template rendering with Tera

## Tech Stack
- **Rocket** - Web framework
- **SQLx** - Database with compile-time safety
- **Serde** - JSON serialization
- **Tera** - HTML templates
- **SQLite** - Database

## Usage

### Setup
```bash
git clone <your-repo-url>
cd meditation_app
cargo run
```

Server runs at `http://localhost:8000`

### API Endpoints
- **POST** `/greeting` - Create meditation message
- **GET** `/greetings` - Get all messages (JSON)
- **GET** `/greetings/view` - Web interface (HTML)
- **GET** `/hello/<name>` - Personalized greeting

### Example
```bash
# Create message
curl -X POST http://localhost:8000/greeting \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice","message":"Breathe deeply"}'

# View all messages
curl http://localhost:8000/greetings
```

## Learning Journey
Part of a structured Rust learning path:
- Phase 1: Language fundamentals ✓
- Phase 2: Web development (this project) ✓
- Phase 3: Systems programming (next)
- Phase 4: Advanced patterns (planned)

## License

MIT License

Copyright (c) 2024


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

