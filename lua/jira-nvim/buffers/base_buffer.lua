local M = {}

local writer = require("jira-nvim.buffers.writer")

local BaseBuffer = {}
BaseBuffer.__index = BaseBuffer

function BaseBuffer:new(opts)
	local this = {
		bufnr = opts.bufnr,
	}
	setmetatable(this, self)
	return this
end

function BaseBuffer:clear()
	writer.clear_buffer(self.bufnr)
end

M.BaseBuffer = BaseBuffer
return M


