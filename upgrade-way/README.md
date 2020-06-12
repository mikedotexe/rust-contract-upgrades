Reproduce:

### (Re)create account to deploy contract to
near delete upgrade.mike.testnet mike.testnet
~/near/near-shell/bin/near create_account upgrade.mike.testnet --masterAccount mike.testnet

### Checkout and build
git checkout version-1
./build.sh

### Export your testnet account as an environment variable
export NEAR_ACCT=mike.testnet

### Deploy
./deploy.sh

### Set Version 1 info
./new.sh

### Change name
./change-name.sh

### View Version 1 info (0 indicates index of versions)
./version-data.sh 0

### Upgrade contract to Version 2
git checkout version-2
./build.sh
./deploy.sh
./add-version-2.sh
./version-data.sh 0
./version-data.sh 1

### Change current version's (Version 2) color:
./set-color.sh "pastel green"
./version-data.sh 1

./set-musician.sh "Nilüfer Yanya"
./version-data.sh 1

### Set all variables using setters
./set-all.sh blanka "hipster plaid red" "glitch mob"

### Get all variables using getters
./get_all.sh

### Get current version, which is always the last one
./get-current-version.sh