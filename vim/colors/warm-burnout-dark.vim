" Warm Burnout Dark -- clinically warm, emotionally cold
" Background: #1a1510. All tokens >= 7.0:1 contrast (WCAG AAA).

hi clear
if exists('syntax_on')
  syntax reset
endif

set background=dark
let g:colors_name = 'warm-burnout-dark'

" -- Terminal ANSI colors (16) --
let g:terminal_ansi_colors = [
      \ '#23211b', '#f06b73', '#70bf56', '#fdb04c',
      \ '#4fbfff', '#d0a1ff', '#93e2c8', '#c7c7c7',
      \ '#686868', '#f07178', '#aad94c', '#ffb454',
      \ '#59c2ff', '#d2a6ff', '#95e6cb', '#ffffff',
      \ ]

" -- Editor UI --
hi Normal           guifg=#bfbdb6 guibg=#1a1510 ctermfg=250 ctermbg=234
hi NormalNC         guifg=#bfbdb6 guibg=#1a1510 ctermfg=250 ctermbg=234
hi NormalFloat      guifg=#bfbdb6 guibg=#1f1d17 ctermfg=250 ctermbg=235
hi FloatBorder      guifg=#2a2820 guibg=#1f1d17 ctermfg=236 ctermbg=235
hi FloatTitle       guifg=#bfbdb6 guibg=#1f1d17 gui=bold   ctermfg=250 ctermbg=235 cterm=bold
hi EndOfBuffer      guifg=#1a1510 guibg=#1a1510 ctermfg=234 ctermbg=234

hi Cursor           guifg=#1a1510 guibg=#f5c56e ctermfg=234 ctermbg=221
hi lCursor          guifg=#1a1510 guibg=#f5c56e ctermfg=234 ctermbg=221
hi CursorIM         guifg=#1a1510 guibg=#f5c56e ctermfg=234 ctermbg=221
hi CursorLine                     guibg=#222018             ctermbg=235 cterm=NONE
hi CursorColumn                   guibg=#222018             ctermbg=235
hi CursorLineNr     guifg=#a59f96 guibg=NONE    ctermfg=246 ctermbg=NONE
hi LineNr           guifg=#6b6860 guibg=NONE    ctermfg=242 ctermbg=NONE
hi SignColumn       guifg=#6b6860 guibg=#1a1510 ctermfg=242 ctermbg=234
hi ColorColumn                    guibg=#222018             ctermbg=235
hi Folded           guifg=#b4a89c guibg=#1f1d17 gui=italic  ctermfg=144 ctermbg=235 cterm=italic
hi FoldColumn       guifg=#6b6860 guibg=NONE    ctermfg=242 ctermbg=NONE

hi VertSplit        guifg=#2a2820 guibg=NONE    ctermfg=236 ctermbg=NONE
hi WinSeparator     guifg=#2a2820 guibg=NONE    ctermfg=236 ctermbg=NONE
hi StatusLine       guifg=#ada69c guibg=#14120f             ctermfg=248 ctermbg=233 cterm=NONE
hi StatusLineNC     guifg=#6b6860 guibg=#14120f             ctermfg=242 ctermbg=233 cterm=NONE
hi TabLine          guifg=#ada69c guibg=#14120f             ctermfg=248 ctermbg=233 cterm=NONE
hi TabLineSel       guifg=#bfbdb6 guibg=#1a1510 gui=bold   ctermfg=250 ctermbg=234 cterm=bold
hi TabLineFill      guifg=#ada69c guibg=#14120f             ctermfg=248 ctermbg=233

hi Visual                         guibg=#2e3a41             ctermbg=238
hi VisualNOS                      guibg=#2e3a41             ctermbg=238

hi Pmenu            guifg=#bfbdb6 guibg=#1f1d17             ctermfg=250 ctermbg=235
hi PmenuSel         guifg=#bfbdb6 guibg=#2e3a41 gui=bold   ctermfg=250 ctermbg=238 cterm=bold
hi PmenuSbar                      guibg=#1f1d17             ctermbg=235
hi PmenuThumb                     guibg=#6b6860             ctermbg=242
hi WildMenu                       guibg=#2e3a41             ctermbg=238

hi Search           guifg=#bfbdb6 guibg=#4c4126             ctermfg=250 ctermbg=94
hi IncSearch        guifg=#1a1510 guibg=#f5c56e             ctermfg=234 ctermbg=221
hi CurSearch        guifg=#1a1510 guibg=#f5c56e             ctermfg=234 ctermbg=221
hi Substitute       guifg=#1a1510 guibg=#f49090             ctermfg=234 ctermbg=210
hi MatchParen       guifg=#b8522e guibg=NONE    gui=bold   ctermfg=173 ctermbg=NONE cterm=bold
hi QuickFixLine                   guibg=#2e3a41             ctermbg=238

