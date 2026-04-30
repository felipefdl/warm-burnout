" Headless Vim colorscheme load test.
" Usage: vim -es -u NONE -i NONE --not-a-term \
"   --cmd "set rtp+=vim" \
"   --cmd "let g:variant='dark'" \
"   --cmd "let g:test_out='/tmp/wb.out'" \
"   -S tests/vim_load_test.vim
" Writes 'OK:warm-burnout-<variant>' on success or 'FAIL: ...' on failure
" to the file at g:test_out, then exits.

let s:variant = get(g:, 'variant', 'dark')
let s:colorscheme = 'warm-burnout-' . s:variant
let s:out = get(g:, 'test_out', '/tmp/warm-burnout-vim-test.out')

set t_Co=256

try
  execute 'colorscheme ' . s:colorscheme
catch
  call writefile(['FAIL: ' . s:colorscheme . ': ' . v:exception], s:out)
  cquit!
endtry

if g:colors_name !=# s:colorscheme
  call writefile(['FAIL: expected ' . s:colorscheme . ', got ' . string(g:colors_name)], s:out)
  cquit!
endif

" Spot-check that the colorscheme defined critical highlight groups across
" core syntax, diagnostics, and the plugins the platform claims to support.
let s:critical = [
      \ 'Normal',
      \ 'Comment',
      \ 'Keyword',
      \ 'Function',
      \ 'Type',
      \ 'String',
      \ 'DiagnosticError',
      \ 'GitGutterAdd',
      \ 'ALEErrorSign',
      \ 'CocErrorSign',
      \ 'SignifySignAdd',
      \ 'NERDTreeDir',
      \ 'netrwDir',
      \ ]

for s:g in s:critical
  if !hlexists(s:g)
    call writefile(['FAIL: highlight group ' . s:g . ' not defined'], s:out)
    cquit!
  endif
endfor

call writefile(['OK:' . s:colorscheme], s:out)
qall!
