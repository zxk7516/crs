-- Your SQL goes here
CREATE table "users" (
	"id" serial4,
	"username" varchar(40),
	"email" varchar(200),
	"password" varchar(200),
	"created_at" timestamptz(6) not null default now(),
	"updated_at" timestamptz(6) not null default now(),
	constraint "PK" primary key ("id"),
	constraint "UNemail" unique ("email"),
	constraint "UNusername" unique ("username")
);