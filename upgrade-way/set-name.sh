#!/bin/bash
TEMP1=\"$1\"
near call upgrade.$NEAR_ACCT set_name "{\"new_name\": $TEMP1}" --accountId upgrade.$NEAR_ACCT