local M = {}

local finders = require("telescope.finders")

function M.query_finder(issues, _)
	return finders.new_table({
		results = issues,
		entry_maker = function(entry)
			return {
				value = entry,
				ordinal = entry.summary,
				display = function(display_opts)
					return display_opts.ordinal
				end,
			}
		end,
	})
end

return M

