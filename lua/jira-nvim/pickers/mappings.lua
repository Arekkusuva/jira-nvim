local M = {}

local actions = require("telescope.actions")

local picker_actions = require("jira-nvim.pickers.actions")

function M.attach_mappings(_)
    return function(prompt_bufnr, map)
        actions.select_default:replace(function()
            actions.close(prompt_bufnr)
        end)
		map("n", "<C-b>", picker_actions.open_in_browser)
        return true
    end
end

return M

