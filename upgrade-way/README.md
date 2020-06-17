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
./set-name.sh "George Costanza"

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

### Get current version, which is usually the last item in the Vector, but can change if there are removals
./get-current-version.sh

### Add version 3 which "migrates" data from Version 1 into a new struct
./add-version-3.sh

### Remove Version 1 as it's no longer needed in storage
./remove-version-1.sh

### Regular confirmations…
./get-current-version.sh
./get_all.sh

# References

What is the syntax to match on a reference to an enum?
- https://stackoverflow.com/a/36592628/711863

Possibly consider adding "ref" in places
https://doc.rust-lang.org/stable/rust-by-example/scope/borrow/ref.html

Possibly ref here:
https://stackoverflow.com/a/22266744/711863

#### Upgrade:
Gas cost
6,186,107,190,595
6186107190595
https://explorer.testnet.near.org/transactions/8gZtQFrCLZRgpY4PVL6BN7iUUFkKuks2DBZeC3geRmsr
100000000000000 --attached
100000000000000/6186107190595 could do 16 calls

#### Old way
Gas cost
5,772,080,413,987
5772080413987
https://explorer.testnet.near.org/transactions/2NmZYM18x5sVDebewLptf7fHuc3FhSVdAFcjAKPCWwJd
100000000000000 --attached
100000000000000/5772080413987 could do 17 calls

6186107190595/5772080413987
1.0717292115
Upgrade way costs 7.17% more gas per insertion
