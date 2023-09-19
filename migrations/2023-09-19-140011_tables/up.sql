-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE "lender" (
  "id" integer PRIMARY KEY,
  "org_name" varchar(255) NOT NULL,
  "email" varchar(255) NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT NOW()
);

CREATE TABLE "users" (
  "id" integer PRIMARY KEY,
  "first_name" varchar(255) NOT NULL,
  "last_name" varchar(255) NOT NULL,
  "email" varchar(255) NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT NOW()
);

CREATE TABLE "document" (
  "id" integer PRIMARY KEY,
  "lender_id" integer NOT NULL,
  "document_type" varchar(50) NOT NULL,
  "document_data" text NOT NULL,
  "updated_at" timestamp NOT NULL DEFAULT NOW()
);

CREATE TABLE "deal" (
  "id" integer PRIMARY KEY,
  "lender_id" integer NOT NULL,
  "user_id" integer NOT NULL,
  "document_id" integer NOT NULL,
  "status" varchar(50) NOT NULL,
  "updated_at" timestamp NOT NULL DEFAULT NOW()
);

COMMENT ON COLUMN "document"."document_data" IS 'Content of the document';

ALTER TABLE "document" ADD FOREIGN KEY ("lender_id") REFERENCES "lender" ("id");

ALTER TABLE "deal" ADD FOREIGN KEY ("lender_id") REFERENCES "lender" ("id");

ALTER TABLE "deal" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "deal" ADD FOREIGN KEY ("document_id") REFERENCES "document" ("id");
