{
  clippy,
  rustfmt,
  cargo-shear,
  rust-analyzer,
  callPackage,
}:
let
  mainPkg = callPackage ./default.nix { };
in
mainPkg.overrideAttrs (oa: {
  nativeBuildInputs = [
    # clippy
    # rustfmt
    # rust-analyzer
    # cargo-shear
  ] ++ (oa.nativeBuildInputs or [ ]);
})
