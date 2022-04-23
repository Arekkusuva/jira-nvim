local M = {}

local config = require("jira-nvim.config")

function M.init()
	if vim.g.jira_nvim_initialized then return end

	-- Check if libdash_nvim has been built
	local ok, lib = pcall(require, "libjira_nvim")
	if not ok or lib == nil then
		print("Failed to load `libjira_nvim` module, make sure you have set up jira-nvim with `make build` as a post-update/install hook")
		return
	end
	vim.g.jira_nvim_initialized = true
end

M._setup_once = false
function M.setup(cfg)
	if M._setup_once then return end
	M._setup_once = true

	local with_defalts = require("libjira_nvim").setup(cfg or {})
	config.setup(with_defalts)
end

return M
