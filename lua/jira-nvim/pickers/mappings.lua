local M = {}

local actions = require("telescope.actions")
local action_state = require("telescope.actions.state")

function M.attach_mappings(_)
    return function(prompt_bufnr, _)
        actions.select_default:replace(function()
            actions.close(prompt_bufnr)
            local selection = action_state.get_selected_entry()
            print(vim.inspect(selection))
        end)
        return true
    end
end

return M

