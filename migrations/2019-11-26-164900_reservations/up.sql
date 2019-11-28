CREATE TABLE "booking" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	TEXT NOT NULL,
	"description"	TEXT NOT NULL,
	"author"		TEXT NOT NULL,
	"rooms"	INTEGER NOT NULL,
	"begin_time"	TEXT NOT NULL,
	"end_time"	TEXT NOT NULL,
	"layout"	INTEGER NOT NULL,
	"approved"	INTEGER NOT NULL,
	PRIMARY KEY("id")
);
