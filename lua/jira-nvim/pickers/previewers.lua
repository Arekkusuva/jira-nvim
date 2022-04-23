local M = {}

local previewers = require("telescope.previewers")
local PreviewBuffer = require("jira-nvim.buffers.preview_buffer").PreviewBuffer

function M.issue_previewer(_)
	return previewers.new_buffer_previewer({
		get_buffer_by_name = function(_, entry)
			return entry.value.issue_id
		end,
		keep_last_buf = true,
		define_preview = function(self, entry)
			-- Check if data has already been loaded
			if self.state.bufname == entry.value.issue_id then return end

			local bufnr = self.state.bufnr
			if vim.api.nvim_buf_is_valid(bufnr) then
				local buf = PreviewBuffer:new({
					bufnr = bufnr,
					issue = entry.value,
				})
				buf:render()
			end
		end,
	})
end

function M.issue_transition_previewer(issue, _)
	return previewers.new_buffer_previewer({
		get_buffer_by_name = function(_, entry)
			return entry.value.issue_key
		end,
		keep_last_buf = true,
		define_preview = function(self, entry)
			-- Check if data has already been loaded
			if self.state.bufname == entry.value.issue_key then return end

			local bufnr = self.state.bufnr
			if vim.api.nvim_buf_is_valid(bufnr) then
				local buf = PreviewBuffer:new({
					bufnr = bufnr,
					issue = issue,
				})
				buf:render()
			end
		end,
	})
end

return M

