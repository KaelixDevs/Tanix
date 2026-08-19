# Tanix

### TONEX and AmpliTube 5 on Linux

Tanix is an unofficial Linux compatibility project designed to make **IK Multimedia TONEX** and **AmpliTube 5** available on Linux.

Inspired by projects such as Sober, Tanix aims to provide a simple, native-feeling Linux experience without requiring users to manually configure a traditional Windows compatibility environment.

The goal is simple:

**Install Tanix → Install TONEX or AmpliTube 5 → Play.**

> [!WARNING]
> **Tanix is an unofficial, community-developed project.**
>
> Tanix is **not affiliated with, endorsed by, sponsored by, maintained by, or otherwise associated with IK Multimedia in any way**.
>
> TONEX, AmpliTube, IK Multimedia, and all related trademarks, product names, and logos are the property of their respective owners.
>
> Tanix does not provide ownership or licensing of IK Multimedia software. Users are responsible for obtaining and properly licensing any proprietary software used with Tanix.
>
> **Use Tanix at your own risk.**

## What is Tanix?

Tanix is designed to provide a dedicated Linux compatibility environment for both IK MultiMedia's TONEX and Amplitube 5.

Rather than recreating these applications or developing an alternative guitar amp and effects platform, Tanix focuses on making the existing Windows applications usable on Linux.

Tanix is intended to manage the compatibility environment, application configuration, and Linux integration required to run supported applications while keeping the underlying complexity out of the user's way.

### The Goal

```text
Linux
  ↓
Tanix
  ↓
TONEX / AmpliTube 5
```

The long-term goal is to make using TONEX and AmpliTube on Linux feel as close as possible to using them on Windows.

## Current State of the Project

Tanix is currently in **early development**.

The project is currently focused on establishing the core architecture and Linux integration required to support TONEX and AmpliTube 5.

### Current Development

* Linux application foundation
* Audio device detection
* Linux audio integration
* Core application architecture
* Initial compatibility work

### Planned

* TONEX support
* AmpliTube 5 support
* Application installation
* Application management
* Low-latency audio
* Audio interface integration
* MIDI and controller support
* Configuration management
* Persistent application data
* Application updates

> **Note:** Tanix is not yet a complete replacement for a traditional Windows compatibility setup. Compatibility and functionality will improve as development progresses.

## Supported Applications

### TONEX

Tanix aims to support the desktop version of **IK Multimedia TONEX**.

Planned functionality includes:

* TONEX application support
* Tone Model playback
* Tone Model management
* Presets
* Audio input/output
* Low-latency guitar processing
* MIDI controllers
* TONEX hardware integration where technically feasible

### AmpliTube 5

Tanix also aims to support **IK Multimedia AmpliTube 5**.

Planned functionality includes:

* AmpliTube 5 application support
* Amplifier models
* Effects
* Cabinets
* Signal chains
* Presets
* Audio input/output
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

Tanix aims to handle as much of this complexity as possible.

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

The underlying compatibility technology may still involve components such as Wine or other compatibility technologies, but Tanix is intended to manage that complexity for the user.

## Audio

Low-latency audio is an important part of Tanix because TONEX and AmpliTube are intended to be used for real-time guitar processing.

Tanix aims to integrate with the Linux audio stack while providing a reliable and low-latency experience.

The intended audio workflow is:

```text
Guitar
   ↓
Audio Interface
   ↓
Linux Audio Stack
   ↓
TONEX / AmpliTube 5
   ↓
Linux Audio Stack
   ↓
Audio Interface
   ↓
Headphones / Speakers
```

Tanix sits around the application and compatibility environment rather than acting as the guitar-processing engine itself.

Audio performance will depend on the user's hardware, Linux audio configuration, application version, and system configuration.

## Linux Support

Tanix is built specifically for Linux, with broad distribution support as a core goal of the project.

### Initial Support

Our initial focus is on:

* **All Arch-based distributions**
* **NixOS**
* **Gentoo**
* **Fedora**

This includes distributions such as:

* Arch Linux
* CachyOS
* EndeavourOS
* Manjaro
* Other Arch-based distributions

### Future Support

As Tanix matures, we plan to expand support to additional major Linux distributions and ecosystems, including:

* Debian
* Ubuntu
* Linux Mint
* openSUSE
* Other major Linux distributions


Our goal is to make Tanix as distribution-agnostic as possible while maintaining a reliable and consistent experience across supported platforms.

Distribution support may vary depending on system libraries, package availability, audio configuration, and the packaging method used by Tanix.

## Future Projects

