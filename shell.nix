{ sources ? import ./nix/sources.nix }:
let
  pkgs = import sources.nixpkgs {};
  common = import ./nix/common-deps.nix {};
in pkgs.mkShell {
  buildInputs = [] ++ (common.buildInputs);
  LIBCLANG_PATH = common.LIBCLANG_PATH;
}
