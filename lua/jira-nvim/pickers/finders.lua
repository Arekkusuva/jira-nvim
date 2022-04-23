local M = {}

local finders = require("telescope.finders")

function M.query_finder(issues, _)
	return finders.new_table({
		results = issues,
		entry_maker = function(entry)
			return {
				value = entry,
				ordinal = string.format("[%s] %s", entry.issue_key, entry.summary),
				display = function(opts)
					return opts.ordinal
				end,
			}
		end,
	})
end

function M.issue_transitions_finder(issue_key, transitions, _)
	return finders.new_table({
		results = transitions,
		entry_maker = function(entry)
			return {
				value = {
					issue_key = issue_key,
					transition = entry,
				},
				ordinal = entry.name,
				display = function(opts)
					return opts.ordinal
				end,
			}
		end,
	})
end

return M
