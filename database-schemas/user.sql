CREATE TABLE user (
  id BIGINT PRIMARY KEY,
  commands_run INT NOT NULL DEFAULT 0,
  acquaint_fate INT NOT NULL DEFAULT 0,
  interwined_fate INT NOT NULL DEFAULT 0,
  primogems INT NOT NULL DEFAULT 0,
  standard_pity INT NOT NULL DEFAULT 0,
  weapon_pity INT NOT NULL DEFAULT 0,
  character_pity INT NOT NULL DEFAULT 0
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
