{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:
{
  # languages.rust.enable = true;

  # packages = with pkgs; [
  # ];

  # services.postgres = {
  #   enable = true;
  # createDatabase = true;
  # initialDatabases = [
  #   {
  #     name = "cdl_stats";
  #     initialSQL = ''
  #       CREATE TABLE players (
  #         tag VARCHAR PRIMARY KEY,
  #         kd REAL NOT NULL,
  #         hp_kd REAL NOT NULL,
  #         hp_k_10m REAL NOT NULL,
  #         snd_kd REAL NOT NULL,
  #         ovl_kd REAL NOT NULL,
  #         ovl_k_10m REAL NOT NULL,
  #         created_at TIMESTAMPTZ DEFAULT NOW()
  #       );
  #     '';
  #   }
  # ];
  # listen_addresses = "127.0.0.1";
  # port = 5432;
  # };
}
