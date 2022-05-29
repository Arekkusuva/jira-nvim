local M = {}

local IssueBuffer = require("jira-nvim.buffers.issue_buffer").IssueBuffer

local function issue_file(issue_key)
	return string.format("jiranvim://issue/%s", issue_key)
end

function M.create_issue_buffer(issue)
	local bufnr = vim.api.nvim_create_buf(true, false)
	vim.api.nvim_set_current_buf(bufnr)
	vim.cmd("file " .. issue_file(issue.issue_key))

	local buf = IssueBuffer:new({
		bufnr = bufnr,
		issue = issue,
	})
	buf:render()

	print("Fetching updates...")
	vim.schedule(function()
		issue = require("libjira_nvim").issue_by_key(issue.issue_key)
		buf:update(issue)
		print(" ")
	end)
end

return M
