# Poll App

A full-stack polling application built with Rust (Actix-web) backend and React frontend.

## Features

- Create and manage polls
- Vote on polls
- View poll statistics
- User authentication with JWT
- Real-time poll results

## Prerequisites

- Rust (latest stable version)
- Node.js (v16 or higher)
- PostgreSQL
- npm or yarn

## Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd Poll_app
   ```

2. **Set environment variables**
   ```bash
   export DATABASE_URL="postgresql://username:password@localhost:5432/poll_app"
   export JWT_SECRET="your-super-secret-jwt-key-change-me-in-production"
   ```
   
   Example:
   ```bash
   export DATABASE_URL="postgresql://postgres:mypassword@localhost:5432/poll_app"
   export JWT_SECRET="my-super-secret-key-123"
   ```

3. **Create the database**
   ```bash
   createdb poll_app
   ```

4. **Run the setup script**
   ```bash
   ./setup.sh
   ```

## Running the Application

### Backend (Rust)
```bash
cargo run
```
The backend will start on `http://localhost:8000`

### Frontend (React)
```bash
cd frontend
npm start
```
The frontend will start on `http://localhost:3000`

## API Endpoints

- `POST /login` - User login
- `POST /register` - User registration
- `GET /polls` - Get all polls
- `POST /poll` - Create a new poll
- `GET /poll/{id}` - Get a specific poll
- `PUT /poll/{id}` - Update a poll
- `DELETE /poll/{id}` - Delete a poll
- `POST /poll/{id}/vote` - Vote on a poll
- `GET /poll/{id}/stats` - Get poll statistics
- `GET /polls/expiring` - Get expiring polls

## Database Schema

### Users Table
```sql
CREATE TABLE "user"(
  name TEXT NOT NULL,
  password TEXT NOT NULL
);
```

### Polls Table
```sql
CREATE TABLE poll (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    options TEXT[] NOT NULL,
    votes INTEGER[] NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## Troubleshooting

### Common Issues

1. **JSON Parse Error**: This usually means the backend is not running or there's a CORS issue. Make sure:
   - Backend is running on port 8000
   - Frontend is making requests to `http://localhost:8000`
   - CORS is properly configured

2. **Database Connection Error**: Check your environment variables and ensure:
   - PostgreSQL is running
   - Database credentials are correct
   - Database `poll_app` exists
   - Environment variables are set: `echo $DATABASE_URL`

3. **Migration Errors**: Run `sqlx migrate run` to apply database migrations

4. **Environment Variables Not Set**: Make sure to export the required variables:
   ```bash
   export DATABASE_URL="postgresql://username:password@localhost:5432/poll_app"
   export JWT_SECRET="your-secret-key"
   ```

## Development

### Adding New Features
1. Add backend endpoints in `src/models/`
2. Update frontend components in `frontend/src/components/`
3. Test API endpoints
4. Update documentation

### Project Structure
```
Poll_app/
├── src/                    # Rust backend source
│   ├── main.rs            # Main application entry
│   └── models/            # API models and handlers
├── frontend/              # React frontend
│   ├── src/
│   │   ├── components/    # React components
│   │   └── App.js         # Main app component
│   └── package.json
├── migrations/            # Database migrations
└── Cargo.toml            # Rust dependencies
```
