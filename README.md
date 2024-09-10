# Disperse&Collect Rust API

Short overview:
- The application consists of both a Rust API and a Forge project;
- Collecting ETH is implemented using a withdrawal contract, as passing a private key to the function is not secure, and there is no other way to perform a collection with a single transaction;
- The `/api/run-scripts` directory contains bash scripts to simulate sending requests, provided for example purposes.

## Tests
```bash
cd contracts
forge test
```

```bash
cd api
cargo test
```

```bash
cd api/
cargo run --release
cd run-scripts
chmod +x disperse-eth.sh
./disperse-eth.sh
```
