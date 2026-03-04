return {

  {
    "neovim/nvim-lspconfig",
    name = "lspconfig",
    event = { "BufReadPost", "BufNewFile" },
  },

  vim.lsp.enable({'rust_analyzer' }),
}
