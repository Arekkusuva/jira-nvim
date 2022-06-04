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
	writer.write_block(self.bufnr, {
		"",
		" Description",
		string.rep("-", max_width),
	})
	self:render_description()

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

--local token_doc_begin = 1
--local token_doc_end = 2
--local token_paragraph_begin = 3
local token_paragraph_end = 4
local token_text = 5

function IssueBuffer:render_description_token(token)
	local typ = token.type_id
	if (typ == token_paragraph_end) then
		writer.write_block(self.bufnr, {})
	elseif (typ == token_text) then
		writer.write_lines(self.bufnr, { token:text() })
	end
end

function IssueBuffer:render_description()
	local tokens = self.issue:take_description()
	if tokens then
		for i = 1, #tokens do
			print(i, ". ", self:render_description_token(tokens[i]))
		end
	end
end

M.IssueBuffer = IssueBuffer
return M
