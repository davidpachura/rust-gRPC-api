# A Basic gRPC API Created with Rust

**Steps to install needed dependencies to run:**

```shell
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Protobuf compiler
sudo apt install protobuf-compiler

# Install Protobuf header and lib
sudo apt install libprotobuf-dev

# Generate file set descriptor for api package
protoc --include_imports --include_source_info --descriptor_set_out=src/descriptor.pb -I proto api.proto
```