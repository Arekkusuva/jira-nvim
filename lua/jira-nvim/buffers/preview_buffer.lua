local M = {}

local writer = require("jira-nvim.buffers.writer")
local BaseBuffer = require("jira-nvim.buffers.base_buffer").BaseBuffer

-- Issue preview
local PreviewBuffer = {}
setmetatable(PreviewBuffer, { __index = BaseBuffer })

function PreviewBuffer:new(opts)
    local this = BaseBuffer:new(opts)
    this.issue_id = opts.issue_id
    this.title = opts.title
    this.details = opts.details

    setmetatable(this, { __index = self })
    return this
end

function PreviewBuffer:render()
    self:clear()
    writer.write_block(self.bufnr, { self.title })
end

M.PreviewBuffer = PreviewBuffer
return M
