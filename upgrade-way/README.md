Reproduce:

### (Re)create account to deploy contract to
near delete upgrade.mike.testnet mike.testnet
~/near/near-shell/bin/near create_account upgrade.mike.testnet --masterAccount mike.testnet

### Checkout and build
git checkout version-1
./build.sh

### Deploy
near deploy --accountId upgrade.mike.testnet --wasmFile res/enum_upgrade.wasm

### Set Version 1 info
near call upgrade.mike.testnet new '{"name": "ryu"}' --accountId upgrade.mike.testnet

### Change name
near call upgrade.mike.testnet set_name '{"new_name": "ken"}' --accountId upgrade.mike.testnet

### View Version 1 info
near view upgrade.mike.testnet log_version_data '{"index": 0}'

### Upgrade contract to Version 2
git checkout version-2
./build.sh
near deploy --accountId upgrade.mike.testnet --wasmFile res/enum_upgrade.wasm
near call upgrade.mike.testnet add_v2_with_color '{"favorite_color": "hot pink"}' --accountId upgrade.mike.testnet
near view upgrade.mike.testnet log_version_data '{"index": 0}'
near view upgrade.mike.testnet log_version_data '{"index": 1}'

### Change current version's (Version 2) color:
near call upgrade.mike.testnet set_favorite_color '{"new_color": "pastel green"}' --accountId upgrade.mike.testnet
near view upgrade.mike.testnet log_version_data '{"index": 1}'

near call upgrade.mike.testnet set_favorite_musician '{"new_musician": "radiohead"}' --accountId upgrade.mike.testnet
near view upgrade.mike.testnet log_version_data '{"index": 1}'

### Set all variables using setters
near call upgrade.mike.testnet set_all '{"new_name": "blanka", "new_color": "hipster plaid red", "new_musician": "glitch mob"}' --accountId upgrade.mike.testnet

### Get all variables using getters
near view upgrade.mike.testnet get_all