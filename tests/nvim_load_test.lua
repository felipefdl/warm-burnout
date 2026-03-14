-- Headless Neovim colorscheme load test.
-- Usage: nvim --headless -u NONE --cmd "set rtp+=nvim" -l tests/nvim_load_test.lua <variant>
-- Exits 0 on success, 1 on failure.

local variant = arg[1] or "dark"
local colorscheme = "warm-burnout-" .. variant

local ok, err = pcall(vim.cmd.colorscheme, colorscheme)
if not ok then
  io.stderr:write("FAIL: " .. colorscheme .. ": " .. tostring(err) .. "\n")
  os.exit(1)
end

if vim.g.colors_name ~= colorscheme then
  io.stderr:write(
    "FAIL: expected colors_name='" .. colorscheme .. "', got '" .. tostring(vim.g.colors_name) .. "'\n"
  )
  os.exit(1)
end

-- Verify a sampling of critical highlight groups actually got set
local critical_groups = {
  "Normal", "Comment", "Keyword", "Function", "Type", "String",
  "DiagnosticError", "TelescopeNormal", "GitSignsAdd",
}
for _, group in ipairs(critical_groups) do
  local hl = vim.api.nvim_get_hl(0, { name = group })
  if not hl.fg and not hl.bg and not hl.link then
    io.stderr:write("FAIL: highlight group '" .. group .. "' has no fg, bg, or link\n")
    os.exit(1)
  end
end

io.stdout:write("OK:" .. colorscheme .. "\n")
