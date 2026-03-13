{
  description = "Pathfinding, flow, and graph algorithms";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      pathfinding = pkgs.rustPlatform.buildRustPackage {
        pname = "pathfinding";
        version = "4.15.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
        # Tests require network or special setup; run them in a separate check
        doCheck = true;
      };
    in
    {
      packages.${system} = {
        pathfinding = pathfinding;
        default = pathfinding;
      };

      checks.${system} = {
        pathfinding = pathfinding;
        devShell = self.devShells.${system}.default;
      };

      devShells.${system}.default = pkgs.mkShell {
        inputsFrom = [ pathfinding ];
        packages = with pkgs; [
          cargo
          rustc
          clippy
          rustfmt
        ];
      };
    };
}
