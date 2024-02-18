CREATE TABLE guild (
  id BIGINT PRIMARY KEY,
  mod_log_channel BIGINT,
  message_log_channel BIGINT
);

CREATE TABLE guild_logged_channel (
  guild_id BIGINT,
  channel_id,
  PRIMARY KEY (guild_id, channel_id),
  FOREIGN KEY (guild_id) REFERENCES guild(id) ON DELETE CASCADE
);
