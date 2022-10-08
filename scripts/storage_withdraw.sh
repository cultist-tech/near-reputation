#!/bin/bash
source neardev/dev-account.env
OWNER_ID="muzikant.testnet"
near call $CONTRACT_NAME storage_withdraw --accountId $OWNER_ID "{ \"amount\": \"1250000000000000000000\" }" --depositYocto "1"
