#!/bin/bash

echo "Setting up Poll App..."

# Check if required environment variables are set
check_env_vars() {
    local missing_vars=()
    
    if [ -z "$DATABASE_URL" ]; then
        missing_vars+=("DATABASE_URL")
    fi
    
    if [ -z "$JWT_SECRET" ]; then
        missing_vars+=("JWT_SECRET")
    fi
    
    if [ ${#missing_vars[@]} -ne 0 ]; then
        echo "Error: The following environment variables are required but not set:"
        printf '  - %s\n' "${missing_vars[@]}"
        echo ""
        echo "Please set them before running this script:"
        echo "  export DATABASE_URL='postgresql://username:password@localhost:5432/poll_app'"
        echo "  export JWT_SECRET='your-super-secret-jwt-key-change-me-in-production'"
        echo ""
        echo "Example:"
        echo "  export DATABASE_URL='postgresql://postgres:mypassword@localhost:5432/poll_app'"
        echo "  export JWT_SECRET='my-super-secret-key-123'"
        exit 1
    fi
}

# Check environment variables
check_env_vars

echo "✓ Environment variables are set"

# Install Rust dependencies
echo "Installing Rust dependencies..."
if cargo build --quiet; then
    echo "✓ Rust dependencies installed"
else
    echo "✗ Failed to install Rust dependencies"
    exit 1
fi

# Install frontend dependencies
echo "Installing frontend dependencies..."
cd frontend
if npm install --silent; then
    echo "✓ Frontend dependencies installed"
else
    echo "✗ Failed to install frontend dependencies"
    exit 1
fi
cd ..

# Test database connection
echo "Testing database connection..."
if sqlx migrate info > /dev/null 2>&1; then
    echo "✓ Database connection successful"
else
    echo "✗ Database connection failed"
    echo "Please ensure:"
    echo "  1. PostgreSQL is running"
    echo "  2. Database exists: createdb poll_app"
    echo "  3. DATABASE_URL is correct"
    exit 1
fi

# Run migrations
echo "Running database migrations..."
if sqlx migrate run; then
    echo "✓ Database migrations applied"
else
    echo "✗ Failed to run migrations"
    exit 1
fi

echo ""
echo "🎉 Setup complete!"
echo ""
echo "Next steps:"
echo "1. Start the backend: cargo run"
echo "2. Start the frontend: cd frontend && npm start"
echo ""
echo "The application will be available at:"
echo "  - Frontend: http://localhost:3000"
echo "  - Backend API: http://localhost:8000"
