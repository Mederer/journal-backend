-- DDL

DROP TABLE IF EXISTS "entry";
DROP TABLE IF EXISTS "user";

CREATE TABLE "user" (
  id SERIAL PRIMARY KEY,
  email TEXT NOT NULL,
  firstname TEXT NOT NULL,
  lastname TEXT NOT NULL,
  secret TEXT NOT NULL
);


CREATE TABLE "entry" (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  body TEXT NOT NULL,
  user_id INT NOT NULL,
  FOREIGN KEY(user_id) REFERENCES "user"(id)
);

-- Test Data

INSERT INTO "user" (firstname, lastname, email, secret) VALUES ('Mitchell', 'Dederer', 'mederer@me.com', 'password');
INSERT INTO "entry" (title, body, user_id) VALUES ('Hello world!', 'My first post', 1);
INSERT INTO "entry" (title, body, user_id) VALUES ('Hello world!', 'My first post', 1);
INSERT INTO "entry" (title, body, user_id) VALUES ('Hello world!', 'My first post', 1);
INSERT INTO "entry" (title, body, user_id) VALUES ('Hello world!', 'My first post', 1);
INSERT INTO "entry" (title, body, user_id) VALUES ('Hello world!', 'My first post', 1);
INSERT INTO "entry" (title, body, user_id) VALUES ('Hello world!', 'My first post', 1);
INSERT INTO "entry" (title, body, user_id) VALUES ('Hello world!', 'My first post', 1);
INSERT INTO "entry" (title, body, user_id) VALUES ('Hello world!', 'My first post', 1);
INSERT INTO "entry" (title, body, user_id) VALUES ('Hello world!', 'My first post', 1);
INSERT INTO "entry" (title, body, user_id) VALUES ('Hello world!', 'My first post', 1);