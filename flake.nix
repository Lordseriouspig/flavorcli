{
  description = "StarCLI – Stardance API CLI";

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
          starcli = pkgs.rustPlatform.buildRustPackage rec {
            pname = "starcli";
            version = "1.0.3";

            src = pkgs.fetchFromGitHub {
              owner = "lordseriouspig";
              repo = "starcli";
              rev = "v${version}";
              hash = "sha256-Y7FP47gv8g5hNy7/k8OgQW2slj7dmVi4j2JAfDedrEY=";
            };

            cargoHash = "sha256-V+OTtBVOGke7/+r+y1ciopR85QUju1WEd2DiY7jFiMI=";

            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.openssl pkgs.dbus ];

            postInstall = ''
              install -Dm644 LICENSE $out/share/licenses/${pname}/LICENSE
              install -Dm644 README.MD $out/share/doc/${pname}/README.MD
              install -Dm644 CHANGELOG.md $out/share/doc/${pname}/CHANGELOG.md
            '';

            meta = with pkgs.lib; {
              description = "Fully-featured implementation of the Stardance API";
              homepage = "https://github.com/lordseriouspig/starcli";
              license = licenses.gpl3Plus;
              platforms = platforms.linux;
            };
          };

          default = starcli;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ self.packages.${system}.starcli ];

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