As the Tanix ecosystem matures, we may eventually develop a separate project focused on bringing the same concept to BSD-based operating systems.

This would be a separate project from Tanix rather than an extension of Tanix's Linux support.

## Architecture

Tanix is designed as a dedicated compatibility environment rather than a replacement for TONEX or AmpliTube.

The architecture is intended to separate application-specific compatibility requirements from the host Linux system and provide a consistent environment for supported applications.

A simplified model is:

```text
┌─────────────────────────────────┐
│              Linux              │
├─────────────────────────────────┤
│             Tanix               │
│                                 │
│  Compatibility Environment      │
│  Application Management         │
│  Configuration                  │
│  Linux Integration              │
│  Audio Integration              │
├─────────────────────────────────┤
│       TONEX / AmpliTube 5       │
└─────────────────────────────────┘
```

The internal architecture is still under development and may change as compatibility work progresses.

## Installation

Tanix is currently under active development and does not yet provide a finalized end-user installation workflow.

Once Tanix reaches a sufficiently stable release, the intended experience will be:

```text
1. Install Tanix
2. Launch Tanix
3. Install or select TONEX / AmpliTube 5
4. Configure your audio interface
5. Launch the application
6. Play
```

Installation and application-management functionality will be documented here as it becomes available.

## Building

### Prerequisites

Development requirements currently include:

* Rust
* Cargo
* Git
* A supported Linux distribution
* Required system libraries

Additional dependencies may be required as development progresses.

### Build

Clone the repository:

```bash
git clone https://github.com/KaelixDevs/Tanix.git
cd Tanix
```

Build Tanix:

```bash
cargo build --release
```

Run the development build:

```bash
cargo run
```

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

* [✓] Project architecture
* [✓] Linux runtime
* [ ] Application management
* [ ] Configuration system
* [ ] Basic launcher
* [ ] Logging and diagnostics

### Phase 2 — TONEX

* [ ] TONEX installation
* [ ] TONEX launching
* [ ] TONEX compatibility
* [ ] Audio input/output
* [ ] Preset support
* [ ] MIDI support
* [ ] Controller support

### Phase 3 — AmpliTube

* [ ] AmpliTube 5 installation
* [ ] AmpliTube 5 launching
* [ ] AmpliTube 5 compatibility
* [ ] Audio input/output
* [ ] Preset support
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
* [ ] Runtime management
* [ ] Logs and diagnostics
* [ ] Per-application configuration
* [ ] Desktop integration

### Phase 6 — Advanced Compatibility

* [ ] Improved Windows API compatibility
* [ ] Hardware integration
* [ ] Advanced MIDI functionality
* [ ] Controller integration
* [ ] Plugin compatibility where technically feasible
* [ ] Additional IK Multimedia software where technically feasible

## Compatibility

Tanix does **not** guarantee compatibility with every version of TONEX or AmpliTube.

Compatibility may be affected by:

* Application version
* Linux distribution
* Kernel version
* CPU
* GPU
* Audio interface
* PipeWire/ALSA configuration
* Windows compatibility requirements
* DRM and licensing systems
* Online services
* Hardware-specific drivers

Compatibility may also change between application releases.

If an application works on one system but not another, please provide as much information as possible when reporting the issue.

## Reporting Issues

When opening an issue, include:

* Linux distribution
* Kernel version
* Tanix version or commit
* TONEX/AmpliTube version
* CPU
* GPU
* Audio interface
* PipeWire/ALSA configuration
* Relevant logs
* Steps to reproduce the problem

Please do **not** include:

* License keys
* Passwords
* Account credentials
* Authentication tokens
* Other sensitive information

## Contributing

Contributions are welcome.

To contribute:

1. Fork the repository.
2. Create a branch for your changes.
3. Make your changes.
4. Test them on Linux.
5. Submit a pull request.

For larger architectural changes, opening an issue before implementing the change is recommended.

## Disclaimer

Tanix is an independent, unofficial open-source project.

**Tanix is not affiliated with IK Multimedia in any way.**

It is not endorsed by, sponsored by, maintained by, or otherwise associated with IK Multimedia.

**TONEX**, **AmpliTube**, **IK Multimedia**, and all related trademarks, product names, and logos are the property of their respective owners.

Tanix does not claim ownership of, redistribute, or provide licenses for proprietary IK Multimedia software.

Users are responsible for obtaining and properly licensing any proprietary software they use with Tanix.

## The Goal

Linux is an increasingly capable platform for music production and guitarists, but many popular Windows-only applications remain difficult to use.

Tanix exists to close that gap.

No complicated setup.

No manually maintained compatibility environments.

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
