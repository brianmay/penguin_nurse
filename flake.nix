{
  description = "Medical Records";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
  inputs.nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.devenv.url = "github:cachix/devenv";
  inputs.crane.url = "github:ipetkov/crane";
  inputs.flockenzeit.url = "github:balsoft/flockenzeit";

  outputs =
    inputs@{
      self,
      nixpkgs,
      nixpkgs-unstable,
      flake-utils,
      rust-overlay,
      devenv,
      crane,
      flockenzeit,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        pkgs-unstable = nixpkgs-unstable.legacyPackages.${system};
        wasm-bindgen-cli = pkgs.wasm-bindgen-cli_0_2_100;

        # This should work but isn't currently required.
        # dioxus-cli = pkgs-unstable.dioxus-cli.overrideAttrs (old: rec {
        #   version = "0.6.1";
        #   src = pkgs.fetchCrate {
        #     inherit version;
        #     pname = old.pname;
        #     hash = "sha256-mQnSduf8SHYyUs6gHfI+JAvpRxYQA1DiMlvNofImElU=";
        #   };
        #   cargoDeps = old.cargoDeps.overrideAttrs (pkgs.lib.const {
        #     name = "${old.pname}-vendor.tar.gz";
        #     inherit src;
        #     outputHash = "sha256-7jNOdlX9P9yxIfHTY32IXnT6XV5/9WDEjxhvHvT7bms=";
        #     # outputHash = pkgs.lib.fakeHash;
        #   });
        # });
        dioxus-cli = pkgs.callPackage ./nix/dioxus-cli.nix { };
        # dioxus-cli = pkgs.dioxus-cli;

        rustPlatform = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
          extensions = [ "rust-src" ];
        };
        # craneLib = (crane.mkLib pkgs).overrideToolchain rustPlatform;

        nodejs = pkgs.nodejs_20;

        build_env = {
          BUILD_DATE = with flockenzeit.lib.splitSecondsSinceEpoch { } self.lastModified; "${F}T${T}${Z}";
          VCS_REF = "${self.shortRev or self.dirtyShortRev or "dirty"}";
        };

        postgres = pkgs.postgresql_15;
        tailwindcss = pkgs.tailwindcss_4;

        nodePackages = pkgs.buildNpmPackage {
          name = "node-packages";
          src = ./.;
          npmDepsHash = "sha256-ygHirjfE6Kc5BPOriGDD2G8FsF4u3VHRahAqPBsSnpo=";
          dontNpmBuild = true;
          inherit nodejs;

          installPhase = ''
            mkdir $out
            cp -r node_modules $out
            ln -s $out/node_modules/.bin $out/bin
          '';
        };

        # frontend =
        #   let
        #     common = {
        #       src = ./.;
        #       pname = "penguin-nurse-frontend";
        #       version = "0.0.0";
        #       cargoExtraArgs = "--features web";
        #       # nativeBuildInputs = with pkgs; [ pkg-config ];
        #       # buildInputs = with pkgs; [
        #       #   protobuf
        #       # ];
        #       CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
        #       doCheck = false;
        #     };

        #     # Build *just* the cargo dependencies, so we can reuse
        #     # all of that work (e.g. via cachix) when running in CI
        #     cargoArtifacts = craneLib.buildDepsOnly common;

        #     # Run clippy (and deny all warnings) on the crate source.
        #     clippy = craneLib.cargoClippy (
        #       {
        #         inherit cargoArtifacts;
        #         cargoClippyExtraArgs = "-- --deny warnings";
        #       }
        #       // common
        #     );

        #     # Build the actual crate itself.
        #     pkg = craneLib.buildPackage (
        #       {
        #         inherit cargoArtifacts;
        #         doCheck = false;
        #       }
        #       // common
        #       // build_env
        #     );
        #   in
        #   {
        #     clippy = clippy;
        #     pkg = pkg;
        #   };

        # frontend-bindgen = pkgs.stdenv.mkDerivation {
        #   name = "penguin-nurse-frontend-bindgen";
        #   src = ./.;

        #   buildPhase = ''
        #     ${wasm-bindgen-cli}/bin/wasm-bindgen \
        #       --target bundler \
        #       --out-dir pkg \
        #       --omit-default-module-path \
        #       ${frontend.pkg}/bin/penguin-nurse.wasm

        #     ln -s ${nodePackages}/node_modules ./node_modules
        #     export PATH="${nodejs}/bin:${nodePackages}/bin:$PATH"
        #     webpack
        #   '';

        #   installPhase = ''
        #     copy_hashed() {
        #         local filename
        #         local hash
        #         local dst
        #         local extension
        #         local name
        #         filename="$(basename "$1")"
        #         hash="''$(${pkgs.b3sum}/bin/b3sum --raw "$1" | head --bytes 6 | base64)"
        #         extension="''${filename##*.}"
        #         name="''${filename%.*}"
        #         dst="$out/$name-$hash.$extension"
        #         cp "$1" "$dst"
        #     }

        #     mkdir $out
        #     cp -rv dist/* $out/

        #     copy_hashed "assets/header.svg"
        #     copy_hashed "assets/main.css"
        #     copy_hashed "assets/favicon.ico"
        #   '';
        # };

        # backend =
        #   let
        #     common = {
        #       src = ./.;
        #       pname = "phone_db-backend";
        #       version = "0.0.0";
        #       cargoExtraArgs = "--features server";
        #       # nativeBuildInputs = with pkgs; [ pkg-config ];
        #       buildInputs = [
        #         pkgs.postgresql_15
        #         #   openssl
        #         #   python3
        #         #   protobuf
        #       ];
        #       # See https://github.com/ipetkov/crane/issues/414#issuecomment-1860852084
        #       # for possible work around if this is required in the future.
        #       # installCargoArtifactsMode = "use-zstd";
        #     };

        #     # Build *just* the cargo dependencies, so we can reuse
        #     # all of that work (e.g. via cachix) when running in CI
        #     cargoArtifacts = craneLib.buildDepsOnly common;

        #     # Run clippy (and deny all warnings) on the crate source.
        #     clippy = craneLib.cargoClippy (
        #       {
        #         inherit cargoArtifacts;
        #         cargoClippyExtraArgs = "-- --deny warnings";
        #       }
        #       // common
        #     );

        #     # Next, we want to run the tests and collect code-coverage, _but only if
        #     # the clippy checks pass_ so we do not waste any extra cycles.
        #     coverage = craneLib.cargoTarpaulin ({ cargoArtifacts = clippy; } // common);

        #     # Build the actual crate itself.
        #     pkg = craneLib.buildPackage (
        #       {
        #         inherit cargoArtifacts;
        #         doCheck = true;
        #         # CARGO_LOG = "cargo::core::compiler::fingerprint=info";
        #       }
        #       // common
        #       // build_env
        #     );
        #   in
        #   {
        #     inherit clippy coverage pkg;
        #   };

        # combined = pkgs.runCommand "penguin-nurse" { } ''
        #   mkdir -p $out
        #   mkdir -p $out/bin/public
        #   cp -r ${backend.pkg}/. $out
        #   cp -r ${frontend-bindgen}/. $out/bin/public
        # '';

        combined =
          let
            cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
            rev = build_env.VCS_REF;
          in
          pkgs.rustPlatform.buildRustPackage {
            pname = cargoToml.package.name;
            version = "${cargoToml.package.version}-${rev}";
            src = ./.;
            strictDeps = true;
            buildInputs = [ pkgs.openssl ];
            nativeBuildInputs = [
              dioxus-cli
              rustPlatform
              wasm-bindgen-cli
              postgres
              pkgs.binaryen
              pkgs.pkg-config
            ];
            buildPhase = ''
              export VCS_REF="${build_env.VCS_REF}"
              export BUILD_DATE="${build_env.BUILD_DATE}"
              export NO_DOWNLOADS=1
              ln -s ${nodePackages}/node_modules ./node_modules
              ${tailwindcss}/bin/tailwindcss -i ./input.css -o ./assets/tailwind.css
              ./node_modules/.bin/rollup --config rollup.config.mjs
              dx --version
              dx build --release --verbose
            '';
            installPhase = ''
              mkdir -p $out
              cp -r target/dx/$pname/release/web $out/bin
            '';
            cargoLock.lockFile = ./Cargo.lock;
            cargoLock.outputHashes = {
              # "const-serialize-0.7.0-rc.2" = "sha256-G2M0SyCWitPORvI3IeR2juuzLn1cOLhzbH6Y9lq71I8=";
              # "const-serialize-0.7.0-rc.2" = pkgs.lib.fakeHash;
            };
            meta.mainProgram = "penguin_nurse";
          };

        test_module = pkgs.nixosTest {
          name = "penguin-nurse-test";
          nodes.machine =
            { ... }:
            {
              imports = [
                self.nixosModules.default
              ];
              services.penguin-nurse = {
                enable = true;
                port = 4000;
                secretsFile = builtins.toFile "penguin-nurse.env" ''
                  DATABASE_URL="postgresql://penguin_nurse:your_secure_password_here@localhost/penguin_nurse"
                '';
              };
              system.stateVersion = "24.11";

              services.postgresql = {
                enable = true;
                package = pkgs.postgresql_15;
                extensions = ps: [ ps.postgis ];
                initialScript = pkgs.writeText "init.psql" ''
                  CREATE DATABASE penguin_nurse;
                  CREATE USER penguin_nurse with encrypted password 'your_secure_password_here';
                  ALTER DATABASE penguin_nurse OWNER TO penguin_nurse;
                  ALTER USER penguin_nurse WITH SUPERUSER;
                '';
              };
            };

          testScript = ''
            machine.wait_for_unit("penguin-nurse.service")
            machine.wait_for_open_port(4000)
            machine.succeed("${pkgs.curl}/bin/curl --fail -v http://localhost:4000/_health")
          '';
        };

        port = 4000;
        postgres_port = 6201;

        devShell = devenv.lib.mkShell {
          inherit inputs pkgs;
          modules = [
            {
              packages = [
                rustPlatform
                pkgs-unstable.rust-analyzer
                wasm-bindgen-cli
                pkgs.binaryen
                nodejs
                pkgs.cargo-watch
                pkgs.sqlx-cli
                # pkgs.jq
                pkgs.openssl
                pkgs.prefetch-npm-deps
                dioxus-cli
                # pkgs.b3sum
                pkgs.diesel-cli
                pkgs.diesel-cli-ext
                postgres
                tailwindcss
                pkgs.watchman
              ];
              enterShell = ''
                # export DIOXUS_ASSET_ROOT="dist"
                export PORT="${toString port}"
                export BASE_URL="http://localhost:$PORT/"
                export DATABASE_URL="postgresql://penguin_nurse:your_secure_password_here@localhost:${toString postgres_port}/penguin_nurse"
              '';
              services.postgres = {
                enable = true;
                package = pkgs.postgresql_15.withPackages (ps: [ ps.postgis ]);
                listen_addresses = "127.0.0.1";
                port = postgres_port;
                initialDatabases = [ { name = "penguin_nurse"; } ];
                initialScript = ''
                  \c penguin_nurse;
                  CREATE USER penguin_nurse with encrypted password 'your_secure_password_here';
                  GRANT ALL PRIVILEGES ON DATABASE penguin_nurse TO penguin_nurse;
                  -- GRANT ALL ON SCHEMA public TO penguin_nurse;
                  ALTER USER penguin_nurse WITH SUPERUSER;
                '';
              };
            }
          ];
        };
      in
      {
        checks = {
          # brian-backend = backend.clippy;
          # frontend-bindgen = frontend.clippy;
          test_module = test_module;
        };
        packages = {
          devenv-up = devShell.config.procfileScript;
          # backend = backend.pkg;
          # frontend = frontend-bindgen;
          # combined = combined;
          default = combined;
        };
        devShells.default = devShell;
      }
    )
    // {
      nixosModules.default = import ./nix/module.nix { inherit self; };
    };
}
