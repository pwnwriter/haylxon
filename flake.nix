{
  inputs =
    {
      nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    };

  outputs = { self, nixpkgs, ... }@inputs:
    let
      system = "aarch64-darwin";
      pkgs = nixpkgs.legacyPackages.${system};
      nonRustDeps = [
        pkgs.libiconv
      ];
    in
    {
      devShells.${system}.default = pkgs.mkShell
        {
          shellHook = ''
            echo
            echo "🍎🍎 You are inside nix-shell"
            echo
            zsh
          '';
          buildInputs = nonRustDeps;
          packages = with pkgs; [ darwin.apple_sdk.frameworks.SystemConfiguration ];
        };
    };
}

