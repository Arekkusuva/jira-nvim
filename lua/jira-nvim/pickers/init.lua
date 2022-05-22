local M = {}

local pickers = require("telescope.pickers")
local conf = require("telescope.config").values

local finders = require("jira-nvim.pickers.finders")
local mappings = require("jira-nvim.pickers.mappings")
local previewers = require("jira-nvim.pickers.previewers")

function M.query(jql, opts)
	opts = opts or {}

	print("Fetching issues ...")
	vim.schedule(function()
		local issues = require("libjira_nvim").query(jql or "")
		print(" ")

		pickers.new(opts, {
			prompt_title = "Issues",
			finder = finders.query_finder(issues, opts.finder),
			sorter = conf.generic_sorter(opts.sorter),
			attach_mappings = mappings.issues_mappings({
				issue_transitions = M.issue_transitions,
			}),
			previewer = previewers.issue_previewer(opts.previewer),
		}):find()
	end)
end

function M.issue_transitions(issue, opts)
	opts = opts or {}
	-- TODO: Request issue by key

	print("Fetching transitions ...")
	vim.schedule(function()
		print(issue.issue_key)
		local transitions = require("libjira_nvim").issue_transitions(issue.issue_key)
		print(" ")

		pickers.new(opts, {
			prompt_title = issue.issue_key .. " transitions",
			finder = finders.issue_transitions_finder(issue.issue_key, transitions, opts.finder),
			sorter = conf.generic_sorter(opts.sorter),
			attach_mappings = mappings.issue_transitions_mappings(opts.mappings),
			previewer = previewers.issue_transition_previewer(issue, opts.previewer),
		}):find()
	end)
end

return M
