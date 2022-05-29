local M = {}

local writer = require("jira-nvim.buffers.writer")

local BaseBuffer = {}
BaseBuffer.__index = BaseBuffer

local function setup_buffer(bufnr)
	vim.api.nvim_buf_call(bufnr, function()
	vim.cmd [[setlocal filetype=jiranvim]]
	vim.cmd [[setlocal buftype=acwrite]]
	vim.cmd [[setlocal wrap]]
  end)
end


function BaseBuffer:new(opts)
	local this = {
		bufnr = opts.bufnr,
	}
	setup_buffer(opts.bufnr)
	setmetatable(this, self)
	return this
end

function BaseBuffer:clear()
	writer.clear_buffer(self.bufnr)
end

function BaseBuffer:begin_frame()
	self:clear()
end

function BaseBuffer:end_frame()
	vim.api.nvim_buf_set_option(self.bufnr, "modified", false)
end

M.BaseBuffer = BaseBuffer
return M


