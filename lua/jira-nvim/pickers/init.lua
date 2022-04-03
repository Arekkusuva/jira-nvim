local M = {}

local pickers = require("telescope.pickers")
local conf = require("telescope.config").values

local finders = require("jira-nvim.pickers.finders")
local mappings = require("jira-nvim.pickers.mappings")
local previewers = require("jira-nvim.pickers.previewers")

local function query(jql, opts)
	opts = opts or {}

	print("Fetching issues ...")
	vim.schedule(function()
		local issues = require("libjira_nvim").query(jql or "")
		print(" ")

		pickers.new(opts, {
			prompt_title = "Issues",
			finder = finders.query_finder(issues, opts.finder),
			sorter = conf.generic_sorter(opts.sorter),
			attach_mappings = mappings.attach_mappings(opts.mappings),
			previewer = previewers.ticket_previewer(opts.previewer),
		}):find()
	end)
end

M.pickers = {
    query = query,
}

return M

