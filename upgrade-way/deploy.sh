#!/bin/bash
near deploy --accountId upgrade.$NEAR_ACCT --wasmFile res/enum_upgrade.wasm
