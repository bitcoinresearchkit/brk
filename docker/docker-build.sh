#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
IMAGE_NAME="brk"
TAG="latest"

# Function to print colored output
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -t|--tag)
            TAG="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -t, --tag TAG             Tag for the image (default: latest)"
            echo "  -h, --help                Show this help message"
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Build the image
print_info "Building BRK Docker image..."
print_info "Image: ${IMAGE_NAME}:${TAG}"

# Detect script location and set paths accordingly
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Determine if we're running from project root or docker directory
if [[ "$(basename "$PWD")" == "docker" ]]; then
    # Running from docker directory
    DOCKERFILE_PATH="./Dockerfile"
    BUILD_CONTEXT=".."
    print_info "Running from docker directory"
else
    # Running from project root or elsewhere
    DOCKERFILE_PATH="docker/Dockerfile"
    BUILD_CONTEXT="."
    print_info "Running from project root"
fi

# Execute the build
if docker build -f "$DOCKERFILE_PATH" -t "${IMAGE_NAME}:${TAG}" "$BUILD_CONTEXT"; then
    print_info "Build completed successfully!"
    print_info "Image built as ${IMAGE_NAME}:${TAG}"
else
    print_error "Build failed!"
    exit 1
fi