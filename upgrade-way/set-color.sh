#!/bin/bash
# ./set-color.sh "pastel green" or ./set-color.sh red
TEMP=\"$@\"
echo $TEMP
near call upgrade.$NEAR_ACCT set_favorite_color "{\"new_color\": $TEMP}" --accountId upgrade.$NEAR_ACCT