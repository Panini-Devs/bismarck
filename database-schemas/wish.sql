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
