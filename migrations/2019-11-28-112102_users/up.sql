-- Your SQL goes here
CREATE TABLE "users" (
	"id"    INTEGER NOT NULL UNIQUE,
	"name"  TEXT NOT NULL,
	"email" TEXT NOT NULL,
	"role"  TEXT NOT NULL,
	PRIMARY KEY("id")
)
