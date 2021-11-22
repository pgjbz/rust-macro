" Specify a directory for plugins
call plug#begin('~/.local/share/nvim/site/plugged')

Plug 'scrooloose/nerdtree'
"Plug 'tsony-tsonev/nerdtree-git-plugin'
Plug 'Xuyuanp/nerdtree-git-plugin'
Plug 'tiagofumo/vim-nerdtree-syntax-highlight'
Plug 'ryanoasis/vim-devicons'
Plug 'airblade/vim-gitgutter'
Plug 'ctrlpvim/ctrlp.vim' " fuzzy find files
Plug 'scrooloose/nerdcommenter'
Plug 'rust-lang/rust.vim'
"Plug 'prettier/vim-prettier', { 'do': 'yarn install' }

Plug 'christoomey/vim-tmux-navigator'

Plug 'vim-airline/vim-airline'
"Plug 'morhetz/gruvbox'

Plug 'HerringtonDarkholme/yats.vim' " TS Syntax

" Initialize plugin system
call plug#end()

inoremap jk <ESC>
nmap <C-n> :NERDTreeToggle<CR>
vmap ++ <plug>NERDCommenterToggle
nmap ++ <plug>NERDCommenterToggle

" open NERDTree automatically
"autocmd StdinReadPre _ let s:std_in=1
"autocmd VimEnter _ NERDTree

let g:NERDTreeGitStatusWithFlags = 1
"let g:WebDevIconsUnicodeDecorateFolderNodes = 1
"let g:NERDTreeGitStatusNodeColorization = 1
"let g:NERDTreeColorMapCustom = {
"\ "Staged" : "#0ee375",  
 "\ "Modified" : "#d9bf91",  
 "\ "Renamed" : "#51C9FC",  
 "\ "Untracked" : "#FCE77C",  
 "\ "Unmerged" : "#FC51E6",  
 "\ "Dirty" : "#FFBD61",  
 "\ "Clean" : "#87939A",  
 "\ "Ignored" : "#808080"  
 "\ }

let g:NERDTreeIgnore = ['^node_modules$']

" vim-prettier
"let g:prettier#quickfix*enabled = 0
"let g:prettier#quickfix_auto_focus = 0
" run prettier on save
"let g:prettier#autoformat = 0
"autocmd BufWritePre *.js,_.jsx,_.mjs,_.ts,_.tsx,_.css,_.less,_.scss,_.json,_.graphql,_.md,_.vue,_.yaml,\_.html PrettierAsync

" ctrlp
let g:ctrlp_user_command = ['.git/', 'git --git-dir=%s/.git ls-files -oc --exclude-standard']

" j/k will move virtual lines (lines that wrap)
noremap <silent> <expr> j (v:count == 0 ? 'gj' : 'j')
noremap <silent> <expr> k (v:count == 0 ? 'gk' : 'k')

set number
set smarttab
set cindent
set tabstop=2
set shiftwidth=2
" always uses spaces instead of tab characters
set expandtab

" \***\* for nord time something **\*\*\*\*\*
" sync open file with NERDTree
" " Check if NERDTree is open or active
"function! IsNERDTreeOpen()  
" return exists("t:NERDTreeBufName") && (bufwinnr(t:NERDTreeBufName) != -1)
"endfunction

" Call NERDTreeFind iff NERDTree is active, current window contains a modifiable
" file, and we're not in vimdiff
"function! SyncTree()
" if &modifiable && IsNERDTreeOpen() && strlen(expand('%')) > 0 && !&diff
" NERDTreeFind
" wincmd p
" endif
"endfunction

" Highlight currently open buffer in NERDTree
"autocmd BufEnter \* call SyncTree()
"\***\* end here **\*\*\*\*\*\*\*\*


function! s:check_back_space() abort
let col = col('.') - 1
return !col || getline('.')[col - 1] =~# '\s'
endfunction
