#!/bin/bash
for i in {1..191}
do
  near call old.mike.testnet add_to_map "{\"account\": \"explorer03-$i\", \"desc\": \"old friend\"}" --accountId old.mike.testnet
  near call upgrade.mike.testnet add_to_map "{\"account\": \"explorer03-$i\", \"desc\": \"old friend\"}" --accountId upgrade.mike.testnet
done