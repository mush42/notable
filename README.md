## Notable

Notable is a user-friendly tool designed for managing and editing your personal wiki.

### Features

- **Embedded Wiki Engine**: Notable incorporates [Feather Wiki](https://feather.wiki/), a versatile, lightweight and accessible wiki engine.
- **Local Server**: Provides a local server for easy management and editing of your wiki content.
- **Single File Storage**: All your wiki content is saved in a single HTML file named `notable-wiki.html` located in your Documents directory.

With Notable, organizing and accessing your personal knowledge has never been simpler.

### Usage

Getting started with Notable is straightforward. Follow these steps:

1. **Download the Single-File Executable**: Obtain the latest version from the [latest release page](https://github.com/mush42/notable/releases/latest).
2. **Place the Executable**: Move the `notable` executable to your Desktop or any directory you want.
3. **Run Notable** by **double-clicking** the `notable` executable file.

Notable includes the [auto-save extension](https://feather.wiki/?page=extensions_auto-save) which automatically saves your pages and settings as you work. However, you can always manually click the "Save to Server" button to ensure your changes are preserved.

### Build

**Notable** is cross-platform and does not have any platform-specific dependencies or configurations. You can build it on any major operating system (Windows, macOS, Linux) without issues.

To build the project, you'll need to have Rust installed on your machine. If you haven't installed Rust yet, you can do so by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can build the project by running:

```bash
cargo build --release
```

### License

Copyright (c) Musharraf Omer. MIT Licence. See [LICENSE](./LICENSE) for more details.
