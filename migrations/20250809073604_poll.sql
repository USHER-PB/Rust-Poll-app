-- Add migration script here
CREATE TABLE poll (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    options TEXT[] NOT NULL,
    votes INTEGER[] NOT NULL,  -- Changed from TEXT[] to INTEGER[]
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
