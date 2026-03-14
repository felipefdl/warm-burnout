local M = {}

-- Neovim's nvim_set_hl does not support 8-digit alpha hex (#rrggbbaa).
-- Pre-blend any alpha hex value against the editor background to produce
-- an opaque 6-digit hex that Neovim can use.
local function hex_to_rgb(hex)
  hex = hex:gsub("^#", "")
  return tonumber(hex:sub(1, 2), 16), tonumber(hex:sub(3, 4), 16), tonumber(hex:sub(5, 6), 16)
end

local function blend_alpha(hex, bg_hex)
  local raw = hex:gsub("^#", "")
  if #raw ~= 8 then
    return hex
  end
  local r, g, b = hex_to_rgb(raw)
  local a = tonumber(raw:sub(7, 8), 16) / 255
  local br, bg_c, bb = hex_to_rgb(bg_hex)
  local nr = math.floor(br + (r - br) * a + 0.5)
  local ng = math.floor(bg_c + (g - bg_c) * a + 0.5)
  local nb = math.floor(bb + (b - bb) * a + 0.5)
  return string.format("#%02x%02x%02x", nr, ng, nb)
end

local function resolve_palette(p)
  local bg = p.bg
  local resolved = {}
  for k, v in pairs(p) do
    if type(v) == "string" and v:match("^#%x%x%x%x%x%x%x%x$") then
      resolved[k] = blend_alpha(v, bg)
    else
      resolved[k] = v
    end
  end
  return resolved
end

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

  local p = resolve_palette(palette[variant])
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
