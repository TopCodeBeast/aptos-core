-- Your SQL goes here
-- allows quick lookup of staking pool address to voter address and vice versa. Each staking pool 
-- can only be mapped to one voter address at a time. 
CREATE TABLE current_staking_pool_voter (
  staking_pool_address VARCHAR(66) PRIMARY KEY NOT NULL,
  voter_address VARCHAR(66) NOT NULL,
  last_transaction_version BIGINT NOT NULL
);
CREATE UNIQUE INDEX ctpv_spa_index ON current_staking_pool_voter (staking_pool_address);
CREATE INDEX ctpv_va_index ON current_staking_pool_voter (voter_address);