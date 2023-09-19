-- Your SQL goes here
CREATE TABLE users(
  user_id SERIAL PRIMARY KEY,
  first_name varchar(50) NOT NULL,
  last_name varchar(50) NOT NULL,
  user_type varchar(20) NOT NULL,
  user_email varchar(50) NOT NULL
);