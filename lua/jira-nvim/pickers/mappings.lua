local M = {}

local actions = require("telescope.actions")

local picker_actions = require("jira-nvim.pickers.actions")

function M.issues_mappings(opts)
	opts = opts or {}
	return function(prompt_bufnr, map)
		actions.select_default:replace(function()
			picker_actions.open_issue(prompt_bufnr)
		end)
		map("n", "<C-y>", picker_actions.copy_issue_url)
		map("n", "<C-b>", picker_actions.open_in_browser)
		map("n", "<C-t>", picker_actions.issue_transitions(opts.issue_transitions))
		return true
	end
end

function M.issue_transitions_mappings(_)
	return function(prompt_bufnr, map)
		actions.select_default:replace(function()
			-- TODO: Do transition
			picker_actions.do_transition(prompt_bufnr, false)
			actions.close(prompt_bufnr)
		end)
		map("n", "<C-t>", function(bufnr) picker_actions.do_transition(bufnr, true) end)
		return true
	end
end

return M
