{ sources ? import ./nix/sources.nix }:
let
  pkgs = import sources.nixpkgs;
  common = import ./common.nix { inherit pkgs; };
in pkgs.mkShell {
  buildInputs = [] ++ common;
}
