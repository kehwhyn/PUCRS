let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.05";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in

pkgs.mkShellNoCC {
  packages = with pkgs; [
    gcc
    rustc
    python311
  ];

  shellHook = ''
    echo "Welcome to the Maze Solver dev environment!"
    echo "Compile options:"
    echo "python main.py"
    echo "rustc main.rs && ./main"
  '';
}
