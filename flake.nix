{
  description = "Dev shell for cdl-stats with a postgres db";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = [ pkgs.postgresql ];

        shellHook = ''
            export PGDATA=$PWD/.postgres/data
            export PGHOST=$PWD/.postgres
            export PGPORT=5432
            export PGDATABASE=cdl_stats

            if [ ! -d "$PGDATA" ]; then
                echo "Initializing PostgresSQL..."
                initdb --no-locale --encoding=UTF8

                pg_ctl start -l "$PWD/.postgres/postgres.log" -o "0k $PGHOST"

                createdb cdl_stats

                psql -d cdl_stats  <<SQL
                CREATE TABLE players (
                  tag VARCHAR PRIMARY KEY,
                  kd REAL NOT NULL,
                  hp_kd REAL NOT NULL,
                  hp_k_10m REAL NOT NULL,
                  snd_kd REAL NOT NULL,
                  ovl_kd REAL NOT NULL,
                  ovl_k_10m REAL NOT NULL
                  created_at TIMESTAMPTZ DEFAULT NOW()
                );

              SQL
              else
              pg_ctl start -l "$PWD/.postgres/postgres.log" -o "-k $PGHOST"
          fi

          stop_postgres() {
          pg_ctl stop
          }

          trap stop_postgres EXIT
        '';
      };
    };
}
