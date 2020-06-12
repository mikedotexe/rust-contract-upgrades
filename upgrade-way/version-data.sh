#!/bin/bash
# ./version-data.sh 0 returns the index 0 of the versions
near view upgrade.$NEAR_ACCT log_version_data '{"index": '$1'}'
