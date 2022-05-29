local M = {}

local action_state = require "telescope.actions.state"
local actions = require "telescope.actions"

local config = require("jira-nvim.config")
local buffers = require("jira-nvim.buffers")

function M.issue_url(_)
	local issue = action_state.get_selected_entry().value;
	local url = string.format("%s/browse/%s", config.host(), issue.issue_key);
	return url
end

function M.copy_issue_url(prompt_bufnr)
	local url = M.issue_url(prompt_bufnr)
	vim.notify("URL copied to clipboard", 1, { title = "Jira.nvim" })
	vim.fn.setreg("+", url)
end

function M.open_in_browser(prompt_bufnr)
	local url = M.issue_url(prompt_bufnr);

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

function M.issue_transitions(picker)
	return function(prompt_bufnr)
		local issue = action_state.get_selected_entry().value;

		-- TODO: Should we hide it and then show again, to avoid redundant requests?
		actions.close(prompt_bufnr)
		picker(issue)
	end
end

function M.do_transition(_, _)
	local selected = action_state.get_selected_entry().value;
	local issue_key = selected.issue_key
	local transition_id = selected.transition.transition_id

	-- TODO: Reload issues
	print("Performing issue transition ...")
	vim.schedule(function()
		require("libjira_nvim").perform_issue_transition(issue_key, transition_id)
		print(" ")
	end)
end

function M.open_issue(prompt_bufnr)
	actions.close(prompt_bufnr)

	local selected = action_state.get_selected_entry(prompt_bufnr).value;
	buffers.create_issue_buffer(selected)
end

return M
