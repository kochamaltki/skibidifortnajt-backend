DROP TABLE IF EXISTS posts;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS posts_tags;
DROP TABLE IF EXISTS tags;

CREATE TABLE posts(
	post_id INTEGER PRIMARY KEY NOT NULL,
	user_id INTEGER NOT NULL,
	date BIGINT NOT NULL,
	body VARCHAR(2048) NOT NULL
);

CREATE TABLE posts_tags(
	post_id INTEGER,
	tag_id INTEGER
);

CREATE TABLE tags(
	tag_id INTEGER PRIMARY KEY NOT NULL,
	tag_name VARCHAR(64) UNIQUE NOT NULL
);

CREATE TABLE users(
	user_id INTEGER PRIMARY KEY NOT NULL,
	user_name VARCHAR(64) UNIQUE NOT NULL,
	passwd VARCHAR(128) NOT NULL,
	is_admin INTEGER NOT NULL,
	is_banned INTEGER NOT NULL
);

--   -------------            -----------          ----------------            ____________
--   |   users   |            |  posts  |          |  posts_tags  |            |   tags   |
--   -------------            -----------          ----------------            ------------
--   |  user_id  | 1 -|       | post_id | 1 - many |    post_id   |       |- 1 |  tag_id  |
--   | user_name |    |- many | user_id |          |    tag_id    | many -|    | tag_name |
--   |  passwd   |            |  date   |          ----------------            ------------
--   | is_admin  |            |  body   |  
--   | is_banned |            -----------  
--   -------------

INSERT INTO users VALUES (0, 'root', 'toor', 1, 0);
