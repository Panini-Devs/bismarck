CREATE IF NOT EXISTS TABLE user (
  id BIGINT PRIMARY KEY NOT NULL,
  commands_run BIGINT NOT NULL DEFAULT 0,
  acquaint_fate INT NOT NULL DEFAULT 0,
  interwined_fate INT NOT NULL DEFAULT 0,
  primogems INT NOT NULL DEFAULT 0,
  standard_pity INT NOT NULL DEFAULT 0,
  weapon_pity INT NOT NULL DEFAULT 0,
  character_pity INT NOT NULL DEFAULT 0
);

CREATE IF NOT EXISTS TABLE guild (
  id BIGINT PRIMARY KEY NOT NULL,
  mod_log_channel BIGINT,
  message_log_channel BIGINT,
  guild_owner BIGINT NOT NULL,
  commands_ran BIGINT NOT NULL,
  songs_played INT NOT NULL,
  mute_role BIGINT,
  mute_style TEXT NOT NULL DEFAULT "timeout",
  mute_duration BIGINT NOT NULL DEFAULT 3600,
  prefix TEXT NOT NULL DEFAULT '-',
  FOREIGN KEY (owner) REFERENCES user(id)
);

CREATE IF NOT EXISTS TABLE guild_logged_channel (
  guild_id BIGINT,
  channel_id,
  PRIMARY KEY (guild_id, channel_id),
  FOREIGN KEY (guild_id) REFERENCES guild(id) ON DELETE CASCADE
);

CREATE IF NOT EXISTS TABLE user_guild (
  user_id BIGINT NOT NULL,
  guild_id BIGINT NOT NULL,
  join_date TEXT NOT NULL,
  infractions INT NOT NULL DEFAULT 0,
  PRIMARY KEY (user_id, guild_id),
  FOREIGN KEY (guild_id) REFERENCES guild(id) ON DELETE CASCADE
);

CREATE IF NOT EXISTS TABLE guild_log (
  uuid TEXT NOT NULL,
  guild_id BIGINT,
  user_id BIGINT,
  moderator_id BIGINT,
  action_type TEXT NOT NULL,
  reason TEXT NOT NULL,
  time_created TIMESTAMP NOT NULL,
  PRIMARY KEY (uuid),
  FOREIGN KEY (guild_id) REFERENCES guild(id) ON DELETE CASCADE,
  FOREIGN KEY (moderator_id) REFERENCES user(id)
);

CREATE IF NOT EXISTS TABLE item (
  id INT PRIMARY KEY,
  item_name TEXT NOT NULL,
  rarity INT CHECK(rarity BETWEEN 3 AND 5),
  is_event INT CHECK(is_event = 0 OR is_event = 1),
  item_type INT CHECK(type = 0 OR type = 1), -- 0 character, 1 weapon
  release_update INT
);

CREATE IF NOT EXISTS TABLE wish_odds (
  wish_type INT PRIMARY KEY,
  rarity5_odds REAL NOT NULL,
  rarity4_odds REAL NOT NULL,
  rarity3_odds REAL NOT NULL,
  pity4_start INT NOT NULL,
  pity4_end INT NOT NULL,
  pity5_start INT NOT NULL,
  pity5_end INT NOT NULL
);

CREATE IF NOT EXISTS TABLE wish (
  id INT PRIMARY KEY,
  wish_name TEXT NOT NULL,
  wish_type INT,
  FOREIGN KEY (wish_type) REFERENCES wish_odds(wish_type)
);

CREATE IF NOT EXISTS TABLE wish_item (
  wish_id INT,
  item_id INT,
  PRIMARY KEY (wish_id, item_id),
  FOREIGN KEY (wish_id) REFERENCES wish(id),
  FOREIGN KEY (item_id) REFERENCES item(id)
);
