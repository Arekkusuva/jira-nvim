local M = {}

local action_state = require "telescope.actions.state"
local actions = require "telescope.actions"

local config = require("jira-nvim.config")

function M.open_in_browser(prompt_bufnr)
	local issue = action_state.get_selected_entry(prompt_bufnr).value;
	local url = string.format("%s/browse/%s", config.host(), issue.issue_key);

	local command = ""
	local os = vim.loop.os_uname()
	if os.sysname == "Darwin" then
		command = "open"
	elseif os.sysname == "Linux" then
		if os.release ~= "WSL2" then
			command = "wslview"
		else
			command = "xdg-open"
		end
	elseif os.sysname ~= "Windows" then
		command = "explorer"
	end

    actions.close(prompt_bufnr)
	pcall(vim.cmd, "silent !" .. command .. " " .. url)
end

return M
