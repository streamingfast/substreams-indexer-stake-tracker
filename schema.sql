CREATE TABLE IF NOT EXISTS staked_tokens_changes (
    "indexer" VARCHAR(40),
    "tokens" NUMERIC,
    "block_number" DECIMAL,
    "block_timestamp" TIMESTAMP,
    PRIMARY KEY ("indexer", "block_number")
)