#!/bin/bash
source neardev/dev-account.env
#OWNER_ID="muzikant.testnet"
#near call $CONTRACT_NAME storage_deposit --accountId $CONTRACT_NAME "{ \"account_id\": \"$OWNER_ID\" }" --amount "0.1"

ACCOUNT="nalogovik.testnet"

near call $CONTRACT_NAME storage_deposit --accountId $CONTRACT_NAME "{ \"account_id\": \"$ACCOUNT\" }" --amount "0.1"
