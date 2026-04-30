" Warm Burnout Light -- sepia cream, zero halation
" Background: #F5EDE0. All tokens >= 4.5:1 contrast (WCAG AA).

hi clear
if exists('syntax_on')
  syntax reset
endif

set background=light
let g:colors_name = 'warm-burnout-light'

" -- Terminal ANSI colors (16) --
let g:terminal_ansi_colors = [
      \ '#3a3630', '#b82820', '#2d6a14', '#8a6000',
      \ '#2060a0', '#8a3090', '#146858', '#c0b8aa',
      \ '#686868', '#c83028', '#3a7a20', '#9a7008',
      \ '#2870b0', '#9a38a0', '#208870', '#faf6f0',
      \ ]

" -- Editor UI --
hi Normal           guifg=#3a3630 guibg=#f5ede0 ctermfg=236 ctermbg=230
hi NormalNC         guifg=#3a3630 guibg=#f5ede0 ctermfg=236 ctermbg=230
hi NormalFloat      guifg=#3a3630 guibg=#f0e8dc ctermfg=236 ctermbg=255
hi FloatBorder      guifg=#ddd6ca guibg=#f0e8dc ctermfg=252 ctermbg=255
hi FloatTitle       guifg=#3a3630 guibg=#f0e8dc gui=bold    ctermfg=236 ctermbg=255 cterm=bold
hi EndOfBuffer      guifg=#f5ede0 guibg=#f5ede0 ctermfg=230 ctermbg=230

hi Cursor           guifg=#f5ede0 guibg=#8a6600 ctermfg=230 ctermbg=94
hi lCursor          guifg=#f5ede0 guibg=#8a6600 ctermfg=230 ctermbg=94
hi CursorIM         guifg=#f5ede0 guibg=#8a6600 ctermfg=230 ctermbg=94
hi CursorLine                     guibg=#ece4d6             ctermbg=254 cterm=NONE
hi CursorColumn                   guibg=#ece4d6             ctermbg=254
hi CursorLineNr     guifg=#6a6258 guibg=NONE    ctermfg=241 ctermbg=NONE
hi LineNr           guifg=#a89f8c guibg=NONE    ctermfg=248 ctermbg=NONE
hi SignColumn       guifg=#a89f8c guibg=#f5ede0 ctermfg=248 ctermbg=230
hi ColorColumn                    guibg=#ece4d6             ctermbg=254
hi Folded           guifg=#544c40 guibg=#f0e8dc gui=italic  ctermfg=59 ctermbg=255 cterm=italic
hi FoldColumn       guifg=#a89f8c guibg=NONE    ctermfg=248 ctermbg=NONE

hi VertSplit        guifg=#ddd6ca guibg=NONE    ctermfg=252 ctermbg=NONE
hi WinSeparator     guifg=#ddd6ca guibg=NONE    ctermfg=252 ctermbg=NONE
hi StatusLine       guifg=#5c5750 guibg=#ede6da             ctermfg=59 ctermbg=255 cterm=NONE
hi StatusLineNC     guifg=#a89f8c guibg=#ede6da             ctermfg=248 ctermbg=255 cterm=NONE
hi TabLine          guifg=#5c5750 guibg=#ede6da             ctermfg=59 ctermbg=255 cterm=NONE
hi TabLineSel       guifg=#3a3630 guibg=#f5ede0 gui=bold    ctermfg=236 ctermbg=230 cterm=bold
hi TabLineFill      guifg=#5c5750 guibg=#ede6da             ctermfg=59 ctermbg=255

hi Visual                         guibg=#cfdae2             ctermbg=152
hi VisualNOS                      guibg=#cfdae2             ctermbg=152

hi Pmenu            guifg=#3a3630 guibg=#f0e8dc             ctermfg=236 ctermbg=255
hi PmenuSel         guifg=#3a3630 guibg=#cfdae2 gui=bold    ctermfg=236 ctermbg=152 cterm=bold
hi PmenuSbar                      guibg=#f0e8dc             ctermbg=255
hi PmenuThumb                     guibg=#a89f8c             ctermbg=248
hi WildMenu                       guibg=#cfdae2             ctermbg=152

hi Search           guifg=#3a3630 guibg=#e0c890             ctermfg=236 ctermbg=222
hi IncSearch        guifg=#f5ede0 guibg=#8a6600             ctermfg=230 ctermbg=94
hi CurSearch        guifg=#f5ede0 guibg=#8a6600             ctermfg=230 ctermbg=94
hi Substitute       guifg=#f5ede0 guibg=#b03434             ctermfg=230 ctermbg=124
hi MatchParen       guifg=#b8522e guibg=NONE    gui=bold    ctermfg=173 ctermbg=NONE cterm=bold
hi QuickFixLine                   guibg=#cfdae2             ctermbg=152

