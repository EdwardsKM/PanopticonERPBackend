
-- This schema will be used to manage users
create schema internal;

CREATE TABLE internal.users (
	email VARCHAR(200) NOT NULL PRIMARY KEY,
	name VARCHAR(200) NOT NULL,
	username    VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(50),
	UNIQUE (username)
);

CREATE TABLE internal.sessions (
	"user" VARCHAR(200) PRIMARY KEY,
	"token" VARCHAR(200) NOT NULL,
	created TIMESTAMP DEFAULT NOW(),
	CONSTRAINT fk_user_sessions
	FOREIGN KEY("user")
	REFERENCES testing.users(email)
	ON DELETE CASCADE
);
