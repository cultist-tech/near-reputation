#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="gena.testnet"
near view $CONTRACT_NAME storage_balance_of "{ \"account_id\": \"$ACCOUNT_ID\" }"
