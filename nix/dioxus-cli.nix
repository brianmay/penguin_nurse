{
  lib,
  # stdenv,
  # fetchCrate,
  fetchFromGitHub,
  rustPlatform,
  pkg-config,
  rustfmt,
  cacert,
  openssl,
  nix-update-script,
  testers,
  dioxus-cli,
}:

rustPlatform.buildRustPackage rec {
  pname = "dioxus-cli";
  version = "0.7.1";

  # src = fetchCrate {
  #   inherit pname version;
  #   hash = "sha256-xt/DJhcZz3TZLodfJTaFE2cBX3hedo+typHM5UezS94=c";
  # };

  src = fetchFromGitHub {
    owner = "DioxusLabs";
    repo = "dioxus";
    rev = "v${version}";
    hash = "sha256-EzfuD3rWVuomyzqSv4b3SVA6MmQiWAeePbdfXEvkiRk=";
  };

  cargoHash = "sha256-cZe+p4pnXgkOvKxNSSylQzbQcBklqJuzf96YzsI3XX4=";
  # cargoHash = lib.fakeHash;

  cargoBuildFlags = [
    "-p"
    "dioxus-cli"
  ];
  # Tests not working, trying build dioxus-examples with "do-downloads" flag, no
  # idea how to fix.
  doCheck = false;
  cargoInstallFlags = [
    "-p"
    "dioxus-cli"
  ];

  buildFeatures = [
    "no-downloads"
    # "optimizations"
  ];

  nativeBuildInputs = [
    pkg-config
    cacert
  ];

  buildInputs = [
    openssl
  ];

  OPENSSL_NO_VENDOR = 1;

  postPatch = ''
    # sed -i '/dioxus-examples/d' Cargo.toml
    # wasm-opt-sys build.rs tries to verify C++17 support, but the check appears to be faulty.
    # substituteInPlace $cargoDepsCopy/wasm-opt-sys-*/build.rs \
    #   --replace-fail 'check_cxx17_support()?;' '// check_cxx17_support()?;'
  '';

  nativeCheckInputs = [ rustfmt ];

  checkFlags = [
    "-pdioxus-cli"
    # requires network access
    "--skip=serve::proxy::test"
    "--skip=wasm_bindgen::test"
  ];

  passthru = {
    updateScript = nix-update-script { };
    tests.version = testers.testVersion { package = dioxus-cli; };
  };

  meta = with lib; {
    homepage = "https://dioxuslabs.com";
    description = "CLI tool for developing, testing, and publishing Dioxus apps";
    changelog = "https://github.com/DioxusLabs/dioxus/releases";
    license = with licenses; [
      mit
      asl20
    ];
    maintainers = with maintainers; [
      xanderio
      cathalmullan
    ];
    mainProgram = "dx";
  };
}