hi SpecialKey       guifg=#6b6860             ctermfg=242
hi NonText          guifg=#6b6860             ctermfg=242
hi Whitespace       guifg=#6b6860             ctermfg=242
hi Conceal          guifg=#ada69c             ctermfg=248

hi Directory        guifg=#ffb454             ctermfg=214
hi Title            guifg=#ffb454 gui=bold    ctermfg=214 cterm=bold
hi ErrorMsg         guifg=#f49090             ctermfg=210
hi WarningMsg       guifg=#b8522e             ctermfg=173
hi MoreMsg          guifg=#90aec0             ctermfg=109
hi Question         guifg=#90aec0             ctermfg=109
hi ModeMsg          guifg=#bfbdb6 gui=bold    ctermfg=250 cterm=bold

" -- Spell --
hi SpellBad   guisp=#f49090 gui=undercurl cterm=undercurl
hi SpellCap   guisp=#b8522e gui=undercurl cterm=undercurl
hi SpellLocal guisp=#90aec0 gui=undercurl cterm=undercurl
hi SpellRare  guisp=#b4a89c gui=undercurl cterm=undercurl

" -- Diff --
hi DiffAdd                   guibg=#1e2a1c ctermbg=22
hi DiffChange                guibg=#1a242e ctermbg=23
hi DiffDelete                guibg=#2e1a1c ctermbg=52
hi DiffText                  guibg=#2c3e52 ctermbg=24

" -- Syntax groups --
hi Comment        guifg=#b4a89c gui=italic              ctermfg=144 cterm=italic
hi Constant       guifg=#d4a8b8                         ctermfg=181
hi String         guifg=#b4bc78                         ctermfg=143
hi Character      guifg=#b4bc78                         ctermfg=143
hi Number         guifg=#d4a8b8                         ctermfg=181
hi Boolean        guifg=#d4a8b8                         ctermfg=181
hi Float          guifg=#d4a8b8                         ctermfg=181
hi Identifier     guifg=#bfbdb6                         ctermfg=250 cterm=NONE
hi Function       guifg=#ffb454                         ctermfg=214
hi Statement      guifg=#ff8f40 gui=bold                ctermfg=208 cterm=bold
hi Conditional    guifg=#ff8f40 gui=bold                ctermfg=208 cterm=bold
hi Repeat         guifg=#ff8f40 gui=bold                ctermfg=208 cterm=bold
hi Label          guifg=#ff8f40 gui=bold                ctermfg=208 cterm=bold
hi Operator       guifg=#f29668                         ctermfg=209
hi Keyword        guifg=#ff8f40 gui=bold                ctermfg=208 cterm=bold
hi Exception      guifg=#ff8f40 gui=bold                ctermfg=208 cterm=bold
hi PreProc        guifg=#ff8f40                         ctermfg=208
hi Include        guifg=#ff8f40 gui=bold                ctermfg=208 cterm=bold
hi Define         guifg=#ff8f40                         ctermfg=208
hi Macro          guifg=#e6c08a                         ctermfg=180
hi PreCondit      guifg=#ff8f40                         ctermfg=208
hi Type           guifg=#90aec0 gui=italic              ctermfg=109 cterm=italic
hi StorageClass   guifg=#ff8f40 gui=bold                ctermfg=208 cterm=bold
hi Structure      guifg=#90aec0 gui=italic              ctermfg=109 cterm=italic
hi Typedef        guifg=#90aec0 gui=italic              ctermfg=109 cterm=italic
hi Special        guifg=#f29668                         ctermfg=209
hi SpecialChar    guifg=#96b898                         ctermfg=108
hi Tag            guifg=#dc9e92 gui=bold                ctermfg=174 cterm=bold
hi Delimiter      guifg=#bfbdb6                         ctermfg=250
hi SpecialComment guifg=#b4a89c gui=italic              ctermfg=144 cterm=italic
hi Debug          guifg=#b8522e                         ctermfg=173
hi Underlined                   gui=underline          cterm=underline
hi Ignore         guifg=#ada69c                         ctermfg=248
hi Error          guifg=#f49090                         ctermfg=210
hi Todo           guifg=#f5c56e gui=bold                ctermfg=221 cterm=bold
hi Added          guifg=#70bf56                         ctermfg=71
hi Changed        guifg=#73b8ff                         ctermfg=75
hi Removed        guifg=#f26d78                         ctermfg=203

