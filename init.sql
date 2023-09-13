CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL UNIQUE,
    CONSTRAINT email_unique UNIQUE (email)
);

CREATE INDEX idx_users_email ON users(email);