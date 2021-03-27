call plug#begin(stdpath('data') . '/plugged')
Plug 'ntk148v/vim-horizon'
Plug 'vim-airline/vim-airline'
Plug 'ryanoasis/vim-devicons'
Plug 'kien/rainbow_parentheses.vim'
Plug 'vim-airline/vim-airline-themes'
Plug 'scrooloose/nerdtree'
Plug 'Wafelack/Ark.vim'
Plug 'cespare/vim-toml'
Plug 'dense-analysis/ale'
call plug#end()
set termguicolors
set omnifunc=ale#completion#OmniFunc
let g:ale_completion_enabled = 1
let g:ale_completion_autoimport = 1
let g:ale_sign_column_always = 1
let g:ale_fix_on_save = 1
let g:ale_linters = { 
	\ 'rust': ['analyzer'], 
	\ 'C': ['clangd'],
	\ 'Bash': ['shellcheck'],
	\ 'Markdown': ['mdl']
\}
source ~/.config/nvim/ext/fileexp.vim
colorscheme horizon
let g:lightline = {'colorscheme' : 'horizon'}
set number
inoremap <expr> <C-j> pumvisible() ? "\<C-n>" : "\<C-j>"
inoremap <expr> <C-k> pumvisible() ? "\<C-p>" : "\<C-k>"
let g:airline_theme="onedark"
