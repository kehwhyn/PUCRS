let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.05";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in

pkgs.mkShellNoCC {
  packages = with pkgs; [
    jdk17
  ];

  shellHook = ''
    echo "Welcome to the Advanced Algorithms Assigments dev environment!"
    echo "How to run:"
    echo "javac Main.java && java Main"
  '';
}
