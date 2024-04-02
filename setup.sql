DROP TABLE IF EXISTS posts;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS posts_tags;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS reactions;

CREATE TABLE posts(
	post_id INTEGER PRIMARY KEY NOT NULL,
	user_id INTEGER NOT NULL,
	date BIGINT NOT NULL,
	body VARCHAR(2048) NOT NULL
);

CREATE TABLE posts_tags(
	post_id INTEGER NOT NULL,
	tag_id INTEGER NOT NULL
);

CREATE TABLE tags(
	tag_id INTEGER PRIMARY KEY NOT NULL,
	tag_name VARCHAR(64) UNIQUE NOT NULL
);

CREATE TABLE users(
	user_id INTEGER PRIMARY KEY NOT NULL,
	user_name VARCHAR(64) UNIQUE NOT NULL,
	display_name VARCHAR(64) NOT NULL,
	description VARCHAR(2048) NOT NULL,
	passwd VARCHAR(128) NOT NULL,
	is_admin INTEGER NOT NULL,
	is_banned INTEGER NOT NULL
);

CREATE TABLE reactions(
	type INTEGER NOT NULL,
	user_id INTEGER NOT NULL,
	post_id INTEGER NOT NULL
);

--   -------------                                           -----------          ----------------            ____________
--   |   users   |                                           |  posts  |          |  posts_tags  |            |   tags   |
--   -------------                                           -----------          ----------------            ------------
--   |  user_id  | 1 -|                           |------- 1 | post_id | 1 - many |    post_id   |       |- 1 |  tag_id  |
--   | user_name |    |--------------------------- ---- many | user_id |          |    tag_id    | many -|    | tag_name |
--   |  passwd   |    |                           |          |  date   |          ----------------            ------------
--   | is_admin  |    |                           |          |  body   |  
--   | is_banned |    |                           |          -----------  
--   -------------    |                           |
--                    |       -------------       |
--                    |       | reactions |       |
--                    |       -------------       |
--                    |       |   type    |       |
--                    |- many |  user_id  |       | 
--                            |  post_id  | many -|
--                            -------------

INSERT INTO users VALUES (0, 'root', 'root', 'hala madrid', 'toor', 1, 0);
