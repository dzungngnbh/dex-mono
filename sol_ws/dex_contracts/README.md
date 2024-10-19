This contracts are written for Arbitrum mainnet

# Deploy
forge script -vvvv .\script\DeployDex.s.sol --rpc-url=http://127.0.0.1:8545 --broadcast --private-key=$PRIVATE_KEY

# Upgrade
```
# remember to edit .env file , in powershell use script sourceEnv, in bash use source .env
forge script -vvvv .\script\Upgrade.s.sol --rpc-url=http://127.0.0.1:8545 --broadcast --private-key=$PRIVATE_KEY
```
