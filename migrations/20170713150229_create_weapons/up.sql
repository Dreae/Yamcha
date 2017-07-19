CREATE TABLE player_weapons (
    player_id INT NOT NULL,
    weapon VARCHAR(32) NOT NULL,
    kills INT DEFAULT(0) NOT NULL,
    deaths INT DEFAULT(0) NOT NULL,
    headshots INT DEFAULT(0) NOT NULL,
    shots_fired BIGINT DEFAULT(0) NOT NULL,
    shots_hit BIGINT DEFAULT(0) NOT NULL,
    PRIMARY KEY (player_id, weapon),
    FOREIGN KEY (player_id)
        REFERENCES players(player_id)
        ON DELETE CASCADE
);

CREATE INDEX weapon_kills_idx ON player_weapons (kills);