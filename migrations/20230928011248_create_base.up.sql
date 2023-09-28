-- Add up migration script here
CREATE TABLE IF NOT EXISTS users
(
    id       serial PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
		email VARCHAR ( 255 ) UNIQUE NOT NULL,
		"state" integer DEFAULT 1 NOT NULL,
		created_at timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
		updated_at timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
		login_at TIMESTAMP
);

CREATE INDEX "idx-users-username" ON "users" (username);