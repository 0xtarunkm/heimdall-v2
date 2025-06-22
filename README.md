# Heimdall

Heimdall v2 is a Rust-based system designed to capture, process, and stream real-time data from the Solana blockchain. It leverages a Geyser plugin for data ingestion, Apache Kafka for reliable distribution, ClickHouse for analytics, and a gRPC service for real-time data streaming.

## Description
<img width="1088" alt="Screenshot 2025-06-22 at 7 20 24 PM" src="https://github.com/user-attachments/assets/45f0c876-3155-454d-bbb2-da6b6c67ef6e" />

This project provides a robust pipeline for accessing Solana's on-chain data, enabling applications like live DEX monitoring, trade visualization, and liquidity tracking.

## Setup

### Prerequisites

*   **Rust**: Version 1.70+
*   **Solana Tool Suite**: For validator operations.
*   **Apache Kafka**: Running and accessible (e.g., `localhost:9092`).
*   **ClickHouse**: Running and accessible (e.g., `localhost:8123`).

### Installation

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/0xtarunkm/heimdall-v2.git
    cd heimdall-v2
    ```
2.  **Build the project:**
    ```bash
    cargo build --release
    ```
3.  **Configure Services:**
    *   Modify the `.json` configuration files in the `config/` directory.
    *   Ensure Kafka and ClickHouse connection details are correct.
    *   Update the `libpath` in `config/heimdall.json` to point to your compiled Geyser plugin (`core/target/release/libcore.dylib`).

### Running Services

1.  **Start Solana Validator with Geyser Plugin:**
    ```bash
    solana-validator --geyser-plugin-config config/heimdall.json --ledger /path/to/your/solana/ledger
    ```
    *(Ensure your validator is configured to produce relevant data).*

2.  **Start Kafka and Create Topics:**
    *(Follow standard Kafka setup instructions. Ensure topics like `heimdall-accounts`, `heimdall-slots`, and `heimdall-transactions` exist).*

3.  **Start ClickHouse:**
    *(Follow standard ClickHouse setup instructions).*

4.  **Start the Consumer Service (to ClickHouse):**
    ```bash
    cargo run --bin heimdall-consumer config/consumer.json
    ```

## TODO

### High Priority
*   **gRPC Streaming Service Enhancements**: Implement client-side filtering for specific programs and accounts, add reconnection logic, and explore historical data streaming.
*   **Prometheus Setup**: Integrate Prometheus for system health monitoring, metrics collection (latency, throughput, errors), and dashboarding.
