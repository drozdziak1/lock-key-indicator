# import niv sources and the pinned nixpkgs
{ sources ? import ./nix/sources.nix, pkgs ? import sources.nixpkgs { } }:
let
  # import rust compiler
  rust = import ./nix/rust.nix { inherit sources; };

  # configure naersk to use our pinned rust compiler
  naersk = pkgs.callPackage sources.naersk {
    rustc = rust;
    cargo = rust;
  };

  common-deps = import ./nix/common-deps.nix { inherit sources rust; };

  buildInputs = common-deps.buildInputs;
  LIBCLANG_PATH = common-deps.LIBCLANG_PATH;

  # tell nix-build to ignore the `target` directory
  src = builtins.filterSource
    (path: type: type != "directory" || builtins.baseNameOf path != "target")
    ./.;
in naersk.buildPackage {
  inherit LIBCLANG_PATH buildInputs src;
  remapPathPrefix =
    true; # remove nix store references for a smaller output package
}
