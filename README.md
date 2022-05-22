# Jira.nvim

Jira client as an extension for [telescope.nvim](https://github.com/nvim-telescope/telescope.nvim)

| ⚠️ | Work in progress |
| ----------- | ----------- |

* 🌠 [Features](#-features)
* ✔️ [Requirements](#-requirements)
* 📦 [Installation](#-installation)
* 🤖 [Commands](#-commands)
* 🚀 [Actions](#-actions)
* 📃 [Configuration sample](#-configuration-sample)
* 📋 [To-dos](#-to-dos)

## 🌠 Features

* Search issues using JQL
* Transition issues from one status to another

## ✔️ Requirements

* [neovim](https://neovim.io), tested for version >= 0.7.0
* [Rust](https://www.rust-lang.org/tools/install) to build dynamic lib
* [telescope.nvim](https://github.com/nvim-telescope/telescope.nvim) to show issues

## 📦 Installation

Using [packer.nvim](https://github.com/wbthomason/packer.nvim)

```lua
use {
  "Arekkusuva/jira-nvim",
  requires = {
    "nvim-telescope/telescope.nvim",
  },
  run = "make build",
  config = function ()
    require("jira-nvim").setup({
      host = "https://your-domain.atlassian.com",
      token_path = "~/.config/jira-nvim/token.txt",
    })
  end
}
```

## 🤖 Commands

There is only one command available `JiraQuery <your_jql>`, which executes query and shows found issues using telescope.

## 🚀 Actions

### Transition from one status to another

1. Select the issue to move and press `<C-t>`
2. Select the desired status and press `<CR>`

### Copy issue URL

1. Select the issue and press `<C-y>`

### Open issue in browser

1. Select the issue and press `<C-b>`

## 📃 Configuration sample

With [which-key.nvim](https://github.com/folke/which-key.nvim)

```lua
require("which-key").register({
  j = {
    name = "Jira",
    t = { "<cmd>JiraQuery project = <your_project> and status = 'To Do'<cr>", "To Do" },
    p = { "<cmd>JiraQuery project = <your_project> and status = 'In Progress'<cr>", "In Progress" },
    d = { "<cmd>JiraQuery project = <your_project> and status = 'Done'<cr>", "Done" },
  },
}, { prefix = "<leader>" })
```

## 📋 To-dos

- [x] Use specific version of Jira API
- [x] Fix error model in jira client
- [ ] Add issue detailed view with description and comments
- [ ] Ability to edit issue description
- [ ] Ability to add and edit comments

