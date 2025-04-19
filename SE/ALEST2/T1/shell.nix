let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.05";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in

pkgs.mkShellNoCC {
  packages = with pkgs; [
    nim
    gcc
    ruby
    haxe
    rustc
    jdk17
    python311
  ];

  shellHook = ''
    echo "Welcome to the IP Filter dev environment!"
    echo "Compile options:"
    echo "haxe --main Main.hx --interp"
    echo "javac Main.java && java Main <file_path or directory_path to run all>"
    echo "nim c -r main.nim <file_path or directory_path to run all>"
    echo "python main.py <-f file_path or -d directory_path to run all>"
    echo "ruby main.rb <file_path or directory_path to run all>"
    echo "rustc main.rs && ./main"
  '';
}
