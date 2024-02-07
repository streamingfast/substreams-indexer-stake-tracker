# Indexer Staked Tokens Tracker

This is a substreams which can be used to track the staked tokens of an indexer.

This is a first draft and the which recalculates the staked tokens of an indexer every time an allocation is closed.

Future work could include:
- Tracking when a stake is directly added to an indexer's account

## Usage

By default, this substreams tracks all indexers, but you can filter the indexers you want to track by setting the params on the `map_allocation_closed` module.

The default parameter is '*' to track all indexers.  To track specific indexers, set this value to a json list of indexer addresses.  For example: '["35917c0eb91d2e21bef40940d028940484230c06"]'

## Deployment

This substreams has a `db_out` module which can be used with the `substreams-sink-sql` to store the data in a SQL database.

For more information on how to deploy this way, refer to https://github.com/streamingfast/substreams-sink-sql