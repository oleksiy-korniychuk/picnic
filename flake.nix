{
  description = "Rust development environment for guns-germs-steel (Bevy game)";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Bevy runtime dependencies
        libraries = with pkgs; [
          # Graphics
          vulkan-loader
          libxkbcommon
          wayland

          # X11 libraries
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr

          # Audio
          alsa-lib

          # System
          udev
        ];

        # Development packages
        packages = with pkgs; [
          pkg-config
          clang
          mold  # Fast linker
        ];

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            # Rust toolchain with rust-analyzer, clippy, rustfmt
            (pkgs.rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" "rust-analyzer" "clippy" ];
            })
          ] ++ packages ++ libraries;

          shellHook = ''
            echo "🦀 Rust/Bevy development environment loaded!"
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo ""
            echo "Available commands:"
            echo "  cargo run          - Run the game"
            echo "  cargo build        - Build the project"
            echo "  cargo test         - Run tests"
            echo "  cargo clippy       - Run linter"
            echo "  cargo fmt          - Format code"
            echo ""
            echo "Using mold linker for faster builds"
          '';

          # Environment variables for Bevy
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libraries;

          # Use mold linker for faster builds
          CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = "${pkgs.clang}/bin/clang";
          RUSTFLAGS = "-C link-arg=-fuse-ld=${pkgs.mold}/bin/mold";

          # Set Vulkan SDK path
          VULKAN_SDK = "${pkgs.vulkan-loader}/share/vulkan";
          VK_LAYER_PATH = "${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d";
        };
      }
    );
}
