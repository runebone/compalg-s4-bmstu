{ pkgs ? import <nixpkgs> {} }:

with { LUA_CONFIG = "$HOME/.config/nvim/init.lua"; };

# I haven't yet found out ~~a better~~ any other solution to use my default vim
# config in nix-shell (without using nix-flakes)
with {
  my-neovim = pkgs.neovim.override {
    vimAlias = true;

    configure = {
      packages.myVimPackage = with pkgs.vimPlugins; {
        start = [
          vim-nix
          vim-surround
          vim-commentary
          vim-airline
          vim-css-color
          srcery-vim

          # Other plugins
          vim-go
          vim-glsl

          indentLine

          nvim-tree-lua
          nvim-web-devicons

          telescope-nvim
          {
            plugin = (nvim-treesitter.withPlugins (
              plugins: with plugins; [
                c
                cmake
                cpp
                glsl
                go
                javascript
                latex
                lua
                make
                nix
                python
                rust
                typescript
                vim
              ]
              )
            );
          }
          nvim-lspconfig
          nvim-cmp
          cmp_luasnip
          cmp-nvim-lsp
          cmp-nvim-lua
          luasnip
        ];
      };
      customRC = ''
        luafile ${LUA_CONFIG}
      '';
    };
  };
};

# Minimal shell for rust-nannou development
pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    cargo
    rustc

    xorg.libxcb
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi

    alsa-lib # Sound
    vulkan-loader

    # My packages
    rust-analyzer
    rnix-lsp
    my-neovim
    git
    devour
  ];

  LD_LIBRARY_PATH = "${pkgs.vulkan-loader}/lib";
}
