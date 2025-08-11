# 🌟 Xerve

<div align="center">
  
  <img src="docs/logo.png" alt="Xerve Logo" width="150" style="border-radius: 20px; box-shadow: 0 4px 8px rgba(0,0,0,0.1);">
  
  <h3>✨ A Lightweight Local Development Platform ✨</h3>
  <p><em>Effortlessly run and manage your development services with style</em></p>

  <p align="center">
    <a href="https://github.com/FANNYMU/xerve/releases/latest">
      <img src="https://img.shields.io/github/v/release/FANNYMU/xerve?style=for-the-badge&logo=github&color=2bbc8a&labelColor=1a1a1a" alt="Latest Release">
    </a>
    <a href="https://github.com/FANNYMU/xerve/stargazers">
      <img src="https://img.shields.io/github/stars/FANNYMU/xerve?style=for-the-badge&logo=github&color=ffd700&labelColor=1a1a1a" alt="Stars">
    </a>
    <a href="https://github.com/FANNYMU/xerve/network/members">
      <img src="https://img.shields.io/github/forks/FANNYMU/xerve?style=for-the-badge&logo=github&color=blueviolet&labelColor=1a1a1a" alt="Forks">
    </a>
  </p>

  <p align="center">
    <a href="https://github.com/FANNYMU/xerve/releases">
      <img src="https://img.shields.io/github/downloads/FANNYMU/xerve/total?style=for-the-badge&color=orange&labelColor=1a1a1a" alt="Downloads">
    </a>
    <a href="https://github.com/FANNYMU/xerve/graphs/contributors">
      <img src="https://img.shields.io/github/contributors/FANNYMU/xerve.svg?style=for-the-badge&color=informational&labelColor=1a1a1a" alt="Contributors">
    </a>
    <a href="https://github.com/FANNYMU/xerve/issues">
      <img src="https://img.shields.io/github/issues/FANNYMU/xerve.svg?style=for-the-badge&color=critical&labelColor=1a1a1a" alt="Issues">
    </a>
  </p>

  <p align="center">
    <a href="https://www.rust-lang.org/">
      <img src="https://img.shields.io/badge/Built_with-Rust-DEA584?style=for-the-badge&logo=rust&logoColor=white&labelColor=1a1a1a" alt="Rust">
    </a>
    <a href="https://github.com/emilk/egui">
      <img src="https://img.shields.io/badge/UI_Framework-egui-47848F?style=for-the-badge&logo=rust&logoColor=white&labelColor=1a1a1a" alt="egui">
    </a>
  </p>

  <h4>
    🚀 <a href="#features">Features</a> • 
    📦 <a href="#installation">Installation</a> • 
    ▶️ <a href="#usage">Usage</a> • 
    📸 <a href="#screenshots">Screenshots</a> • 
    🤝 <a href="#contributing">Contributing</a>
  </h4>

</div>

---

## 🌟 Overview

**Xerve** is a modern, Rust-powered application that transforms how you manage local development services. With its elegant graphical interface, you can effortlessly control services like **Nginx** and **MariaDB** without touching the command line.

> 🎯 **Perfect for developers** who want a clean, visual way to manage their local development stack

### 💡 Why Choose Xerve?

- **🎨 Beautiful Interface** - Modern dark theme designed for developers
- **⚡ Lightning Fast** - Built with Rust for optimal performance
- **🔧 Zero Configuration** - Works out of the box with automatic service detection
- **📊 Real-time Monitoring** - Live process output and status indicators

---

## ✨ Features

<table>
<tr>
<td width="50%">

### 🚀 **Service Management**
- 🌐 **Nginx Integration** - Complete web server control
- 🗄️ **MariaDB Integration** - Full database management
- ⚙️ **Auto-Initialization** - Automatic setup for new installations
- 🔍 **Process Tracking** - Reliable PID-based service monitoring

</td>
<td width="50%">

### 🎨 **Beautiful Interface**
- 🌙 **Modern Dark Theme** - Easy on the eyes
- 📊 **Real-time Status** - Live service indicators
- 💻 **Integrated Terminal** - Process logs in real-time
- 🖱️ **One-Click Controls** - Simple start/stop buttons

</td>
</tr>
</table>

### 🛡️ **System Integration**

| Feature | Description |
|---------|-------------|
| 🛑 **Graceful Shutdown** | Automatic service cleanup on exit |
| ⚠️ **Error Handling** | Comprehensive error reporting |
| 🧹 **Resource Management** | Proper cleanup and resource handling |
| 🔒 **Thread Safety** | Atomic operations prevent race conditions |

