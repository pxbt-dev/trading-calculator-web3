# 🦀 Trading Calculator Web3

![Rust](https://img.shields.io/badge/Rust-1.70+-orange)
![Solidity](https://img.shields.io/badge/Solidity-0.8.30-blue)
![Web3](https://img.shields.io/badge/Web3-Multi--Chain-purple)

First attempt at a Web3 version of trading-calculator-web allowing users to connect Bitcoin, Ethereum or Solana wallet to pre-fill values - // WIP

## 📋 Project Status
- **Backend**: ✅ (please see examples below)
- **Frontend**: 🚧 In Development

## 🚀 Live API Examples

### Multi-Chain Wallet Queries

**Ethereum - WETH Contract**
```
curl -X POST http://127.0.0.1:8080/wallet/connect -H "Content-Type: application/json" -d '{"address": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", "chain": "ethereum"}'

Response: {"address":"0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2","chain":"ethereum","balances":{"ETH":2608722.5834535016},"total_value_usd":8028187227.223144}
💰 2.6M ETH = $8.0B - Wrapped ETH contract balance
```

**Bitcoin - Whale Wallet**
```
curl -X POST http://127.0.0.1:8080/wallet/connect -H "Content-Type: application/json" -d '{"address": "34xp4vRoCGJym3xR7yCVPFHoCNxv4Twseo", "chain": "bitcoin"}'

Response: {"address":"34xp4vRoCGJym3xR7yCVPFHoCNxv4Twseo","chain":"bitcoin","balances":{"BTC":248597.58117559},"total_value_usd":22795652401.058075}
🐋 248K BTC = $22.8B - One of the largest Bitcoin wallets
```

**Solana - Binance Hot Wallet**
```
curl -X POST http://127.0.0.1:8080/wallet/connect -H "Content-Type: application/json" -d '{"address": "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM", "chain": "solana"}'

Response: {"address":"9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM","chain":"solana","balances":{"SOL":11503453.678619087},"total_value_usd":1614624758.330975}
🏦 11.5M SOL = $1.6B - Binance exchange hot wallet
```

## 🛠️ Tech Stack
- **Backend**: Rust + Actix-web
- **Smart Contracts**: Solidity + Foundry
- **Blockchain**: Real RPC for ETH, BTC, SOL
- **Prices**: CoinGecko API
