local M = {}

local writer = require("jira-nvim.buffers.writer")
local BaseBuffer = require("jira-nvim.buffers.base_buffer").BaseBuffer

-- Issue page
local IssueBuffer = {}
setmetatable(IssueBuffer, { __index = BaseBuffer })

function IssueBuffer:new(opts)
	local this = BaseBuffer:new(opts)
	this.issue = opts.issue

	setmetatable(this, { __index = self })
	return this
end

function IssueBuffer:render()
	self:begin_frame()

	local max_width = math.floor(vim.fn.winwidth(0) * 0.8)
	writer.write_block(self.bufnr, {
		string.format(" [%s] %s", self.issue.issue_key, self.issue.summary),
		string.rep("-", max_width),
	})
	self:render_details()

	self:end_frame()
end

function IssueBuffer:update(issue)
	self.issue = issue
	self:render()
end

function IssueBuffer:render_details()
	writer.write_block(self.bufnr, {
		string.format("  Type:     %s", self.issue.issue_type),
		string.format("  Status:   %s", self.issue.status),
		string.format("  Labels:   %s", self.issue.labels),
		string.format("  Created:  %s", self.issue.created_ago),
		string.format("  Assignee: %s", self.issue.assignee_name),
		string.format("  Priority: %s", self.issue.priority),
	})
end

M.IssueBuffer = IssueBuffer
return M
