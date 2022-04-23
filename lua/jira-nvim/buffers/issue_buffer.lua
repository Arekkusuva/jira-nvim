local M = {}

local writer = require("jira-nvim.buffers.writer")
local BaseBuffer = require("jira-nvim.buffers.base_buffer").BaseBuffer

-- Issue page
local IssueBuffer = {}
setmetatable(IssueBuffer, { __index = BaseBuffer })

function IssueBuffer:new(opts)
	local this = BaseBuffer:new(opts)
	this.issue_id = opts.issue_id
	this.title = opts.title
	this.details = opts.details

	setmetatable(this, { __index = self })
	return this
end

function IssueBuffer:render()
	self:clear()
	writer.write_block(self.bufnr, { self.title })
end

M.IssueBuffer = IssueBuffer
return M
