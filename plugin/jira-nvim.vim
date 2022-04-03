if exists('g:jira_nvim_initialized') | finish | endif

command! -nargs=1 JiraQuery lua require("jira-nvim.pickers").pickers.query(<f-args>)

lua require("jira-nvim").init()

