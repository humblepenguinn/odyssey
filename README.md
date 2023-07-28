<h1 align="center">
<img src="assets/logo/cover.png" alt="oddyssey logo" width="600">
</h1>

<div align="center">
<h2 align="center">A custom blockchain written in Rust</h2>

[![Rust](https://img.shields.io/badge/Rust-Programming%20Language-orange)](https://www.rust-lang.org/)
[![Blockchain](https://img.shields.io/badge/Blockchain-v0.0.0--pre-blue)](https://example.com/your-blockchain-repo)


</div>


## Overview
`odyssey` is a custom blockchain project built from scratch in Rust. It aims to provide a simple and educational example of how a blockchain operates and its fundamental components. As of now, it is a work in progress and not ready for production use. There are several essential features already implemented, but there's still much to be done before it can be considered safe and reliable for real-world applications.

## Features
- `Block Creation`: Odyssey allows the creation of blocks that contain transactions and other necessary data for maintaining the blockchain.

- `Proof of Work`: The blockchain uses a Proof of Work (PoW) consensus mechanism to secure and validate new blocks before adding them to the chain.

- `Addresses`: Odyssey supports addresses, which are used to identify users and facilitate the transfer of funds between different accounts.

- `Transaction Support`: Users can send funds from one address to another within the blockchain.

- `Persistence`: The blockchain data is persisted on disk, ensuring that the data is not lost when the application restarts.

## Future Improvements
`Production Readiness`: Currently, Odyssey is not ready for production use. There are known issues, potential vulnerabilities, and performance bottlenecks that need to be addressed.

`Performance Enhancements`: The blockchain's performance can be improved to handle a larger number of transactions per second and reduce the time it takes to mine new blocks.

`Networking`: At the moment, Odyssey is a single-node blockchain running locally. In the future, it will be expanded to support a network of nodes that can communicate and synchronize with each other.

## Testing and Documentation

As the project is still under development, detailed documentation and usage examples have not been provided yet. However, if you are interested in exploring and testing the blockchain, you are welcome to view the source code and experiment with it. Keep in mind that it might require some understanding of Rust and blockchain concepts to get started.

## Contributing

Contributions to this project are welcome! Feel free to submit issues, suggestions, or even pull requests to help improve the `odyssey` blockchain.

## Disclaimer

This project is provided as-is, and there is no guarantee of its correctness, security, or suitability for any particular purpose. Use it at your own risk.

## License
This `odyssey` blockchain is licensed under the [MIT](LICENSE-MIT) License and the [Apache](LICENSE-APACHE) License. Feel free to modify and distribute it, but keep in mind the disclaimers and licensing terms.
