#!/bin/bash
# ./set-musician.sh "pastel green" or ./set-musician.sh red
TEMP=\"$@\"
echo $TEMP
near call upgrade.$NEAR_ACCT set_favorite_musician "{\"new_musician\": $TEMP}" --accountId upgrade.$NEAR_ACCT