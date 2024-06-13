# GDScript LSP

Hi! This is a work-in-progress LSP for GDScript.


Right now this project provides basic autocomplete for gdscript variables and functions, since the official LSP doesn't seem to work right for autocomplete in neovim.


## Installation

__Disclaimer__: I've only tried to set this up with the lazyvim package manager, but if you know what you're doing, you should be able to get this to work with other setups as well.

### Requirements

1. rustup v1.26.0 or higher

2. cargo v1.74.1 or higher

3. neovim v0.10.0 or higher

4. lazyvim package manager - maybe? read the disclaimer above

### Usage 

1. Clone the repo.

2. Compile the project with `cargo build`.

3. In your neovim config directory, add the following structure:

    - nvim/
        - lua/
            - custom/
                - plugins/
                    - godot_lsp.lua

4. In your godot_lsp.lua file add the following code:

```lua
local client = vim.lsp.start_client {
  name = 'godot-lsp-test',
  cmd = { '<path-to-cloned-project>/target/debug/godot-lsp' },
}

if not client then
  vim.notify 'the test client failed'
  return
end

vim.api.nvim_create_autocmd('FileType', {
  pattern = 'gdscript',
  callback = function()
    vim.lsp.buf_attach_client(0, client)
  end,
})

return {}
```

5. Replace `path-to-cloned-project` with the correct path.

6. Make sure the godot_lsp plugin is loaded. My lazyvim config is set up so everything in the plugins folder is automatically loaded. This will probably be different depending on your vim package manager, but in lazyvim this is what I did (in `init.lua`):

```lua
require('lazy').setup({
  { import = 'custom.plugins' },
})
```

