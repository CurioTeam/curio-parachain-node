# initial configuration
  you need:
   - linux server with 4 CPU Intel Xeon or AMD EPYC
   - 8 Gb RAM (32 Gb for better performance)
   - 400 Gb free space on SSD 
   
# install curio as a service

[download binary](https://github.com/CurioTeam/curio-parachain-node/releases)
put it into /usr/local/bin/curio
```bash
mv  curio /usr/local/bin/
chmod +x /usr/local/bin/curio
```

generate node's key and keystore if it needs

## configure service.

create new user account
```bash
#useradd -s /sbin/nologin  blockchain
```

create file /etc/systemd/system/curio.service

```text
[Unit]
Description=Curio node service
After=network.target
StartLimitIntervalSec=0
[Service]
Type=simple
Restart=always
RestartSec=1
LimitNOFILE=49152
User=blockchain
Group=blockchain
ExecStart=/usr/local/bin/curio --chain /etc/curio/staging.json --name curio_v1 --rpc-cors all --rpc-methods unsafe --node-key-file /etc/curio/node.key --validator  --rpc-external --ws-external  --ws-port 9944  --rpc-port 9933 --base-path /var/curio  --bootnodes /ip4/135.181.132.159/tcp/30333/p2p/12D3KooWN2spXSwSSCmdA9EE2mkJQESAFQefqRGvYGw75kHcmmFb

[Install]
WantedBy=multi-user.target
Alias=curio.service
```

[see more about systemd service](https://manpages.ubuntu.com/manpages/focal/man5/systemd.service.5.html)

##  create chain spec from scratch

```bash
curio build-spec --chain staging > spec.json      
```

replace properties with 

  "properties": {
     "tokenDecimals": 18,
     "tokenSymbol": "CGT"
  },

replace bootnodes and telemetryUrl with empty list []
set id "staging" value

move staging.json into  /etc/curio/ directory.

# node.key was earlier generated by keyring.sh script
place node.key file in /etc/curio.
adjust read-only(0400) access only for “blockchain” account

```
chown blockchain:blockchain /etc/curio/*
chmod 400 node.key
```

## create database directory

```bash
mkdir -p /var/curio/chains/staging
mv keystore /var/curio/chains/staging

chown -R blockchain:blockchain /var/curio
```

## check and run service

Start and stop service
```bash
systemctl start curio
```

Check status of service
```bash
systemctl status curio
```
Working node should show in log that new blocks are created/received, and last finalised block number increments. Finalised blocks numbers should be not too far from number of latest block if things are good.


## prepare for mainnet
download keyring script [from](https://github.com/CurioTeam/curio-parachain-node/blob/main/scripts/keyring.sh)

run it for keys generation

``` bash
secret_key=ENTER_YOUR_SECRET_HERE ./keyring.sh
``` 
WARN! keep your secret confidential    

use generated keystore, node keys and accounts to configure validators accounts and validators nodes.

**NOTE! DON'T USE DEFAULT keystore and staging.json   for MAINNET. REPLACE chain config (staging.json) account data with newly generated 


Now, we need to make “mainnet.json”, based on “staging.json”.
Replace stash, controller, grandpa, babe, imonline keys in `staging.json`,
set node id value in mainnet.json replacing "staging" with "mainnet"

generate raw chain config 
```bash
curio build-spec --raw --disable-default-bootnode --chain /etc/curio/staging.json  >  /etc/curio/mainnet.json
```
change `--chain /etc/curio/staging.json` with `--chain /etc/curio/mainnet.json`  in `/etc/systemd/system/curio.service` 

replace keystores and node keys for each validator and restart nodes.

(new keystore file should be placed in /var/curio/chains/mainnet/keystore )

```bash
#systemctl restart curio
```

[more about node configuration](https://substrate.dev/docs/en/tutorials/start-a-private-network/)

## purge and restart blockchain

**PURGE destroy all data and the  blockchain will start from scratch

stop all validators
```bash
#systemctl stop curio
```
clear database
```bash
#sudo -u blockchain curio pursh-chain --chain /etc/curio/staging.json --base-path /var/curio
```

replace staging.json and/or  ipci executable if required.
Start all nodes and look at service log
```bash
#systemctl start curio
```

```bash
journalctl -xe -u curio
```



