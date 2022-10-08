#!/bin/bash
source neardev/dev-account.env
TOKEN_ID="muzikant.testnet"
near view $CONTRACT_NAME ft_storage_minimum_balance "{ }"
