CREATE TABLE user (
  id BIGINT PRIMARY KEY NOT NULL,
  commands_run BIGINT NOT NULL DEFAULT 0,
  acquaint_fate INT NOT NULL DEFAULT 0,
  interwined_fate INT NOT NULL DEFAULT 0,
  primogems INT NOT NULL DEFAULT 0,
  standard_pity INT NOT NULL DEFAULT 0,
  weapon_pity INT NOT NULL DEFAULT 0,
  character_pity INT NOT NULL DEFAULT 0
);

CREATE TABLE guild (
  id BIGINT PRIMARY KEY NOT NULL,
  mod_log_channel BIGINT,
  message_log_channel BIGINT,
  owner BIGINT NOT NULL,
  commands_ran BIGINT NOT NULL,
  songs_played INT NOT NULL,
  mute_role BIGINT,
  mute_style TEXT NOT NULL DEFAULT "timeout",
  mute_duration BIGINT NOT NULL DEFAULT 3600,
  prefix TEXT NOT NULL DEFAULT '-',
  FOREIGN KEY (owner) REFERENCES user(id)
);

CREATE TABLE guild_logged_channel (
  guild_id BIGINT,
  channel_id,
  PRIMARY KEY (guild_id, channel_id),
  FOREIGN KEY (guild_id) REFERENCES guild(id) ON DELETE CASCADE
);

CREATE TABLE user_guild (
  user_id BIGINT,
  guild_id BIGINT,
  join_date TEXT NOT NULL,
  first_join_date TEXT NOT NULL,
  infractions INT NOT NULL DEFAULT 0,
  PRIMARY KEY (user_id, guild_id),
  FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE,
  FOREIGN KEY (guild_id) REFERENCES guild(id) ON DELETE CASCADE
);

CREATE TABLE item (
  id INT PRIMARY KEY,
  name TEXT NOT NULL,
  rarity INT CHECK(rarity BETWEEN 3 AND 5),
  is_event INT CHECK(is_event = 0 OR is_event = 1),
  type INT CHECK(type = 0 OR type = 1), -- 0 character, 1 weapon
  release_update INT
);

CREATE TABLE wish (
  id INT PRIMARY KEY,
  name TEXT NOT NULL,
  type INT CHECK(type IN (1, 2, 3)) -- 0 standard, 1 weapon, 2 character
);

CREATE TABLE wish_item (
  wish_id INT,
  item_id INT,
  PRIMARY KEY (wish_id, item_id),
  FOREIGN KEY (wish_id) REFERENCES wish(id),
  FOREIGN KEY (item_id) REFERENCES item(id)
);
