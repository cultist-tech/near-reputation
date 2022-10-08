#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
CALLER="mfight-rep.testnet"

near call $CONTRACT_NAME rep_vote --accountId $CALLER "{ \"receiver_id\": \"$ACCOUNT_ID\", \"amount\": 1, \"up\": true }"