hi SpecialKey       guifg=#a89f8c             ctermfg=248
hi NonText          guifg=#a89f8c             ctermfg=248
hi Whitespace       guifg=#a89f8c             ctermfg=248
hi Conceal          guifg=#5c5750             ctermfg=59

hi Directory        guifg=#855700             ctermfg=94
hi Title            guifg=#855700 gui=bold    ctermfg=94 cterm=bold
hi ErrorMsg         guifg=#b03434             ctermfg=124
hi WarningMsg       guifg=#b8522e             ctermfg=173
hi MoreMsg          guifg=#285464             ctermfg=24
hi Question         guifg=#285464             ctermfg=24
hi ModeMsg          guifg=#3a3630 gui=bold    ctermfg=236 cterm=bold

" -- Spell --
hi SpellBad   guisp=#b03434 gui=undercurl cterm=undercurl
hi SpellCap   guisp=#b8522e gui=undercurl cterm=undercurl
hi SpellLocal guisp=#285464 gui=undercurl cterm=undercurl
hi SpellRare  guisp=#544c40 gui=undercurl cterm=undercurl

" -- Diff --
hi DiffAdd                   guibg=#e2ecd8 ctermbg=194
hi DiffChange                guibg=#dce4ec ctermbg=189
hi DiffDelete                guibg=#f0d4d4 ctermbg=224
hi DiffText                  guibg=#c4d4e4 ctermbg=152

" -- Syntax groups --
hi Comment        guifg=#544c40 gui=italic              ctermfg=59 cterm=italic
hi Constant       guifg=#7e4060                         ctermfg=89
hi String         guifg=#4d5c1a                         ctermfg=58
hi Character      guifg=#4d5c1a                         ctermfg=58
hi Number         guifg=#7e4060                         ctermfg=89
hi Boolean        guifg=#7e4060                         ctermfg=89
hi Float          guifg=#7e4060                         ctermfg=89
hi Identifier     guifg=#3a3630                         ctermfg=236 cterm=NONE
hi Function       guifg=#855700                         ctermfg=94
hi Statement      guifg=#924800 gui=bold                ctermfg=94 cterm=bold
hi Conditional    guifg=#924800 gui=bold                ctermfg=94 cterm=bold
hi Repeat         guifg=#924800 gui=bold                ctermfg=94 cterm=bold
hi Label          guifg=#924800 gui=bold                ctermfg=94 cterm=bold
hi Operator       guifg=#8f4418                         ctermfg=94
hi Keyword        guifg=#924800 gui=bold                ctermfg=94 cterm=bold
hi Exception      guifg=#924800 gui=bold                ctermfg=94 cterm=bold
hi PreProc        guifg=#924800                         ctermfg=94
hi Include        guifg=#924800 gui=bold                ctermfg=94 cterm=bold
hi Define         guifg=#924800                         ctermfg=94
hi Macro          guifg=#7a5a1c                         ctermfg=58
hi PreCondit      guifg=#924800                         ctermfg=94
hi Type           guifg=#285464 gui=italic              ctermfg=24 cterm=italic
hi StorageClass   guifg=#924800 gui=bold                ctermfg=94 cterm=bold
hi Structure      guifg=#285464 gui=italic              ctermfg=24 cterm=italic
hi Typedef        guifg=#285464 gui=italic              ctermfg=24 cterm=italic
hi Special        guifg=#8f4418                         ctermfg=94
hi SpecialChar    guifg=#286a48                         ctermfg=23
hi Tag            guifg=#8e4632 gui=bold                ctermfg=95 cterm=bold
hi Delimiter      guifg=#3a3630                         ctermfg=236
hi SpecialComment guifg=#544c40 gui=italic              ctermfg=59 cterm=italic
hi Debug          guifg=#b8522e                         ctermfg=173
hi Underlined                   gui=underline          cterm=underline
hi Ignore         guifg=#5c5750                         ctermfg=59
hi Error          guifg=#b03434                         ctermfg=124
hi Todo           guifg=#8a6600 gui=bold                ctermfg=94 cterm=bold
hi Added          guifg=#226414                         ctermfg=22
hi Changed        guifg=#2868a0                         ctermfg=25
hi Removed        guifg=#c43040                         ctermfg=160

