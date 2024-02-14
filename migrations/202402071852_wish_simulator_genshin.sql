-- Wish Simulator Schema

CREATE TABLE IF NOT EXISTS user (
  id BIGINT PRIMARY KEY,
  primogems INTEGER NOT NULL DEFAULT 0,
  interwined_fate INTEGER NOT NULL DEFAULT 0,
  acquaint_fate INTEGER NOT NULL DEFAULT 0,
  -- Used for pity
  standard_wishes_since_reset INTEGER NOT NULL DEFAULT 0,
  character_wishes_since_reset INTEGER NOT NULL DEFAULT 0,
  weapon_wishes_since_reset INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS item (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  -- The update where the item was added.
  -- Ideally it would be stored in a separate table since it's only used for the wanderlust invocation.
  -- For now, omitted.
  -- added_in TEXT,
  rarity TEXT CHECK(rarity IN ('3', '4', '5')) NOT NULL, -- Rarity regarding wishes
  availability TEXT CHECK(availability IN ('event', 'standard')) NOT NULL DEFAULT 'standard',
  kind TEXT CHECK(kind IN ('weapon', 'character')) NOT NULL
);

CREATE TABLE IF NOT EXISTS user_item (
  user BIGINT,
  item INTEGER,
  PRIMARY KEY (user, item),
  FOREIGN KEY (user) REFERENCES user(id),
  FOREIGN KEY (item) REFERENCES item(id)
);

CREATE TABLE IF NOT EXISTS wish (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  kind TEXT CHECK(kind IN ('weapon', 'character', 'standard')) NOT NULL
);

-- 3 star items
CREATE TABLE IF NOT EXISTS standard_pool (
  item INTEGER,
  wish INTEGER,
  PRIMARY KEY (item, wish),
  FOREIGN KEY (item) REFERENCES item(id),
  FOREIGN KEY (wish) REFERENCES wish(id)
);

-- 4 star featured items
CREATE TABLE IF NOT EXISTS feature_pool (
  item INTEGER,
  wish INTEGER,
  PRIMARY KEY (item, wish),
  FOREIGN KEY (item) REFERENCES item(id),
  FOREIGN KEY (wish) REFERENCES wish(id)
);

-- 5 star promotional items
CREATE TABLE IF NOT EXISTS promotional_pool (
  item INTEGER,
  wish INTEGER,
  PRIMARY KEY (item, wish),
  FOREIGN KEY (item) REFERENCES item(id),
  FOREIGN KEY (wish) REFERENCES wish(id)
);

CREATE VIEW IF NOT EXISTS wanderlust_itempool
AS
SELECT
  id
FROM
  item
WHERE
  availability = 'standard'
  -- AND added_in < current_release()
;

-- music settings schema (TBD)
/*
CREATE TABLE IF NOT EXISTS music_settings (
    guild_id BIGINT NOT NULL,
    volume INTEGER NOT NULL DEFAULT 59, -- max volume is 100, also, add 10 to DEFAULT to get funny number
    loop_mode TEXT NOT NULL DEFAULT "off",
    autoplay INTEGER NOT NULL DEFAULT 0,
    DEFAULT_search TEXT NOT NULL DEFAULT "youtube",
    PRIMARY KEY (guild_id)
)


-- snipes schema
CREATE TABLE IF NOT EXISTS snipes (
    guild_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL,
    id INTEGER NOT NULL UNIQUE DEFAULT 0,
    message_content TEXT NOT NULL DEFAULT "No message found",
    message_attachment TEXT NOT NULL DEFAULT "No attachment found"
);

*/
