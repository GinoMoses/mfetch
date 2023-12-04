return {
  'nvimdev/dashboard-nvim',
  event = 'VimEnter',
  config = function()
    require('dashboard').setup {
      config = {
        header = require('custom.plugins.assets.dashboard-art')
      }
    }
  end,
  dependencies = { {'nvim-tree/nvim-web-devicons'}}
}