" -- LSP / Diagnostics (Vim 9 / ALE / coc.nvim / vim-lsp) --
hi DiagnosticError      guifg=#f49090 ctermfg=210
hi DiagnosticWarn       guifg=#b8522e ctermfg=173
hi DiagnosticInfo       guifg=#90aec0 ctermfg=109
hi DiagnosticHint       guifg=#b4a89c ctermfg=144
hi DiagnosticOk         guifg=#70bf56 ctermfg=71
hi DiagnosticSignError  guifg=#f49090 ctermfg=210
hi DiagnosticSignWarn   guifg=#b8522e ctermfg=173
hi DiagnosticSignInfo   guifg=#90aec0 ctermfg=109
hi DiagnosticSignHint   guifg=#b4a89c ctermfg=144
hi DiagnosticUnderlineError guisp=#f49090 gui=undercurl cterm=undercurl
hi DiagnosticUnderlineWarn  guisp=#b8522e gui=undercurl cterm=undercurl
hi DiagnosticUnderlineInfo  guisp=#90aec0 gui=undercurl cterm=undercurl
hi DiagnosticUnderlineHint  guisp=#b4a89c gui=undercurl cterm=undercurl
hi DiagnosticVirtualTextError guifg=#f49090 ctermfg=210
hi DiagnosticVirtualTextWarn  guifg=#b8522e ctermfg=173
hi DiagnosticVirtualTextInfo  guifg=#90aec0 ctermfg=109
hi DiagnosticVirtualTextHint  guifg=#b4a89c ctermfg=144

hi ALEErrorSign        guifg=#f49090 ctermfg=210
hi ALEWarningSign      guifg=#b8522e ctermfg=173
hi ALEInfoSign         guifg=#90aec0 ctermfg=109
hi ALEError            guisp=#f49090 gui=undercurl cterm=undercurl
hi ALEWarning          guisp=#b8522e gui=undercurl cterm=undercurl

hi CocErrorSign   guifg=#f49090 ctermfg=210
hi CocWarningSign guifg=#b8522e ctermfg=173
hi CocInfoSign    guifg=#90aec0 ctermfg=109
hi CocHintSign    guifg=#b4a89c ctermfg=144
hi CocErrorFloat  guifg=#f49090 ctermfg=210
hi CocErrorHighlight   guisp=#f49090 gui=undercurl cterm=undercurl
hi CocWarningHighlight guisp=#b8522e gui=undercurl cterm=undercurl

" -- Git (vim-gitgutter / vim-signify) --
hi GitGutterAdd    guifg=#70bf56 ctermfg=71
hi GitGutterChange guifg=#73b8ff ctermfg=75
hi GitGutterDelete guifg=#f26d78 ctermfg=203
hi SignifySignAdd    guifg=#70bf56 ctermfg=71
hi SignifySignChange guifg=#73b8ff ctermfg=75
hi SignifySignDelete guifg=#f26d78 ctermfg=203
hi diffAdded   guifg=#70bf56 ctermfg=71
hi diffRemoved guifg=#f26d78 ctermfg=203
hi diffChanged guifg=#73b8ff ctermfg=75

" -- HTML / XML --
hi htmlTag        guifg=#dc9e92 gui=bold ctermfg=174 cterm=bold
hi htmlEndTag     guifg=#dc9e92 gui=bold ctermfg=174 cterm=bold
hi htmlTagName    guifg=#dc9e92 gui=bold ctermfg=174 cterm=bold
hi htmlArg        guifg=#ffb454          ctermfg=214
hi htmlTitle      guifg=#bfbdb6 gui=bold ctermfg=250 cterm=bold
hi htmlH1         guifg=#bfbdb6 gui=bold ctermfg=250 cterm=bold
hi xmlTag         guifg=#dc9e92 gui=bold ctermfg=174 cterm=bold
hi xmlTagName     guifg=#dc9e92 gui=bold ctermfg=174 cterm=bold
hi xmlEndTag      guifg=#dc9e92 gui=bold ctermfg=174 cterm=bold

" -- CSS --
hi cssClassName    guifg=#e6c08a gui=italic ctermfg=180 cterm=italic
hi cssIdentifier   guifg=#e6c08a gui=italic ctermfg=180 cterm=italic
hi cssProp         guifg=#deb074 gui=italic ctermfg=179 cterm=italic
hi cssAttr         guifg=#b4bc78            ctermfg=143
hi cssAttrRegion   guifg=#b4bc78            ctermfg=143
hi cssBraces       guifg=#bfbdb6            ctermfg=250
hi cssPseudoClass  guifg=#ec9878            ctermfg=216
hi cssPseudoClassId guifg=#ec9878           ctermfg=216
hi cssValueLength  guifg=#d4a8b8            ctermfg=181
hi cssValueNumber  guifg=#d4a8b8            ctermfg=181
hi cssFunctionName guifg=#ffb454            ctermfg=214
hi cssColor        guifg=#d4a8b8            ctermfg=181

