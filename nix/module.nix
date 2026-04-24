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
    mkOption
    mkEnableOption
    mkIf
    ;

  cfg = config.services.penguin-nurse;

  system = pkgs.stdenv.hostPlatform.system;
  penguin-nurse = self.packages.${system}.default;
  lint = self.packages.${system}.lint;

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
      type = types.nullOr types.str;
      default = null;
      example = "/run/secrets/penguin-nurse.env";
      description = lib.mdDoc ''
        Optional path to an env file containing the secrets used by penguin-nurse.

        Might contain:
        - `OIDC_DISCOVERY_URL` - The URL to the OIDC.
        - `OIDC_CLIENT_ID` - The Client ID for the OIDC.
        - `OIDC_CLIENT_SECRET` - The Client secret for the OIDC.
        - `OIDC_AUTH_SCOPE` - "openid profile groups email" or similar.
      '';
    };
  };

  config = mkIf cfg.enable {
    environment.systemPackages = [ lint ];
    users.users.penguin_nurse = {
      isSystemUser = true;
      description = "Penguin Nurse user";
      group = "penguin_nurse";
      createHome = true;
      home = "${cfg.data_dir}";
    };

    users.groups.penguin_nurse = { };

    systemd.services.penguin-nurse = {
      wantedBy = [ "multi-user.target" ];
      unit = {
        after = [ "postgresql.service" ];
        requires = [ "postgresql.service" ];
      };
      serviceConfig = {
        User = "penguin_nurse";
        ExecStart = "${lib.getExe penguin-nurse}";
        EnvironmentFile = cfg.secretsFile;
      };
      environment = {
        RUST_LOG = "info";
        PORT = toString cfg.port;
        BASE_URL = cfg.base_url;
        DATABASE_URL = "postgresql:///penguin_nurse?host=/var/run/postgresql";
      };
    };
  };
}
