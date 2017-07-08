CREATE TABLE players (
  steam_id VARCHAR(64) NOT NULL,
  server_id INT UNSIGNED NOT NULL,
  last_name VARCHAR(255) NOT NULL,
  rating INT UNSIGNED DEFAULT(1000) NOT NULL,
  accuracy FLOAT DEFAULT(0.0) NOT NULL,
  kills INT UNSIGNED DEFAULT(0) NOT NULL,
  deaths INT UNSIGNED DEFAULT(0) NOT NULL,
  headshots INT UNSIGNED DEFAULT(0) NOT NULL,
  PRIMARY KEY (steam_id, server_id),
  INDEX rating_ind (rating),
  FOREIGN KEY (server_id)
    REFERENCES servers(id)
    ON DELETE CASCADE
)