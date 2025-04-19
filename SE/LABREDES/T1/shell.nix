let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.05";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in

pkgs.mkShellNoCC {
  packages = with pkgs; [
    python311
  ];

  shellHook = ''
    echo "Welcome to the quess game dev environment!"
    echo "How to run:"
    echo "python main.py"
  '';
}
