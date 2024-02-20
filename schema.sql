CREATE TABLE IF NOT EXISTS staked_tokens_changes (
    "indexer" VARCHAR(40),
    "tokens" NUMERIC,
    "staked_tokens" NUMERIC,
    "block_number" DECIMAL,
    "transaction_hash" VARCHAR(64),
    "block_timestamp" TIMESTAMP,
    PRIMARY KEY ("indexer", "block_number", "transaction_hash")
)