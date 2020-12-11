{ sources ? import ./sources.nix
, rust ? import ./rust.nix { inherit sources; } }:
let pkgs = import sources.nixpkgs { };
    our-rust = rust;
in with pkgs; {
  buildInputs = [
    atk
    cairo
    clang
    gdk-pixbuf
    gtk3
    libappindicator
    llvm
    llvmPackages.libclang
    our-rust
    pango
    pkgconfig
  ];
  LIBCLANG_PATH = llvmPackages.libclang.outPath + "/lib";
}
