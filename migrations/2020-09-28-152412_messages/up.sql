-- Your SQL goes here
CREATE TABLE messages (
  id SERIAL PRIMARY KEY,
  contents TEXT NOT NULL
--   published BOOLEAN NOT NULL DEFAULT 'f'
)