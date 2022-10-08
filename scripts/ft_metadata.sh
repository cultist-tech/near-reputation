#!/bin/bash
source neardev/dev-account.env
near view $CONTRACT_NAME ft_metadata "{ }"
