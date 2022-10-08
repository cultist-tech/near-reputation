#!/bin/bash
source neardev/dev-account.env
near call $CONTRACT_NAME migrate --accountId $CONTRACT_NAME "{  }"
