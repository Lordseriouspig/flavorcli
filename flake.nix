{
  description = "FlavorCLI – Flavortown API CLI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        packages = rec {
          flavorcli = pkgs.rustPlatform.buildRustPackage rec {
            pname = "flavorcli";
            version = "0.3.0";

            src = pkgs.fetchFromGitHub {
              owner = "lordseriouspig";
              repo = "flavorcli";
              rev = "v${version}";
              hash = "sha256-Y7FP47gv8g5hNy7/k8OgQW2slj7dmVi4j2JAfDedrEY=";
            };

            cargoHash = "sha256-V+OTtBVOGke7/+r+y1ciopR85QUju1WEd2DiY7jFiMI=";

            # Only needed if you have native deps later
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.openssl pkgs.dbus ];

            postInstall = ''
              install -Dm644 LICENSE $out/share/licenses/${pname}/LICENSE
              install -Dm644 README.MD $out/share/doc/${pname}/README.MD
              install -Dm644 CHANGELOG.md $out/share/doc/${pname}/CHANGELOG.md
            '';

            meta = with pkgs.lib; {
              description = "Fully-featured implementation of the Flavortown API";
              homepage = "https://github.com/lordseriouspig/flavorcli";
              license = licenses.gpl3Plus;
              platforms = platforms.linux;
            };
          };

          default = flavorcli;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ self.packages.${system}.flavorcli ];

          packages = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            git
          ];

          RUST_BACKTRACE = 1;
        };
      });
}