" -- LSP / Diagnostics --
hi DiagnosticError      guifg=#b03434 ctermfg=124
hi DiagnosticWarn       guifg=#b8522e ctermfg=173
hi DiagnosticInfo       guifg=#285464 ctermfg=24
hi DiagnosticHint       guifg=#544c40 ctermfg=59
hi DiagnosticOk         guifg=#226414 ctermfg=22
hi DiagnosticSignError  guifg=#b03434 ctermfg=124
hi DiagnosticSignWarn   guifg=#b8522e ctermfg=173
hi DiagnosticSignInfo   guifg=#285464 ctermfg=24
hi DiagnosticSignHint   guifg=#544c40 ctermfg=59
hi DiagnosticUnderlineError guisp=#b03434 gui=undercurl cterm=undercurl
hi DiagnosticUnderlineWarn  guisp=#b8522e gui=undercurl cterm=undercurl
hi DiagnosticUnderlineInfo  guisp=#285464 gui=undercurl cterm=undercurl
hi DiagnosticUnderlineHint  guisp=#544c40 gui=undercurl cterm=undercurl
hi DiagnosticVirtualTextError guifg=#b03434 ctermfg=124
hi DiagnosticVirtualTextWarn  guifg=#b8522e ctermfg=173
hi DiagnosticVirtualTextInfo  guifg=#285464 ctermfg=24
hi DiagnosticVirtualTextHint  guifg=#544c40 ctermfg=59

hi ALEErrorSign        guifg=#b03434 ctermfg=124
hi ALEWarningSign      guifg=#b8522e ctermfg=173
hi ALEInfoSign         guifg=#285464 ctermfg=24
hi ALEError            guisp=#b03434 gui=undercurl cterm=undercurl
hi ALEWarning          guisp=#b8522e gui=undercurl cterm=undercurl

hi CocErrorSign   guifg=#b03434 ctermfg=124
hi CocWarningSign guifg=#b8522e ctermfg=173
hi CocInfoSign    guifg=#285464 ctermfg=24
hi CocHintSign    guifg=#544c40 ctermfg=59
hi CocErrorFloat  guifg=#b03434 ctermfg=124
hi CocErrorHighlight   guisp=#b03434 gui=undercurl cterm=undercurl
hi CocWarningHighlight guisp=#b8522e gui=undercurl cterm=undercurl

" -- Git --
hi GitGutterAdd    guifg=#226414 ctermfg=22
hi GitGutterChange guifg=#2868a0 ctermfg=25
hi GitGutterDelete guifg=#c43040 ctermfg=160
hi SignifySignAdd    guifg=#226414 ctermfg=22
hi SignifySignChange guifg=#2868a0 ctermfg=25
hi SignifySignDelete guifg=#c43040 ctermfg=160
hi diffAdded   guifg=#226414 ctermfg=22
hi diffRemoved guifg=#c43040 ctermfg=160
hi diffChanged guifg=#2868a0 ctermfg=25

" -- HTML / XML --
hi htmlTag        guifg=#8e4632 gui=bold ctermfg=95 cterm=bold
hi htmlEndTag     guifg=#8e4632 gui=bold ctermfg=95 cterm=bold
hi htmlTagName    guifg=#8e4632 gui=bold ctermfg=95 cterm=bold
hi htmlArg        guifg=#855700          ctermfg=94
hi htmlTitle      guifg=#3a3630 gui=bold ctermfg=236 cterm=bold
hi htmlH1         guifg=#3a3630 gui=bold ctermfg=236 cterm=bold
hi xmlTag         guifg=#8e4632 gui=bold ctermfg=95 cterm=bold
hi xmlTagName     guifg=#8e4632 gui=bold ctermfg=95 cterm=bold
hi xmlEndTag      guifg=#8e4632 gui=bold ctermfg=95 cterm=bold

" -- CSS --
hi cssClassName    guifg=#7a5a1c gui=italic ctermfg=58 cterm=italic
hi cssIdentifier   guifg=#7a5a1c gui=italic ctermfg=58 cterm=italic
hi cssProp         guifg=#74501c gui=italic ctermfg=94 cterm=italic
hi cssAttr         guifg=#4d5c1a            ctermfg=58
hi cssAttrRegion   guifg=#4d5c1a            ctermfg=58
hi cssBraces       guifg=#3a3630            ctermfg=236
hi cssPseudoClass  guifg=#883850            ctermfg=89
hi cssPseudoClassId guifg=#883850           ctermfg=89
hi cssValueLength  guifg=#7e4060            ctermfg=89
hi cssValueNumber  guifg=#7e4060            ctermfg=89
hi cssFunctionName guifg=#855700            ctermfg=94
hi cssColor        guifg=#7e4060            ctermfg=89

