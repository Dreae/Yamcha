CREATE TABLE players (
  player_id SERIAL PRIMARY KEY,
  steam_id VARCHAR(64) NOT NULL,
  server_id INT NOT NULL,
  last_name VARCHAR(255) NOT NULL,
  rating INT DEFAULT(1000) NOT NULL,
  shots_fired BIGINT DEFAULT(0) NOT NULL,
  shots_hit BIGINT DEFAULT(0) NOT NULL,
  kills INT DEFAULT(0) NOT NULL,
  deaths INT DEFAULT(0) NOT NULL,
  headshots INT DEFAULT(0) NOT NULL,
  UNIQUE (steam_id, server_id),
  FOREIGN KEY (server_id)
    REFERENCES servers(id)
    ON DELETE CASCADE
);

CREATE INDEX rating_idx ON players (rating)