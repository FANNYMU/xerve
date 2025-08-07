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

## [1.0.1] - 2025-08-07

### Added
- Terminal output panel for real-time process logging
- Enhanced UI with professional dark theme and improved visual design
- Visual status indicators for services

### Fixed
- Service status race conditions with atomic status operations
- MariaDB process tracking inconsistency (standardized on mariadbd.exe)
- Error handling improvements with proper status updates
- Resource cleanup with timeout handling and periodic status checking
- Compiler warnings for unused variables

### Improved
- Real-time stdout/stderr capture from all service processes
- Enhanced error reporting with detailed context information
- Process management with better output capturing and status tracking
- UI/UX with modern dark theme, rounded corners, and improved spacing
- Terminal panel with read-only display of all process output
- Service cleanup with 10-second timeout and periodic status checking
- Memory management with automatic log limiting in terminal

### Technical Details
- Added thread-safe terminal logging system using OnceCell for global access
- Implemented BufReader with piped stdout/stderr for non-blocking output capture
- Added helper methods for atomic service status operations (is_running, is_stopped, update_status)
- Enhanced service cleanup with timeout-based waiting mechanism
- Improved process spawning and command execution with better error handling