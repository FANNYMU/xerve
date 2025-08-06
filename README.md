# Xerve

A lightweight local development platform for running and managing development services.

## Overview

Xerve is a Rust-based application that provides an elegant graphical interface for managing local development services such as Nginx and MariaDB. It simplifies the process of starting, stopping, and monitoring these services through an intuitive user interface.

![Xerve Interface](docs/screenshot.png)

## Features

- **Service Management**: Easily start and stop Nginx and MariaDB services
- **Real-time Status Monitoring**: Visual indicators show the current status of each service
- **Automatic Initialization**: Automatically sets up MariaDB data directory if needed
- **Graceful Shutdown**: Ensures all services are properly stopped when the application exits
- **Process Tracking**: Tracks service processes for reliable management

## Prerequisites

- Windows 10/11 (currently focused on Windows support)
- Rust toolchain (for building from source)

## Installation

1. Clone the repository:

   ```
   git clone https://github.com/FANNYMU/xerve.git
   ```

2. Install dependencies:

   ```
   cd xerve
   cargo build --release
   ```

3. Download and extract Nginx and MariaDB to the `resource` directory:
   - Nginx should be in `resource/nginx/`
   - MariaDB should be in `resource/mariadb/`

## Usage

Run the application with:

```
cargo run --release
```

Or execute the compiled binary:

```
./target/release/xerve.exe
```

In the GUI:

1. Click "Start" to start a service
2. Click "Stop" to stop a service
3. Service status is displayed in real-time

## Architecture

Xerve follows a modular architecture:

- `app/` - Main application logic and lifecycle management
- `services/` - Service management implementations
- `ui/` - User interface components

## Building

To build Xerve for release:

```
cargo build --release
```

The executable will be located at `target/release/xerve.exe`.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Rust](https://www.rust-lang.org/) - The programming language used
- [egui](https://github.com/emilk/egui) - The GUI framework
- [Nginx](https://nginx.org/) - Web server
- [MariaDB](https://mariadb.org/) - Database server
