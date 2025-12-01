{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # Common settings
        pkg_version = let env_val = builtins.getEnv "VERSION";
                      in if env_val == "" then "dev" else env_val;
        pname = "AOC2025";
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;
        craneLib = crane.mkLib pkgs;

        # Base arguments for crane
        crateArgs = {
          src = craneLib.cleanCargoSource ./.;
          nativeBuildInputs = [ pkgs.pkg-config pkgs.cargo-nextest pkgs.tzdata ];
          buildInputs = [ pkgs.openssl pkgs.cacert ];
        };

        # Function to generate builds for different profiles (debug/release)
        mkBuild = { isRelease ? false }:
          let
            profile = if isRelease then "release" else "dev";
            # Build dependencies first; they get cached
            cargoArtifacts = craneLib.buildDepsOnly (crateArgs // {
              # Add a profile suffix to the derivation name for clarity
              CARGO_PROFILE = profile;
              pname = "${pname}-deps-${profile}";
            });
          in {

            # Build the final package
            package = craneLib.buildPackage (crateArgs // {
              inherit pname;
              version = pkg_version;
              CARGO_PROFILE = profile;
              checkPhase = "cargo nextest run --cargo-profile ${profile}"; # Lets use nextest
              # Reuse the pre-built dependencies
              inherit cargoArtifacts;
            });

            # Cargo Test
            check = craneLib.cargoNextest (crateArgs // {
              inherit cargoArtifacts;
              cargoNextestArgs = [ "--failure-output=immediate" ];

              CARGO_PROFILE = profile;
              preCheck = ''
                export TZDIR="${pkgs.tzdata}/share/zoneinfo"
                export RUST_BACKTRACE=full
              '';
            });

            lint = craneLib.cargoClippy (crateArgs // {
              inherit cargoArtifacts;
              CARGO_PROFILE = profile;
              preCheck = ''
                export TZDIR="${pkgs.tzdata}/share/zoneinfo"
                export RUST_BACKTRACE=full
              '';
            });

            fmt = craneLib.cargoFmt (crateArgs // {
              CARGO_PROFILE = profile;
              preCheck = ''
                export TZDIR="${pkgs.tzdata}/share/zoneinfo"
                export RUST_BACKTRACE=full
              '';
            });


          };

        # Generate the two build variants
        builds = {
          dev = mkBuild { isRelease = false; };
          release = mkBuild { isRelease = true; };
        };

      in
      {
        packages = {
          # Debug builds
          default = builds.dev.package;

          dev-package = builds.dev.package;

          # Release builds
          release-package = builds.release.package;

        };

        checks = {
          default = builds.dev.check;

          lint = builds.release.lint;
          fmt = builds.release.fmt;
          test = builds.release.check;

          debug = builds.dev.check;
          release = builds.release.check;
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [
            (rustVersion.override { extensions = [ "rust-src" "rustfmt" "clippy" ]; })
            pkgs.rust-analyzer
            pkgs.cargo-nextest
            pkgs.iconv
            pkgs.openssl
            pkgs.tzdata
            pkgs.cargo-aoc
          ];
        };
      });
}


