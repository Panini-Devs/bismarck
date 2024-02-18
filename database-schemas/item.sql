CREATE TABLE item (
  id INT PRIMARY KEY,
  name TEXT NOT NULL,
  rarity INT CHECK(rarity BETWEEN 3 AND 5),
  is_event INT CHECK(is_event = 0 OR is_event = 1),
  type INT CHECK(type = 0 OR type = 1), -- 0 character, 1 weapon
  release_update INT
);
