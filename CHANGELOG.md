# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-08-06

### Added
- Initial release of Xerve - A Local Development Platform
- Service management for Nginx and MariaDB
- Graphical user interface built with eframe/egui
- Automatic service initialization and cleanup
- Cross-platform support (Windows focused in this release)
- Modular architecture for easy extension

### Features
- **Service Management**: Start, stop, and monitor Nginx and MariaDB services
- **Automatic Initialization**: Automatically initializes MariaDB data directory if not present
- **Graceful Shutdown**: Ensures all services are properly stopped when the application exits
- **Status Monitoring**: Real-time service status display with visual indicators
- **Process Tracking**: Tracks service process IDs for more reliable management
- **Error Handling**: Comprehensive error handling and user feedback

### Technical Details
- Built with Rust for performance and safety
- Uses eframe/egui for cross-platform GUI
- Modular design with separate modules for application logic, services, and UI
- Proper resource management and cleanup procedures
- Release build optimizations for better performance

### Known Limitations
- Currently focused on Windows platform support
- Requires manual installation of Nginx and MariaDB in the resource directory
- MariaDB root user has no password by default