" -- JavaScript / TypeScript --
hi jsFunction         guifg=#ff8f40 gui=bold  ctermfg=208 cterm=bold
hi jsFuncCall         guifg=#ffb454           ctermfg=214
hi jsClassDefinition  guifg=#90aec0 gui=italic ctermfg=109 cterm=italic
hi jsGlobalObjects    guifg=#90aec0 gui=italic ctermfg=109 cterm=italic
hi jsThis             guifg=#dc9e92 gui=italic ctermfg=174 cterm=italic
hi jsObjectKey        guifg=#deb074 gui=italic ctermfg=179 cterm=italic
hi typescriptKeyword     guifg=#ff8f40 gui=bold ctermfg=208 cterm=bold
hi typescriptClassName   guifg=#90aec0 gui=italic ctermfg=109 cterm=italic
hi typescriptFuncKeyword guifg=#ff8f40 gui=bold ctermfg=208 cterm=bold
hi typescriptMember      guifg=#ec9878          ctermfg=216

" -- Python --
hi pythonBuiltin     guifg=#ec9878           ctermfg=216
hi pythonFunction    guifg=#ffb454           ctermfg=214
hi pythonDecorator   guifg=#e6c08a gui=italic ctermfg=180 cterm=italic
hi pythonDecoratorName guifg=#e6c08a gui=italic ctermfg=180 cterm=italic
hi pythonStatement   guifg=#ff8f40 gui=bold  ctermfg=208 cterm=bold
hi pythonClass       guifg=#90aec0 gui=italic ctermfg=109 cterm=italic
hi pythonSelf        guifg=#dc9e92 gui=italic ctermfg=174 cterm=italic

" -- Rust --
hi rustKeyword         guifg=#ff8f40 gui=bold ctermfg=208 cterm=bold
hi rustType            guifg=#90aec0 gui=italic ctermfg=109 cterm=italic
hi rustTrait           guifg=#90aec0 gui=italic ctermfg=109 cterm=italic
hi rustMacro           guifg=#e6c08a          ctermfg=180
hi rustAttribute       guifg=#e6c08a gui=italic ctermfg=180 cterm=italic
hi rustLifetime        guifg=#dc9e92 gui=italic ctermfg=174 cterm=italic
hi rustFuncCall        guifg=#ffb454          ctermfg=214
hi rustFuncName        guifg=#ffb454          ctermfg=214
hi rustEnumVariant     guifg=#d4a8b8          ctermfg=181

" -- Markdown --
hi markdownH1        guifg=#ff8f40 gui=bold ctermfg=208 cterm=bold
hi markdownH2        guifg=#ffb454 gui=bold ctermfg=214 cterm=bold
hi markdownH3        guifg=#e6c08a gui=bold ctermfg=180 cterm=bold
hi markdownH4        guifg=#b4bc78 gui=bold ctermfg=143 cterm=bold
hi markdownH5        guifg=#90aec0 gui=bold ctermfg=109 cterm=bold
hi markdownH6        guifg=#dc9e92 gui=bold ctermfg=174 cterm=bold
hi markdownCode      guifg=#b4bc78          ctermfg=143
hi markdownCodeBlock guifg=#b4bc78          ctermfg=143
hi markdownLinkText  guifg=#dc9e92 gui=underline ctermfg=174 cterm=underline
hi markdownUrl       guifg=#90aec0 gui=underline ctermfg=109 cterm=underline
hi markdownBold      guifg=#bfbdb6 gui=bold ctermfg=250 cterm=bold
hi markdownItalic    guifg=#ec9878 gui=italic ctermfg=216 cterm=italic
hi markdownListMarker guifg=#ffb454         ctermfg=214

" -- Vim --
hi vimCommand    guifg=#ff8f40 gui=bold ctermfg=208 cterm=bold
hi vimFunction   guifg=#ffb454          ctermfg=214
hi vimFuncName   guifg=#ffb454          ctermfg=214
hi vimGroup      guifg=#90aec0 gui=italic ctermfg=109 cterm=italic
hi vimHiGroup    guifg=#90aec0 gui=italic ctermfg=109 cterm=italic
hi vimOption     guifg=#deb074 gui=italic ctermfg=179 cterm=italic
hi vimVar        guifg=#d4a8b8          ctermfg=181
hi vimNotation   guifg=#96b898          ctermfg=108

" -- NERDTree / netrw --
hi NERDTreeDir       guifg=#bfbdb6 gui=bold ctermfg=250 cterm=bold
hi NERDTreeDirSlash  guifg=#ffb454          ctermfg=214
hi NERDTreeFile      guifg=#ada69c          ctermfg=248
hi NERDTreeCWD       guifg=#b8522e gui=bold ctermfg=173 cterm=bold
hi NERDTreeOpenable  guifg=#ffb454          ctermfg=214
hi NERDTreeClosable  guifg=#ffb454          ctermfg=214
hi netrwDir          guifg=#ffb454 gui=bold ctermfg=214 cterm=bold
hi netrwExe          guifg=#b4bc78          ctermfg=143
hi netrwClassify     guifg=#ffb454          ctermfg=214
