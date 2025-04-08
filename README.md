# UnifyAir Core - 5G Network Functions Implementation

UnifyAir Core is an open-source implementation of 5G Core Network Functions (NFs) written in Rust, focusing on high performance, reliability, and scalability. This project aims to provide a modular and efficient implementation of key 5G network functions.

## Project Overview

UnifyAir Core implements the following Network Functions (NFs):

### Implemented Network Functions

* **Omnipath (AMF - Access and Mobility Management Function)**
  * Handles mobility management
  * User authentication and authorization
  * Access control and security procedures
  * UE registration and connection management

### Planned Nf Support
* **DataWarp (UPF - User Plane Function)**
  * Packet routing and forwarding
  * QoS handling for user plane traffic
  * Traffic measurement and reporting
  * Packet inspection and policy enforcement

* **InfiniSync (SMF - Session Management Function)**
  * Session establishment, modification, and release
  * IP address allocation and management
  * Traffic steering control
  * QoS policy implementation

## Project Scope

### Current Features
- Async-first architecture using Tokio runtime
- High-performance network packet processing
- Standards-compliant 5G NF implementations
- Modular design for easy extension and modification
- Comprehensive logging and monitoring
- REST APIs for configuration and management

### Planned Features
- Enhanced security features and encryption
- Additional Network Function implementations
- Performance optimization and scaling improvements
- Extended monitoring and analytics
- Cloud-native deployment support

## Getting Started

### Prerequisites
- Rust 1.82 or above nightly
- Cargo package manager

### Local Development Setup
1. Clone the repository:
   ```bash
   git clone https://github.com/unifyair/unifyair-core.git
   cd unifyair-core
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. For detailed setup instructions, refer to [SETUP.md](SETUP.md)

## Configuration

Configuration is managed through:
- Environment variables
- Configuration files in `config/`
- Command line arguments

## Testing

Run the test suite:
```bash
cargo test
```

## Contributing

We welcome contributions! Please see our [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Process
1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to your fork
5. Submit a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

- Project Website: [https://unifyair.io](https://unifyair.io)
- Issue Tracker: [GitHub Issues](https://github.com/UnifyAir/unifyair-core/issues)
- Discussion Forum: [Discord](https://discord.gg/yuJHdZ4vEF)
- Twitter: [Twitter](https://x.com/unifyair)
