# Setup and Run Instructions for unifyair-core (Local Development Environment)

This document provides instructions on how to set up and run the `unifyair-core` project in a **local development environment**, including running the development script `run.sh` and understanding the Cargo configuration.

## Prerequisites
Before you begin, ensure you have the following installed and configured:
  - **Rust:** The project is built using Rust, so you'll need the Rust toolchain (Cargo, rustc, etc.).
  - **rsync:** The `run.sh` script uses `rsync` for file synchronization.
  - **SSH Access:** You need SSH access to the remote server specified in the `run.sh` script.
  - **Git:** You need git installed to fetch dependencies.

## Local and Remote Directory Structure
-   **Local:**
    -   The script expects a specific directory structure. `asn-models`, `open-api`, and `unifyair-core` should be sibling directories.
    -   For example, if `run.sh` is located in `/path/to/unifyair-core`, the following directories must exist:
        -   `/path/to/asn-models`
        -   `/path/to/open-api`
        -   `/path/to/unifyair-core`

-   **Remote:**
    -   The remote server should have a `unifyair` directory in the user's home directory (`$HOME/unifyair`).
    -   After running `run.sh`, the remote directory structure will mirror the local structure:
        -   `$HOME/unifyair/asn-models`
        -   `$HOME/unifyair/open-api`
        -   `$HOME/unifyair/unifyair-core`

## Running the Development Script (`run.sh`)

The `run.sh` script automates the following tasks:

1.  **Cargo Clean:** Cleans the Cargo build artifacts in the `asn-models`, `open-api`, and `unifyair-core` directories.
2.  **Sync Directories:** Synchronizes the `asn-models`, `open-api`, and `unifyair-core` directories from your local machine to the remote server using `rsync`.
3.  **Remote Execution:** Executes the `lightning-cli` command on the remote server via SSH.

**Usage:**

```bash
./run.sh <ssh_server>
```

-   Replace `<ssh_server>` with the address of your remote server (e.g., `user@server.com`).

**Example:**

```bash
./run.sh myuser@192.168.1.100
```

## Cargo Configuration (`.cargo/config.toml`)

The `.cargo/config.toml` file contains build configurations and dependency overrides for the project. Here's a breakdown:

```toml
[build]
rustflags = [
    "--cfg",
    "tokio_unstable",
    "-Zmacro-backtrace",
    # '-C target_feature=+avx2',
]
rustdocflags = ["--cfg", "tokio_unstable"]

[net]
git-fetch-with-cli = true

[patch.'ssh://git@github.com/UnifyAir/asn-models.git/']
ngap = {path = "../asn-models/ngap"}

[patch.'ssh://git@bitbucket.org/blocknet/openapi-5gc.git/']
openapi-smf = { path = "../open-api/openapi-nfs/openapi-smf" }
openapi-nrf = { path = "../open-api/openapi-nfs/openapi-nrf" }
openapi-chf = { path = "../open-api/openapi-nfs/openapi-chf" }
openapi-pcf = { path = "../open-api/openapi-nfs/openapi-pcf" }
openapi-udm = { path = "../open-api/openapi-nfs/openapi-udm" }
oasbi = { path = "../open-api/oasbi" }
```

**Explanation:**

-   **`[build]`:**
    -   `rustflags`: Compiler flags passed to `rustc`.
        -   `--cfg tokio_unstable`: Enables unstable Tokio features.  This suggests the project uses the Tokio runtime and relies on some of its experimental features.
        -   `-Zmacro-backtrace`: Provides more detailed backtraces for macro expansions, aiding in debugging.
        -   `# '-C target_feature=+avx2'`:  This line is commented out, but if uncommented, it would enable AVX2 instructions for potentially improved performance on compatible CPUs.
    -   `rustdocflags`:  Flags passed to `rustdoc` (the Rust documentation generator).
        -    `--cfg tokio_unstable`:  Also enables unstable Tokio features for documentation generation.

-   **`[net]`:**
    -   `git-fetch-with-cli = true`: Forces Cargo to use the system's Git executable instead of its built-in Git library. This can be useful for accessing private repositories or dealing with specific Git configurations.

-   **`[patch.'<repository_url>']`:**  These sections override the source of specific dependencies.  Instead of fetching them from crates.io (the default Rust package registry), Cargo will use the specified local paths.
    -   This setup is crucial for local development where you're likely modifying the `asn-models` and `openapi-5gc` projects alongside `unifyair-core`. Changes in these local dependencies will be reflected immediately without needing to publish new versions.
    -   `ngap = {path = "../asn-models/ngap"}`:  The `ngap` crate (likely a part of `asn-models`) is overridden to use the local path `../asn-models/ngap`.
    -   The `openapi-*` crates and `oasbi` crate are overridden similarly, pointing to directories within the `../open-api` directory.

In summary, the `config.toml` configures the build process to use unstable Tokio features, enables detailed macro backtraces, forces the use of the system Git, and, most importantly, overrides the locations of several key dependencies to use local paths. This configuration is tailored for a development environment where you are working on multiple related projects simultaneously.