#!/bin/bash
# ./set-all.sh blanka "hipster plaid red" "glitch mob"
TEMP1=\"$1\"
echo $TEMP1
TEMP2=\"$2\"
echo $TEMP2
TEMP3=\"$3\"
echo $TEMP3
near call upgrade.$NEAR_ACCT set_all "{\"new_name\": $TEMP1, \"new_color\": $TEMP2, \"new_musician\": $TEMP3}" --accountId upgrade.$NEAR_ACCT