" -- JavaScript / TypeScript --
hi jsFunction         guifg=#924800 gui=bold  ctermfg=94 cterm=bold
hi jsFuncCall         guifg=#855700           ctermfg=94
hi jsClassDefinition  guifg=#285464 gui=italic ctermfg=24 cterm=italic
hi jsGlobalObjects    guifg=#285464 gui=italic ctermfg=24 cterm=italic
hi jsThis             guifg=#8e4632 gui=italic ctermfg=95 cterm=italic
hi jsObjectKey        guifg=#74501c gui=italic ctermfg=94 cterm=italic
hi typescriptKeyword     guifg=#924800 gui=bold ctermfg=94 cterm=bold
hi typescriptClassName   guifg=#285464 gui=italic ctermfg=24 cterm=italic
hi typescriptFuncKeyword guifg=#924800 gui=bold ctermfg=94 cterm=bold
hi typescriptMember      guifg=#883850          ctermfg=89

" -- Python --
hi pythonBuiltin     guifg=#883850           ctermfg=89
hi pythonFunction    guifg=#855700           ctermfg=94
hi pythonDecorator   guifg=#7a5a1c gui=italic ctermfg=58 cterm=italic
hi pythonDecoratorName guifg=#7a5a1c gui=italic ctermfg=58 cterm=italic
hi pythonStatement   guifg=#924800 gui=bold  ctermfg=94 cterm=bold
hi pythonClass       guifg=#285464 gui=italic ctermfg=24 cterm=italic
hi pythonSelf        guifg=#8e4632 gui=italic ctermfg=95 cterm=italic

" -- Rust --
hi rustKeyword         guifg=#924800 gui=bold ctermfg=94 cterm=bold
hi rustType            guifg=#285464 gui=italic ctermfg=24 cterm=italic
hi rustTrait           guifg=#285464 gui=italic ctermfg=24 cterm=italic
hi rustMacro           guifg=#7a5a1c          ctermfg=58
hi rustAttribute       guifg=#7a5a1c gui=italic ctermfg=58 cterm=italic
hi rustLifetime        guifg=#8e4632 gui=italic ctermfg=95 cterm=italic
hi rustFuncCall        guifg=#855700          ctermfg=94
hi rustFuncName        guifg=#855700          ctermfg=94
hi rustEnumVariant     guifg=#7e4060          ctermfg=89

" -- Markdown --
hi markdownH1        guifg=#924800 gui=bold ctermfg=94 cterm=bold
hi markdownH2        guifg=#855700 gui=bold ctermfg=94 cterm=bold
hi markdownH3        guifg=#7a5a1c gui=bold ctermfg=58 cterm=bold
hi markdownH4        guifg=#4d5c1a gui=bold ctermfg=58 cterm=bold
hi markdownH5        guifg=#285464 gui=bold ctermfg=24 cterm=bold
hi markdownH6        guifg=#8e4632 gui=bold ctermfg=95 cterm=bold
hi markdownCode      guifg=#4d5c1a          ctermfg=58
hi markdownCodeBlock guifg=#4d5c1a          ctermfg=58
hi markdownLinkText  guifg=#8e4632 gui=underline ctermfg=95 cterm=underline
hi markdownUrl       guifg=#285464 gui=underline ctermfg=24 cterm=underline
hi markdownBold      guifg=#3a3630 gui=bold ctermfg=236 cterm=bold
hi markdownItalic    guifg=#883850 gui=italic ctermfg=89 cterm=italic
hi markdownListMarker guifg=#855700         ctermfg=94

" -- Vim --
hi vimCommand    guifg=#924800 gui=bold ctermfg=94 cterm=bold
hi vimFunction   guifg=#855700          ctermfg=94
hi vimFuncName   guifg=#855700          ctermfg=94
hi vimGroup      guifg=#285464 gui=italic ctermfg=24 cterm=italic
hi vimHiGroup    guifg=#285464 gui=italic ctermfg=24 cterm=italic
hi vimOption     guifg=#74501c gui=italic ctermfg=94 cterm=italic
hi vimVar        guifg=#7e4060          ctermfg=89
hi vimNotation   guifg=#286a48          ctermfg=23

" -- NERDTree / netrw --
hi NERDTreeDir       guifg=#3a3630 gui=bold ctermfg=236 cterm=bold
hi NERDTreeDirSlash  guifg=#855700          ctermfg=94
hi NERDTreeFile      guifg=#5c5750          ctermfg=59
hi NERDTreeCWD       guifg=#b8522e gui=bold ctermfg=173 cterm=bold
hi NERDTreeOpenable  guifg=#855700          ctermfg=94
hi NERDTreeClosable  guifg=#855700          ctermfg=94
hi netrwDir          guifg=#855700 gui=bold ctermfg=94 cterm=bold
hi netrwExe          guifg=#4d5c1a          ctermfg=58
hi netrwClassify     guifg=#855700          ctermfg=94
