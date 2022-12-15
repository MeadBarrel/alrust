-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "ingredients" (
	"name"	TEXT NOT NULL,
	"lore"	TEXT NOT NULL,
	"al_weight"	INTEGER NOT NULL,
	"dh"	DECIMAL(12, 2),
	"dp"	DECIMAL(12, 2),
	"mdh"	DECIMAL(12, 2),
	"mdp"	DECIMAL(12, 2),
	"hot"	DECIMAL(12, 2),
	"pot"	DECIMAL(12, 2),
	"mhot"	DECIMAL(12, 2),
	"mpot"	DECIMAL(12, 2),
	"hl"	DECIMAL(12, 2),
	"pl"	DECIMAL(12, 2),
	"mhl"	DECIMAL(12, 2),
	"mpl"	DECIMAL(12, 2),
	"a"	DECIMAL(12, 2),
	"ma"	DECIMAL(12, 2),
	"notes"	TEXT,
	PRIMARY KEY("name")
);
CREATE TABLE IF NOT EXISTS "lores" (
	"name"	TEXT NOT NULL,
	"effectiveness"	DECIMAL(12, 6),
	"parent"	TEXT,
	PRIMARY KEY("name")
);
CREATE TABLE IF NOT EXISTS "player_characters" (
	"name"	TEXT NOT NULL,
	"advanced_potion_making"	INTEGER NOT NULL DEFAULT 0,
	"alvarin_clade"	BOOLEAN NOT NULL DEFAULT 0,
	PRIMARY KEY("name")
);
CREATE TABLE IF NOT EXISTS "player_character_lores" (
	"character"	TEXT NOT NULL,
	"lore"	TEXT NOT NULL,
	"value"	INTEGER NOT NULL DEFAULT 0,
	FOREIGN KEY("character") REFERENCES "player_characters",
	FOREIGN KEY("lore") REFERENCES "lores",
	PRIMARY KEY("character", "lore")
);
