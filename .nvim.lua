-- nvcontrol project-local neovim config
-- rust-analyzer uses nightly toolchain for edition 2024 support

vim.g.rustaceanvim = {
  server = {
    settings = {
      ['rust-analyzer'] = {
        rustc = {
          source = 'discover',
        },
        cargo = {
          extraEnv = {
            RUSTUP_TOOLCHAIN = 'nightly',
          },
        },
      },
    },
  },
}
