#!/bin/bash
source neardev/dev-account.env
OWNER_ID="muzikant.testnet"
near call $CONTRACT_NAME ft_transfer --accountId $CONTRACT_NAME "{ \"receiver_id\": \"$OWNER_ID\", \"amount\": \"100000000000000000000000000\" }" --amount "0.01"
