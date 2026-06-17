{ pkgs, ... }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    openssl
  ];

  shellHook = ''
    export PATH="$PWD/target/debug:$PATH"

    echo "dev shell!"
  '';
}
