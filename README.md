<div align="center">
  <img src="docs/logo.png" alt="Xerve Logo" width="120" >
  
  <h1>Xerve</h1>
  
  <p><strong>A lightweight local development platform for running and managing development services</strong></p>

  <div>
    <a href="https://github.com/FANNYMU/xerve/releases/latest">
      <img src="https://img.shields.io/github/v/release/FANNYMU/xerve?style=flat&logo=github&color=2bbc8a" alt="Latest Release">
    </a>
    <a href="https://github.com/FANNYMU/xerve/stargazers">
      <img src="https://img.shields.io/github/stars/FANNYMU/xerve?style=flat&logo=github&color=ffd700" alt="Stars">
    </a>
    <a href="https://github.com/FANNYMU/xerve/network/members">
      <img src="https://img.shields.io/github/forks/FANNYMU/xerve?style=flat&logo=github&color=blueviolet" alt="Forks">
    </a>
    <a href="https://github.com/FANNYMU/xerve/releases">
      <img src="https://img.shields.io/github/downloads/FANNYMU/xerve/total?style=flat&color=orange" alt="Downloads">
    </a>
    <a href="https://github.com/FANNYMU/xerve/graphs/contributors">
      <img src="https://img.shields.io/github/contributors/FANNYMU/xerve.svg?style=flat&color=informational" alt="Contributors">
    </a>
    <a href="https://github.com/FANNYMU/xerve/issues">
      <img src="https://img.shields.io/github/issues/FANNYMU/xerve.svg?style=flat&color=critical" alt="Issues">
    </a>
    <a href="https://github.com/FANNYMU/xerve/pulls">
      <img src="https://img.shields.io/github/issues-pr/FANNYMU/xerve.svg?style=flat&color=blue" alt="Pull Requests">
    </a>
    <a href="https://github.com/FANNYMU/xerve/commits/main">
      <img src="https://img.shields.io/github/last-commit/FANNYMU/xerve?style=flat&logo=github&color=4c1" alt="Last Commit">
    </a>
    <a href="https://www.rust-lang.org/">
      <img src="https://img.shields.io/badge/language-Rust-DEA584?style=flat&logo=rust&logoColor=white" alt="Rust">
    </a>
    <a href="https://github.com/emilk/egui">
      <img src="https://img.shields.io/badge/framework-egui-47848F?style=flat&logo=egui&logoColor=white" alt="egui">
    </a>
  </div>

  <p>
    <a href="#features">Features</a> •
    <a href="#installation">Installation</a> •
    <a href="#usage">Usage</a> •
    <a href="#screenshots">Screenshots</a> •
    <a href="#contributing">Contributing</a>
  </p>
</div>

<br>

## 🌟 Overview

Xerve is a modern, Rust-based application that provides an elegant graphical interface for managing local development services such as Nginx and MariaDB. It simplifies the process of starting, stopping, and monitoring these services through an intuitive user interface with real-time terminal output.

With Xerve, you can effortlessly manage your local development environment without the complexity of command-line tools or configuration files.

<br>

## ✨ Features

### 🚀 Service Management

<div>
  <ul>
    <li><strong>Nginx Integration</strong> - Start, stop, and monitor your local Nginx web server</li>
    <li><strong>MariaDB Integration</strong> - Manage your local MariaDB database server</li>
    <li><strong>Automatic Initialization</strong> - MariaDB data directory is automatically created and initialized if not present</li>
    <li><strong>Process Tracking</strong> - Services are tracked by process ID for reliable management</li>
  </ul>
</div>

### 🖥️ Beautiful User Interface

<div>
  <ul>
    <li><strong>Modern Dark Theme</strong> - Sleek dark interface with carefully chosen colors for reduced eye strain</li>
    <li><strong>Real-time Status Monitoring</strong> - Visual indicators show the current status of each service</li>
    <li><strong>Integrated Terminal</strong> - Real-time terminal output showing all process logs directly in the UI</li>
    <li><strong>One-click Controls</strong> - Simple Start/Stop buttons for each service</li>
  </ul>
</div>

### 🔧 System Integration

<div>
  <ul>
    <li><strong>Graceful Shutdown</strong> - All services are automatically stopped when the application exits</li>
    <li><strong>Enhanced Error Handling</strong> - Comprehensive error handling with clear feedback to the user</li>
    <li><strong>Resource Management</strong> - Proper cleanup of resources when services are stopped</li>
    <li><strong>Atomic Operations</strong> - Service status operations are thread-safe to prevent race conditions</li>
  </ul>
</div>

<br>

## 📦 Prerequisites

