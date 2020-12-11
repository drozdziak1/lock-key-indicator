{pkgs}:
with pkgs; {
  buildInputs = [atk cairo clang gdk-pixbuf gtk3 libappindicator llvm llvmPackages.libclang pango pkgconfig];
  LIBCLANG_PATH = llvmPackages.libclang.outPath + "/lib";
}
