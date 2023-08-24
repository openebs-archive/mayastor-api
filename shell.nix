{ profile ? "nightly", date ? "2023-08-10", rustup ? false }:
let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs {
    overlays = [ (_: _: { inherit sources; }) (import ./nix/overlay.nix { }) ];
  };
  rust_pkg = import sources.nixpkgs { overlays = [ (import sources.rust-overlay) ]; };
  rust = rust_pkg.rust-bin.${profile}.${date}.default.override {
    extensions = [ "rust-src" ];
  };
in
with pkgs;
pkgs.mkShell {
  buildInputs = [
    cacert
    cargo-udeps
    clang
    commitlint
    git
    openssl
    pkg-config
    pre-commit
    protobuf
  ] ++ pkgs.lib.optional (!rustup) rust;

  LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
  PROTOC = "${protobuf}/bin/protoc";
  PROTOC_INCLUDE = "${protobuf}/include";
  NODE_PATH = "${nodePackages."@commitlint/config-conventional"}/lib/node_modules";
  USE_RUSTUP = "${toString (rustup)}";

  shellHook = ''
    pre-commit install
    pre-commit install --hook commit-msg

    if [ -n "$USE_RUSTUP" ]; then
        cat <<EOF >rust-toolchain.toml
    [toolchain]
    channel = "${lib.strings.concatMapStringsSep "-" (x: x) (lib.lists.drop 1 (lib.strings.splitString "-" rust.version))}"
    components = [ "rust-src" ]
    EOF
    fi
  '';
}
