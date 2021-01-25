#!/usr/bin/env bash

set -e

echo "*** Initializing keyring"

seed=${secret_key:-'curio//mvp'}

for i in 1 2 3 4 ; do
  echo "$i node:"
      curio key  generate-node-key
      echo "\nstash:"
      curio key  inspect-key  //${seed}//$i//stash
      echo "controller:"
      curio key  inspect-key  //${seed}//$i
      echo "grandpa:"
      curio key  inspect-key  --scheme Ed25519 //${seed}//$i
      echo "BABE, ImOnlineId, AuthorityDiscoveryId:"
      curio key  inspect-key  --scheme Sr25519 //${seed}//$i

      mkdir -p ./node${i}/keystore
      curio key insert  -d .  --keystore-path ./node${i}/keystore/ --chain staging.json --key-type babe --suri //${seed}//$i
      curio key insert  -d .  --keystore-path ./node${i}/keystore/ --chain staging.json --key-type imon --suri //${seed}//$i
      curio key insert  -d .  --keystore-path ./node${i}/keystore/ --chain staging.json --key-type audi --suri //${seed}//$i
      curio key insert  -d .  --keystore-path ./node${i}/keystore/ --chain staging.json --key-type stak --suri //${seed}//$i//stash
      curio key insert  -d .  --keystore-path ./node${i}/keystore/ --chain staging.json --key-type gran --suri //${seed}//$i --scheme Ed25519

done
