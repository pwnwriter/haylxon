{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { nixpkgs, ... }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
    in
    {
      devShells = builtins.listToAttrs (map
        (system: {
          name = system;
          value =
            let
              pkgs = import nixpkgs { inherit system; };
              buildInputs = with pkgs; [
                pkg-config
                openssl
                libiconv
              ] ++ (if builtins.elem system [ "x86_64-darwin" "aarch64-darwin" ] then [ pkgs.darwin.apple_sdk.frameworks.Security ] else []);
            in
            {
              default = pkgs.mkShell {
                buildInputs = buildInputs;

                packages = with pkgs; [
                   cargo
                   rustc
                   rustfmt
                ];

              };
            };
        })
        systems);
    };
}
