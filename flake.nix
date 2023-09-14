{
  description = "cursive-spinner-view dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    # Systems supported
    systems = [
      "x86_64-linux" # 64-bit Intel/AMD Linux
      "aarch64-linux" # 64-bit ARM Linux
      "x86_64-darwin" # 64-bit Intel macOS
      "aarch64-darwin" # 64-bit ARM macOS
    ];
    eachSystem = with nixpkgs.lib; (
      f: foldAttrs mergeAttrs { } 
        (map (s: mapAttrs (_: v: { ${s} = v; }) (f s)) systems)
    );
  in eachSystem (system:
  let
    pkgs = import nixpkgs { inherit system; };

    devShellInputs = with pkgs; [
      rustc
      cargo
      clippy
      rustfmt
      gcc
      ncurses
      pkg-config
      # Helix code editor
      helix
      # Nix LSP for Helix
      nil
      # Toml LSP
      taplo
      # Rust LSP
      rust-analyzer
      # Fish shell
      fish
      zellij
    ];
  in
  {
    devShells = {
      default = pkgs.mkShell {
        packages = devShellInputs;

        shellHook = ''
          export EDITOR=hx
          # pkg-config for ncurses
          export PKG_CONFIG_PATH="${pkgs.ncurses.dev}/lib/pkgconfig"
          # zellij session
          exec zellij --session cursive-spinner-view-dev \
            --layout dev-layout.kdl
        '';
      };
    };
  });
}
