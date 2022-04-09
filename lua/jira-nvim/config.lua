local M = {}

local config = {}

function M.setup(cfg)
	config = cfg
end

function M.host()
	return config.host
end

return M
