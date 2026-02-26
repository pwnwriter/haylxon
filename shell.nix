{ pkgs, ... }:
pkgs.mkShell {

  buildInputs = with pkgs; [ openssl ];

  shellHook = ''
    echo "You are inside the development shell!"
  '';
}
