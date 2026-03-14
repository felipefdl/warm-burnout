local M = {}

function M.setup(p, variant)
  if variant == "dark" then
    vim.g.terminal_color_0 = "#23211b"
    vim.g.terminal_color_1 = "#f06b73"
    vim.g.terminal_color_2 = "#70bf56"
    vim.g.terminal_color_3 = "#fdb04c"
    vim.g.terminal_color_4 = "#4fbfff"
    vim.g.terminal_color_5 = "#d0a1ff"
    vim.g.terminal_color_6 = "#93e2c8"
    vim.g.terminal_color_7 = "#c7c7c7"
    vim.g.terminal_color_8 = "#686868"
    vim.g.terminal_color_9 = "#f07178"
    vim.g.terminal_color_10 = "#aad94c"
    vim.g.terminal_color_11 = "#ffb454"
    vim.g.terminal_color_12 = "#59c2ff"
    vim.g.terminal_color_13 = "#d2a6ff"
    vim.g.terminal_color_14 = "#95e6cb"
    vim.g.terminal_color_15 = "#ffffff"
  else
    vim.g.terminal_color_0 = "#3a3630"
    vim.g.terminal_color_1 = "#b82820"
    vim.g.terminal_color_2 = "#2d6a14"
    vim.g.terminal_color_3 = "#8a6000"
    vim.g.terminal_color_4 = "#2060a0"
    vim.g.terminal_color_5 = "#8a3090"
    vim.g.terminal_color_6 = "#146858"
    vim.g.terminal_color_7 = "#c0b8aa"
    vim.g.terminal_color_8 = "#686868"
    vim.g.terminal_color_9 = "#b82820"
    vim.g.terminal_color_10 = "#3a7a20"
    vim.g.terminal_color_11 = "#9a7008"
    vim.g.terminal_color_12 = "#2870b0"
    vim.g.terminal_color_13 = "#9a38a0"
    vim.g.terminal_color_14 = "#208870"
    vim.g.terminal_color_15 = "#FAF6F0"
  end
end

return M
