DROP TABLE IF EXISTS posts;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS posts_tags;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS likes;
DROP TABLE IF EXISTS images;
DROP TABLE IF EXISTS posts_images;
DROP TABLE IF EXISTS bans;
DROP TABLE IF EXISTS uploads;

CREATE TABLE posts(
	post_id INTEGER PRIMARY KEY NOT NULL,
	user_id INTEGER NOT NULL,
	date BIGINT NOT NULL,
	body VARCHAR(2048) NOT NULL,
	likes INTEGER NOT NULL
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
	pfp_id INTEGER
);

CREATE TABLE likes(
	user_id INTEGER NOT NULL,
	post_id INTEGER NOT NULL
);

CREATE TABLE posts_images(
	post_id INTEGER NOT NULL,
	image_id INTEGER NOT NULL
);

CREATE TABLE images(
	image_id INTEGER PRIMARY KEY NOT NULL,
	image_file VARCHAR(64) NOT NULL
);

CREATE TABLE bans(
	user_id INTEGER NOT NULL,
	given_on INTEGER NOT NULL,
	expires_on INTEGER NOT NULL,
	ban_message VARCHAR(2048) NOT NULL,
	is_active INTEGER NOT NULL
);

CREATE TABLE uploads(
	user_id INTEGER NOT NULL,
	weight SHORT NOT NULL,
	date BIGINT NOT NULL
);

--                                  -----------------
--                                  |     bans      |
--                                  -----------------
--                          |- many |    user_id    |
--                          |       |   given_on    |
--                          |       |  expires_on   |
--   ----------------       |       |  ban_message  |         -----------           --------------            ____________
--   |    users     |       |       |   is_active   |         |  posts  |           | posts_tags |            |   tags   |
--   ----------------       |       -----------------         -----------           --------------            ------------
--   |   user_id    | 1 ----|                         |---- 1 | post_id | 1 -- many |   post_id  |       |- 1 |  tag_id  |
--   |  user_name   |       |-------------------------+- many | user_id |   |       |   tag_id   | many -|    | tag_name |
--   | display_name |       |                         |       |  date   |   |       --------------            ------------
--   | description  |       |       -----------       |       |  body   |   |
--   |    passwd    |       |       |  likes  |       |       |  likes  |   |       ----------------            --------------
--   |   is_admin   |       |       -----------       |       -----------   |       | posts_images |            |   images   |
--   |    pfp_id    | 1 -*A |- many | user_id |       |                     |       ----------------    A*-|    --------------
--   ----------------       |       | post_id | many -|                     |- many |   post_id    |       |- 1 |  image_id  |
--                          |       -----------                                     |   image_id   | many -|    | image_file |
--                          |                                                       ----------------            --------------
--                          |       -----------
--                          |       | uploads |        
--                          |       -----------
--                          |- many | user_id |
--                                  | weight  |
--                                  |  date   |
--                                  -----------       
