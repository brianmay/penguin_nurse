{ self }:
{
  lib,
  pkgs,
  config,
  ...
}:
let
  inherit (lib)
    types
    boolToString
    mkOption
    mkEnableOption
    mkIf
    ;

  cfg = config.services.penguin-nurse;

  system = pkgs.stdenv.system;
  penguin-nurse = self.packages.${system}.default;

in
# wrapper = pkgs.writeShellScriptBin "penguin-nurse" ''
#   export RUST_LOG=info
#   export PORT=${toString cfg.port}
#   exec "${penguin-nurse}/bin/server"
# '';
{
  options.services.penguin-nurse = {
    enable = mkEnableOption "penguin-nurse service";
    data_dir = mkOption {
      type = types.str;
      default = "/var/lib/penguin-nurse";
      description = lib.mdDoc ''
        The directory where penguin-nurse stores its home directory.
      '';
    };
    port = mkOption {
      type = types.int;
      default = 8080;
      description = lib.mdDoc ''
        The port on which the penguin-nurse service listens.
      '';
    };
    base_url = mkOption {
      type = types.str;
      default = "http://localhost:${toString cfg.port}";
      description = lib.mdDoc ''
        The external base URL of the penguin-nurse service.
        Used to generate the OIDC redirect URL. Not used if OIDC not configured.
      '';
    };
    secretsFile = mkOption {
      type = types.str;
      example = "/run/secrets/penguin-nurse.env";
      description = lib.mdDoc ''
        Path to an env file containing the secrets used by penguin-nurse.

        Must contain at least:
        - `DATABASE_URL` - The URL to the database.
      '';
    };
  };

  config = mkIf cfg.enable {
    users.users.penguin-nurse = {
      isSystemUser = true;
      description = "Robotica user";
      group = "penguin-nurse";
      createHome = true;
      home = "${cfg.data_dir}";
    };

    users.groups.penguin-nurse = { };

    systemd.services.penguin-nurse = {
      wantedBy = [ "multi-user.target" ];
      serviceConfig = {
        User = "penguin-nurse";
        ExecStart = "${penguin-nurse}/bin/server";
        EnvironmentFile = cfg.secretsFile;
      };
      environment = {
        RUST_LOG = "info";
        PORT = toString cfg.port;
        BASE_URL = cfg.base_url;
      };
    };
  };
}
