vim.pack.add({
  { src = "https://github.com/neovim/nvim-lspconfig", name = "lspconfig" }
})

vim.lsp.enable({
  "rust_analyzer",
})

return {}
