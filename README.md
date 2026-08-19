# Tanix

### TONEX and AmpliTube 5 on Linux

Tanix is an unofficial Linux compatibility project designed to make **IK Multimedia TONEX** and **AmpliTube 5** available on Linux.

Inspired by projects such as Sober, Tanix aims to provide a simple, native-feeling Linux experience without requiring users to manually configure a traditional Windows compatibility environment.

The goal is simple:

**Install Tanix → Install TONEX or AmpliTube 5 → Play.**

> [!WARNING]
> **Tanix is an unofficial, community-developed project.**
>
> Tanix is **not affiliated with, endorsed by, sponsored by, or otherwise associated with IK Multimedia**.
>
> TONEX, AmpliTube, IK Multimedia, and all related trademarks, product names, and logos are the property of their respective owners.
>
> Use Tanix at your own risk.

## What is Tanix?

Tanix aims to provide a dedicated Linux runtime for running **TONEX** and **AmpliTube 5**.

Rather than attempting to recreate the applications or build an alternative guitar amp/effects platform, Tanix focuses on making the existing applications usable on Linux.

The project is intended to handle the compatibility layer between the Windows applications and the Linux environment while providing a straightforward user experience.

### The Goal

```text
Linux
  ↓
Tanix
  ↓
TONEX / AmpliTube 5
  ↓
Your Audio Interface
  ↓
Guitar
```

The long-term goal is to make using TONEX and AmpliTube on Linux feel as close as possible to using them on Windows.

## Current State of the Project

Tanix is currently in active development.

The project is focused on establishing the foundation required to run TONEX and AmpliTube 5 reliably on Linux.

Current and planned areas include:

* Linux application runtime
* Windows application compatibility
* TONEX support
* AmpliTube 5 support
* Audio interface support
* Low-latency audio
* Application installation
* Application updates
* Configuration management
* Persistent application data
* Controller/MIDI support

Compatibility will vary depending on the application version, Linux distribution, hardware, and audio configuration.

## Supported Applications

### TONEX

Tanix aims to support the desktop version of **IK Multimedia TONEX**.

Potential functionality includes:

* TONEX application
* Tone Model playback
* Tone Model management
* Presets
* Audio interface input/output
* Low-latency guitar processing
* MIDI controllers
* TONEX hardware integration where compatible

### AmpliTube 5

Tanix also aims to support **IK Multimedia AmpliTube 5**.

Potential functionality includes:

* AmpliTube 5
* Amplifier models
* Effects
* Cabinets
* Signal chains
* Presets
* Audio interface input/output
* MIDI controllers
* Low-latency guitar processing

## Why Tanix?

Running Windows audio applications on Linux can require significant manual configuration.

Users may need to deal with:

* Wine prefixes
* Wine configuration
* Dependencies
* Audio routing
* Windows DLLs
* Registry configuration
* Application-specific workarounds
* Separate launch scripts
* MIDI configuration
* Controller configuration

Tanix aims to hide as much of this complexity as possible.

Instead of:

```text
Wine
 ↓
Prefix
 ↓
Dependencies
 ↓
Configuration
 ↓
TONEX
```

Tanix aims for:

```text
Tanix
 ↓
TONEX
```

## Audio

Real-time guitar processing is a major focus of Tanix.

The project aims to provide reliable integration with Linux audio systems while minimizing latency.

The intended workflow is:

```text
Guitar
   ↓
USB Audio Interface
   ↓
Tanix
   ↓
TONEX / AmpliTube 5
   ↓
USB Audio Interface
   ↓
Headphones / Speakers
```

Support and performance will depend on the user's hardware and Linux audio configuration.

## Linux Support

Tanix is built specifically for Linux, with broad distribution support as a core goal of the project.

### Initial Support

Our initial focus is on:

* **All Arch-based distributions**
* **NixOS**
* **Gentoo**
* **Fedora**

This includes distributions such as Arch Linux, CachyOS, EndeavourOS, Manjaro, and other Arch-based systems.

### Future Support

As Tanix matures, we plan to expand support to additional major Linux distributions and ecosystems, including:

* Debian
* Ubuntu
* Linux Mint
* openSUSE
* Other major Linux distributions

Our goal is to make Tanix as distribution-agnostic as possible while maintaining a reliable and consistent experience across supported platforms.

Distribution support may vary depending on system libraries, package availability, audio configuration, and the packaging method used by Tanix.


## Architecture

Tanix is designed as a dedicated compatibility environment rather than a replacement for TONEX or AmpliTube.

The architecture is intended to isolate application-specific compatibility requirements from the host Linux system.

A simplified model is:

