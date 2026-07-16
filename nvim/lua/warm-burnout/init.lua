local M = {}

function M.load(variant)
  variant = variant or "dark"

  if vim.g.colors_name then
    vim.cmd("hi clear")
  end

  vim.o.termguicolors = true
  vim.o.background = (variant == "light") and "light" or "dark"
  vim.g.colors_name = "warm-burnout-" .. variant

  local palette = require("warm-burnout.palette")
  local highlights = require("warm-burnout.highlights")
  local terminal = require("warm-burnout.terminal")

  local p = palette.resolve(palette[variant])
  local groups = highlights.get(p)

  for group, opts in pairs(groups) do
    vim.api.nvim_set_hl(0, group, opts)
  end

  terminal.setup(p, variant)
end

function M.setup(opts)
  opts = opts or {}
  local variant = opts.variant or "dark"
  M.load(variant)
end

return M
