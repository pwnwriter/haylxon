{ pkgs, ... }:
pkgs.mkShell {

  buildInputs =
    with pkgs;
    [ openssl ]
    ++ lib.optionals stdenv.isDarwin (
      with darwin.apple_sdk.frameworks;
      [
        Security
        CoreFoundation
        SystemConfiguration
      ]
    );

  shellHook = ''
    echo "You are inside the development shell!"
  '';
}
