local p = require("warm-burnout.palette").resolve(require("warm-burnout.palette").light)

return {
  normal = {
    a = { bg = p.accent, fg = p.bg, gui = "bold" },
    b = { bg = p.bg_highlight, fg = p.fg },
    c = { bg = p.bg, fg = p.fg },
  },
  insert = {
    a = { bg = p.added, fg = p.bg, gui = "bold" },
    b = { bg = p.bg_highlight, fg = p.fg },
    c = { bg = p.bg, fg = p.fg },
  },
  visual = {
    a = { bg = p.keyword, fg = p.bg, gui = "bold" },
    b = { bg = p.bg_highlight, fg = p.fg },
    c = { bg = p.bg, fg = p.fg },
  },
  replace = {
    a = { bg = p.error, fg = p.bg, gui = "bold" },
    b = { bg = p.bg_highlight, fg = p.fg },
    c = { bg = p.bg, fg = p.fg },
  },
  command = {
    a = { bg = p.cursor, fg = p.bg, gui = "bold" },
    b = { bg = p.bg_highlight, fg = p.fg },
    c = { bg = p.bg, fg = p.fg },
  },
  terminal = {
    a = { bg = p.info, fg = p.bg, gui = "bold" },
    b = { bg = p.bg_highlight, fg = p.fg },
    c = { bg = p.bg, fg = p.fg },
  },
  inactive = {
    a = { bg = p.bg, fg = p.comment, gui = "bold" },
    b = { bg = p.bg_highlight, fg = p.fg },
    c = { bg = p.bg, fg = p.fg },
  },
}
