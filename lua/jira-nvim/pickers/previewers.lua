local M = {}

local previewers = require("telescope.previewers")
local PreviewBuffer = require("jira-nvim.buffers.preview_buffer").PreviewBuffer

function M.issue_previewer(_)
	return previewers.new_buffer_previewer({
		get_buffer_by_name = function(_, entry)
			return entry.value.issue_id
		end,
		define_preview = function(self, entry)
			-- Check if data has already been loaded
			if self.state.bufname == entry.value.issue_id then return end

			local bufnr = self.state.bufnr
			if vim.api.nvim_buf_is_valid(bufnr) then
				PreviewBuffer:new({
					bufnr = bufnr,
					issue = entry.value,
				}):render()
			end
		end,
	})
end

function M.issue_transition_previewer(issue, _)
	return previewers.new_buffer_previewer({
		get_buffer_by_name = function(_, entry)
			return entry.value.issue_key
		end,
		define_preview = function(self, entry)
			-- Check if data has already been loaded
			if self.state.bufname == entry.value.issue_key then return end

			local bufnr = self.state.bufnr
			if vim.api.nvim_buf_is_valid(bufnr) then
				PreviewBuffer:new({
					bufnr = bufnr,
					issue = issue,
				}):render()
			end
		end,
	})
end

return M

