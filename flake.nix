{
  description = "RustFetch — A blazing-fast, memory-safe system fetch tool written in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "rustfetch";
            version = "0.1.4";
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
            postInstall = ''
              install -Dm644 docs/rustfetch.1 "$out/share/man/man1/rustfetch.1"
              ln -sf rustfetch.1 "$out/share/man/man1/rfetch.1"
            '';
            meta = with pkgs.lib; {
              description = "Blazing-fast, memory-safe system fetch tool written in Rust";
              homepage = "https://github.com/MehmetCanWT/rustfetch";
              license = licenses.mit;
              platforms = platforms.linux;
              mainProgram = "rustfetch";
            };
          };
        });
    };
}
