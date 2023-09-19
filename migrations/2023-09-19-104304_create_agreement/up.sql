-- Your SQL goes here
CREATE TABLE agreement(
  agreement_id SERIAL PRIMARY KEY,
  agreement_data text NOT NULL,
  agreement_type varchar(20) NOT NULL
);