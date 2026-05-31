{
  description = "slown";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;

      buildFor = crossPkgs: crossPkgs.rustPlatform.buildRustPackage {
        pname = "slown";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = [ crossPkgs.openssl ];
      };
    in {
      packages.x86_64-linux = {
        default = buildFor pkgs;
        x86_64 = buildFor pkgs;
        aarch64 = buildFor pkgs.pkgsCross.aarch64-multiplatform;
        musl = buildFor pkgs.pkgsCross.musl64;
        win64 = buildFor pkgs.pkgsCross.mingwW64;
      };

      devShells.x86_64-linux.default = pkgs.mkShell {
        packages = [ pkgs.cargo pkgs.rustc pkgs.rustfmt pkgs.clippy ];
        RUST_BACKTRACE = 1;
      };
    };
}
