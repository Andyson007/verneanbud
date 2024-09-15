{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = [
    pkgs.sea-orm-cli
  ];
  shellHook = ''
    export DATABASE_URL=postgres://vern:vern@localhost:5432/verneanbud
    alias gen_entities="sea-orm-cli generate entity -o src/entities --database-url postgres://vern:vern@localhost:5432/verneanbud;"
  '';
}
