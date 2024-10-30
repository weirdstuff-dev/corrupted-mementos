CREATE TABLE corrupted_mementos (
  extrinsic_hash VARCHAR NOT NULL PRIMARY KEY,
  extrinsic_id INTEGER NOT NULL,
  block_id INTEGER NOT NULL,
  block_hash VARCHAR NOT NULL,
  minted BOOLEAN DEFAULT FALSE
)