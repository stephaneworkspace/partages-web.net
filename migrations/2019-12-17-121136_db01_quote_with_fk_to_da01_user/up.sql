-- Your SQL goes here
CREATE TABLE db01_quote (
  id SERIAL PRIMARY KEY,
  da01_user_id INTEGER NOT NULL,
  author VARCHAR NOT NULL,
  author_activity VARCHAR NOT NULL,
  quote TEXT NOT NULL,
  sw_published BOOLEAN NOT NULL DEFAULT 'f'
);
ALTER TABLE db01_quote ADD CONSTRAINT db01_quote_da01_user_id_fkey FOREIGN KEY (da01_user_id) REFERENCES da01_user;