<div>
  <ul>
    <li><strong>Windows 10/11</strong> (currently focused on Windows support)</li>
    <li><strong>Rust toolchain</strong> (for building from source)</li>
  </ul>
</div>

<br>

## 🛠️ Installation

### 📥 Download Pre-built Binary

<div>
  <ol>
    <li>Download the latest release from the <a href="https://github.com/FANNYMU/xerve/releases">Releases page</a></li>
    <li>Extract the archive to your preferred location</li>
  </ol>
</div>

### 🔧 Build from Source

<div>
  <ol>
    <li>Clone the repository:
      <pre><code>git clone https://github.com/FANNYMU/xerve.git</code></pre>
    </li>
    <li>Navigate to the project directory:
      <pre><code>cd xerve</code></pre>
    </li>
    <li>Install dependencies and build:
      <pre><code>cargo build --release</code></pre>
    </li>
    <li>Download and extract Nginx and MariaDB to the <code>resource</code> directory:
      <ul>
        <li>Nginx should be in <code>resource/nginx/</code></li>
        <li>MariaDB should be in <code>resource/mariadb/</code></li>
      </ul>
    </li>
  </ol>
</div>

<br>

## ▶️ Usage

### Running the Application

<div>
  <p>Run the application with:</p>
  <pre><code>cargo run --release</code></pre>
  
  <p>Or execute the compiled binary:</p>
  <pre><code>./target/release/xerve.exe</code></pre>
</div>

### Using the Interface

<div>
  <ol>
    <li><strong>Start Services</strong> - Click the "Start" button next to a service to begin it</li>
    <li><strong>Stop Services</strong> - Click the "Stop" button to gracefully stop a running service</li>
    <li><strong>Monitor Output</strong> - Watch the integrated terminal for real-time process output</li>
    <li><strong>Check Status</strong> - Visual indicators show the current status of each service</li>
  </ol>
</div>

<br>

## 📸 Screenshots

### Main Interface

<div align="center">
  <img src="docs/screenshot.png" alt="Xerve Interface" width="800">
  <p><em>The elegant main interface showing service controls and integrated terminal output</em></p>
</div>
<br>

## 🔨 Building

To build Xerve for release:

<pre><code>cargo build --release</code></pre>

The executable will be located at <code>target/release/xerve.exe</code>.

<br>

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

<div>
  <ol>
    <li><strong>Fork the repository</strong></li>
    <li><strong>Create your feature branch</strong>
      <pre><code>git checkout -b feature/AmazingFeature</code></pre>
    </li>
    <li><strong>Commit your changes</strong>
      <pre><code>git commit -m 'Add some AmazingFeature'</code></pre>
    </li>
    <li><strong>Push to the branch</strong>
      <pre><code>git push origin feature/AmazingFeature</code></pre>
    </li>
    <li><strong>Open a Pull Request</strong></li>
  </ol>
</div>

### Ways to Contribute

<div>
  <ul>
    <li>Report bugs and issues</li>
    <li>Suggest new features</li>
    <li>Improve documentation</li>
    <li>Submit code enhancements</li>
    <li>Add support for new services</li>
  </ul>
</div>

<br>

## 📜 License

This project is licensed under the MIT License - see the <a href="LICENSE">LICENSE</a> file for details.

<br>

## 🙏 Acknowledgments

We'd like to thank the creators of these amazing technologies that make Xerve possible:

<div>
  <ul>
    <li><a href="https://www.rust-lang.org/"><strong>Rust</strong></a> - Systems programming language that powers Xerve</li>
    <li><a href="https://github.com/emilk/egui"><strong>egui</strong></a> - The immediate mode GUI framework used for the interface</li>
    <li><a href="https://nginx.org/"><strong>Nginx</strong></a> - High-performance web server</li>
    <li><a href="https://mariadb.org/"><strong>MariaDB</strong></a> - Open source relational database</li>
  </ul>
</div>

<br>

## 📚 Additional Resources

<div>
  <ul>
    <li><a href="CHANGELOG.md"><strong>Changelog</strong></a> - See what's new in each release</li>
    <li><a href="RELEASE.md"><strong>Release Notes</strong></a> - Detailed release information</li>
    <li><a href="https://github.com/FANNYMU/xerve/issues"><strong>Issues</strong></a> - Report bugs or request features</li>
    <li><a href="CONTRIBUTING.md"><strong>Contributing Guide</strong></a> - Learn how to contribute to the project</li>
  </ul>
</div>

<br>

<div align="center">
  <strong>Made with ❤️ by FANNYMU, for developers</strong>
</div>