---

## 📦 Installation

### 🎯 **Quick Start - Pre-built Binary**

1. **Download** the latest release from [**Releases**](https://github.com/FANNYMU/xerve/releases) 📥
2. **Extract** to your preferred location 📁
3. **Run** and enjoy! 🚀

### 🔨 **Build from Source**

```bash
# 1. Clone the repository
git clone https://github.com/FANNYMU/xerve.git

# 2. Navigate to project directory
cd xerve

# 3. Build the project
cargo build --release

# 4. Set up resources (create resource directory structure)
mkdir -p resource/nginx resource/mariadb
```

#### 📋 **Resource Setup**
Place your services in the resource directory:
- 📁 `resource/nginx/` - Your Nginx installation
- 📁 `resource/mariadb/` - Your MariaDB installation

---

## ▶️ Usage

### 🏃‍♂️ **Running Xerve**

**Development:**
```bash
cargo run --release
```

**Production:**
```bash
./target/release/xerve.exe
```

### 🎮 **Using the Interface**

<div align="center">

| Action | Description |
|--------|-------------|
| 🟢 **Start Service** | Click the green "Start" button |
| 🔴 **Stop Service** | Click the red "Stop" button |
| 👀 **Monitor Output** | Watch the integrated terminal |
| 📊 **Check Status** | View real-time status indicators |

</div>

---

## 📸 Screenshots

<div align="center">

### 🖥️ **Main Interface**
<img src="docs/screenshot.png" alt="Xerve Main Interface" width="900" style="border-radius: 10px; box-shadow: 0 8px 16px rgba(0,0,0,0.3);">

*The elegant main interface showcasing service controls and integrated terminal output*

</div>

---

## 🔧 Prerequisites

<div align="center">

| Requirement | Version | Status |
|-------------|---------|--------|
| 🖥️ **Windows** | 10/11 | ✅ Supported |
| 🦀 **Rust** | Latest | 🔧 For building |
| 🌐 **Nginx** | Any | 📦 Optional |
| 🗄️ **MariaDB** | Any | 📦 Optional |

</div>

---

## 🤝 Contributing

We ❤️ contributions! Here's how you can help make Xerve even better:

### 🚀 **Getting Started**

1. **🍴 Fork** the repository
2. **🌿 Create** your feature branch
   ```bash
   git checkout -b feature/AmazingFeature
   ```
3. **💾 Commit** your changes
   ```bash
   git commit -m '✨ Add some AmazingFeature'
   ```
4. **📤 Push** to the branch
   ```bash
   git push origin feature/AmazingFeature
   ```
5. **🔄 Create** a Pull Request

### 🎯 **Ways to Contribute**

<div align="center">

| Type | Description |
|------|-------------|
| 🐛 **Bug Reports** | Help us squash those pesky bugs |
| 💡 **Feature Ideas** | Suggest cool new features |
| 📚 **Documentation** | Improve our docs |
| 🔧 **Code** | Submit awesome enhancements |
| 🆕 **Services** | Add support for new services |

</div>

---

## 📜 License

<div align="center">

**MIT License** - see the [**LICENSE**](LICENSE) file for details

*Free to use, modify, and distribute! 🎉*

</div>

---

## 🙏 Acknowledgments

Special thanks to these amazing projects that power Xerve:

<div align="center">

| Technology | Description | Link |
|------------|-------------|------|
| 🦀 **Rust** | Systems programming language | [rust-lang.org](https://www.rust-lang.org/) |
| 🎨 **egui** | Immediate mode GUI framework | [github.com/emilk/egui](https://github.com/emilk/egui) |
| 🌐 **Nginx** | High-performance web server | [nginx.org](https://nginx.org/) |
| 🗄️ **MariaDB** | Open source database | [mariadb.org](https://mariadb.org/) |

</div>

---

## 📚 Resources

<div align="center">

### 📖 **Documentation**
[📋 Changelog](CHANGELOG.md) • [📝 Release Notes](RELEASE.md) • [🤝 Contributing Guide](CONTRIBUTING.md)

### 🔗 **Links**
[🐛 Report Issues](https://github.com/FANNYMU/xerve/issues) • [💡 Request Features](https://github.com/FANNYMU/xerve/issues/new) • [💬 Discussions](https://github.com/FANNYMU/xerve/discussions)

</div>

---

<div align="center">

### ✨ **Made with** ❤️ **by [FANNYMU](https://github.com/FANNYMU)**

**🎯 For developers, by developers**

<sub>⭐ **Star this repo if you find it useful!** ⭐</sub>

</div>