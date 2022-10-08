#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
near view $CONTRACT_NAME ft_storage_balance_of "{ \"account_id\": \"$ACCOUNT_ID\" }"
