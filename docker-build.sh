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

# Execute the build
if docker build -t "${IMAGE_NAME}:${TAG}" .; then
    print_info "Build completed successfully!"
    print_info "Image built as ${IMAGE_NAME}:${TAG}"
else
    print_error "Build failed!"
    exit 1
fi