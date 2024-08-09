{
  rust-analyzer,
  callPackage,
}:
let
  mainPkg = callPackage ./default.nix { };
in
mainPkg.overrideAttrs (oa: {
  nativeBuildInputs = [
    rust-analyzer
  ] ++ (oa.nativeBuildInputs or [ ]);
})
