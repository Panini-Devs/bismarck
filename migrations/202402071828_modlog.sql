-- modlog schema
CREATE TABLE IF NOT EXISTS mod_log (
    uuid VARCHAR(32) UNIQUE NOT NULL,
    guild_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    moderator_id BIGINT NOT NULL,
    action_type TEXT NOT NULL,
    action_duration INTEGER,
    reason TEXT NOT NULL DEFAULT "No reason provided",
    time_created TIMESTAMP NOT NULL,
    PRIMARY KEY (uuid),
    FOREIGN KEY (guild_id) REFERENCES guild_settings(guild_id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES user_profile(user_id),
    FOREIGN KEY (moderator_id) REFERENCES user_profile(user_id)
);