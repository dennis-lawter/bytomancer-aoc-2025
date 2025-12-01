{
  description = "Bytomancer's Advent of Code Nix Flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    feovim.url = "github:breuerfelix/feovim";
  };

  outputs = { self, nixpkgs, feovim, ... }:
  let
    system = "x86_64-linux";

    pkgs = import nixpkgs {
      inherit system;
      overlays = [ feovim.overlay ];
    };
  in
  {
    devShells.${system}.default = pkgs.mkShell {
      RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      LIBCLANG_PATH = "${pkgs.llvmPackages_18.libclang.lib}/lib";

      LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
        libGL
        xorg.libXrandr
        xorg.libXinerama
        xorg.libXcursor
        xorg.libXi
      ];

      packages = with pkgs; [
        bacon
        zellij
        neovim
        rustc
        cargo
        openssl.dev
        pkg-config
        raylib
        cmake
        clang
        glfw
        libxrandr
        libxinerama
        libxcursor
        libxi
      ];
    };
  };
}
