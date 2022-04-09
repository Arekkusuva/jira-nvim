local M = {}


local writer = require("jira-nvim.buffers.writer")
local BaseBuffer = require("jira-nvim.buffers.base_buffer").BaseBuffer

-- Issue preview
local PreviewBuffer = {}
setmetatable(PreviewBuffer, { __index = BaseBuffer })

function PreviewBuffer:new(opts)
    local this = BaseBuffer:new(opts)
    this.issue = opts.issue

    setmetatable(this, { __index = self })
    return this
end

function PreviewBuffer:render()
    self:clear()

	local max_width = vim.fn.winwidth(0)
	local wrapped = require("libjira_nvim").wrap_text(string.format(" %s", self.issue.summary), max_width)
	table.insert(wrapped, string.rep("-", max_width))

	writer.write_block_at(self.bufnr, 0, wrapped)

	self:render_details()
end

function PreviewBuffer:render_details()
	writer.write_block(self.bufnr, {
		string.format("  Key:      %s", self.issue.issue_key),
		string.format("  Type:     %s", self.issue.issue_type),
		string.format("  Status:   %s", self.issue.status),
		string.format("  Labels:   %s", self.issue.labels),
		string.format("  Created:  %s", self.issue.created_ago),
		string.format("  Assignee: %s", self.issue.assignee_name),
		string.format("  Priority: %s", self.issue.priority),
	})
end

M.PreviewBuffer = PreviewBuffer
return M
