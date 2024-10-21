# 🚀 DEX - A Cutting-Edge Adventure in DeFi!
🔥 Warning: This project is a blazing trail in development. Proceed with caution and excitement!

## 🚀 Stack:
1. EVM chain
2. Rust as backend services.
3. Turbo framework on frontend.

## 🏗️ Architecture:
⚙️ Services That Keep The Engine Running:
1. Cache: ClickHouse & Dragonfly (Redis)

## 🧠 Smart Contract:
Deploy the smart contract locally to test services.

## How to run application
1. Running redis, clickhouse
2. Create def_exchange database, and int seed data
3. Generate key for https locally
```
cd rust_ws/crates/web-app; 
openssl genrsa -out key.pem 2048
openssl req -new -key key.pem -out csr.pem
openssl req -x509 -new -nodes -key key.pem -subj "/C=US/ST=California/L=San Francisco/O=YourCompany/CN=localhost" -days 365 -out cert.pem
```

4. Build css files
```
cd rust_ws/crates/web-app; 
just compile-js-webapp
just compile-css-prototype
```

5. Running application in dev mode

```
cd web-app; cargo run;
# enter https://0.0.0.0:3000/prototypes/trade/index.html
```
