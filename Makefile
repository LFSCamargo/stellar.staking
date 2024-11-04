# Default target, which will trigger the build
default: build

# Build target using the Soroban contract build command
build:
	soroban contract build

# Test target which builds the project and then runs the tests
test: build
	cargo test

# Clean target which removes the build artifacts
clean:
	cargo clean

# Alias for the test target, that technically runs all the important scripts
all: test