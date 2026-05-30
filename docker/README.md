# Devora Docker Images

This directory contains Docker configurations for Devora, providing containerized versions of the universal project scaffolding framework.

## Available Images

### Image Variants

- **`ghcr.io/Nathandona/devora:latest-debian`** - Default Debian-based image
- **`ghcr.io/Nathandona/devora:latest-alpine`** - Minimal Alpine-based image
- **`ghcr.io/Nathandona/devora:latest`** - Multi-architecture image (Debian-based)

### Architecture Support

- `linux/amd64` - Intel/AMD 64-bit
- `linux/arm64` - ARM 64-bit (Apple Silicon, ARM servers)

## Quick Start

### Using the Docker Image

```bash
# Pull the image
docker pull ghcr.io/Nathandona/devora:latest

# Run Devora commands
docker run --rm -v $(pwd):/workspace ghcr.io/Nathandona/devora:latest list

# Create a new project
docker run --rm -v $(pwd):/workspace ghcr.io/Nathandona/devora:latest new my-project rust --framework cmake
```

### Docker Compose

Create a `docker-compose.yml` file:

```yaml
version: '3.8'
services:
  devora:
    image: ghcr.io/Nathandona/devora:latest
    volumes:
      - ./:/workspace
    working_dir: /workspace
    command: ["--help"]
```

Then run:

```bash
docker-compose run --rm devora list
docker-compose run --rm devora new my-project rust
```

## Usage Examples

### Project Generation

```bash
# Generate a Rust project
docker run --rm -v $(pwd):/workspace ghcr.io/Nathandona/devora:latest \
  new my-rust-app rust --framework cmake --non-interactive

# Generate a Python project (if plugin exists)
docker run --rm -v $(pwd):/workspace ghcr.io/Nathandona/devora:latest \
  new my-python-app python --framework flask --non-interactive
```

### Plugin Management

```bash
# List available languages and frameworks
docker run --rm ghcr.io/Nathandona/devora:latest list

# Get information about a specific framework
docker run --rm ghcr.io/Nathandona/devora:latest info rust cmake
```

### Development Environment

```bash
# Create an interactive development environment
docker run -it --rm -v $(pwd):/workspace \
  ghcr.io/Nathandona/devora:latest bash
```

## Image Differences

### Debian Image (`:latest-debian`)

- **Size**: ~150MB
- **Base**: Debian Bookworm Slim
- **Features**: Full development tools, shell completions
- **Use case**: Development, CI/CD pipelines

### Alpine Image (`:latest-alpine`)

- **Size**: ~80MB
- **Base**: Alpine Linux 3.19
- **Features**: Minimal footprint, musl libc
- **Use case**: Production, resource-constrained environments

## Configuration

### Environment Variables

- `RUST_LOG` - Logging level (default: `info`)
- `DEVORA_PLUGINS_DIR` - Plugins directory path
- `HOME` - User home directory (set to `/workspace`)

### Volumes

- `/workspace` - Working directory for project generation

### User

The containers run as a non-root user `devora` (UID: 1001, GID: 1001) for security.

## Building Locally

```bash
# Build Debian variant
docker build -f docker/Dockerfile.debian -t devora:debian .

# Build Alpine variant
docker build -f docker/Dockerfile.alpine -t devora:alpine .

# Build multi-architecture image
docker buildx build -f docker/Dockerfile.debian --platform linux/amd64,linux/arm64 -t devora:multi .
```

## Security

- Non-root user execution
- Minimal base images
- Regular security updates
- Vulnerability scanning
- Signed images

## Support

For issues with the Docker images:

1. Check the [GitHub Issues](https://github.com/Nathandona/Devora/issues)
2. Review the [Devora Documentation](https://github.com/Nathandona/Devora#readme)
3. Create a new issue with the `docker` label

## License

The Docker images and Devora are licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.