```text
┌─────────────────────────────┐
│           Linux             │
├─────────────────────────────┤
│           Tanix             │
│                             │
│  Runtime / Compatibility    │
│  Audio Integration          │
│  Configuration              │
│  Application Management     │
├─────────────────────────────┤
│      TONEX / AmpliTube 5    │
└─────────────────────────────┘
```

The internal architecture is subject to change as compatibility work progresses.

## Installation

Installation instructions will be provided once Tanix reaches a sufficiently stable release.

The intended experience is:

```text
1. Install Tanix
2. Launch Tanix
3. Install TONEX or AmpliTube 5
4. Configure your audio interface
5. Launch the application
6. Play
```

## Building

### Prerequisites

Development requirements may include:

* Rust
* Cargo
* A modern Linux distribution
* Git
* Required system libraries
* Audio development libraries

Additional dependencies may be required as compatibility functionality is implemented.

### Build

Clone the repository:

```bash
git clone https://github.com/xodus-gaming/tanix.git
cd tanix
```

Build Tanix:

```bash
cargo build --release
```

Run the development build:

```bash
cargo run
```

> Replace the repository URL above with the actual Tanix repository URL if it differs.

## Development

Check the project:

```bash
cargo check
```

Format the source:

```bash
cargo fmt
```

Run tests:

```bash
cargo test
```

Run Clippy:

```bash
cargo clippy
```

## Roadmap

### Phase 1 — Foundation

* [ ] Project architecture
* [ ] Linux runtime
* [ ] Application management
* [ ] Configuration system
* [ ] Basic launcher
* [ ] Logging and diagnostics

### Phase 2 — TONEX

* [ ] TONEX installation
* [ ] TONEX launching
* [ ] TONEX application compatibility
* [ ] Audio input/output
* [ ] Preset support
* [ ] MIDI support
* [ ] Controller support

### Phase 3 — AmpliTube

* [ ] AmpliTube 5 installation
* [ ] AmpliTube 5 launching
* [ ] Application compatibility
* [ ] Audio input/output
* [ ] Presets
* [ ] MIDI support
* [ ] Controller support

### Phase 4 — Audio

* [ ] PipeWire integration
* [ ] ALSA integration
* [ ] Low-latency configuration
* [ ] Automatic audio-device detection
* [ ] Buffer configuration
* [ ] Sample-rate configuration
* [ ] Audio diagnostics

### Phase 5 — User Experience

* [ ] Graphical installer
* [ ] Automatic configuration
* [ ] Application updates
* [ ] Prefix/runtime management
* [ ] Logs and diagnostics
* [ ] Per-application configuration
* [ ] Desktop integration

### Phase 6 — Advanced Compatibility

* [ ] Improved Windows API compatibility
* [ ] Hardware integration
* [ ] Advanced MIDI functionality
* [ ] Plugin compatibility
* [ ] Additional IK Multimedia software where feasible

## Compatibility

Tanix does **not** guarantee compatibility with every version of TONEX or AmpliTube.

Compatibility may be affected by:

* Application version
* Linux distribution
* Kernel version
* GPU
* Audio interface
* PipeWire/ALSA configuration
* Windows compatibility requirements
* DRM/licensing systems
* Online services
* Hardware-specific drivers

If an application works on one system but not another, please provide as much information as possible when reporting the issue.

## Reporting Issues

When opening an issue, include:

* Linux distribution
* Kernel version
* Tanix version
* TONEX/AmpliTube version
* CPU
* GPU
* Audio interface
* PipeWire/ALSA configuration
* Relevant logs
* Steps to reproduce the problem

Please do not include personal license keys, account credentials, or other sensitive information in issue reports.

## Contributing

Contributions are welcome.

To contribute:

1. Fork the repository.
2. Create a branch for your changes.
3. Make your changes.
4. Test them on Linux.
5. Submit a pull request.

For large architectural changes, opening an issue before implementing the change is recommended.

## Disclaimer

Tanix is an independent, unofficial open-source project.

**Tanix is not affiliated with IK Multimedia in any way.**

It is not endorsed by, sponsored by, maintained by, or otherwise associated with IK Multimedia.

**TONEX**, **AmpliTube**, **IK Multimedia**, and all related trademarks, product names, and logos are the property of their respective owners.

Tanix does not distribute proprietary IK Multimedia software unless explicitly permitted by its respective license.

Users are responsible for obtaining and licensing any proprietary software they use with Tanix.

## The Goal

Linux is an increasingly capable platform for music production and guitarists, but many popular Windows-only applications remain difficult to use.

Tanix exists to close that gap.

No complicated setup.

No manually maintained Wine prefixes.

No endless configuration guides.

Just:

```text
Install Tanix
     ↓
Install TONEX / AmpliTube 5
     ↓
Connect your interface
     ↓
Play
```

### The great guitar software migration to Linux.

**Tanix**
