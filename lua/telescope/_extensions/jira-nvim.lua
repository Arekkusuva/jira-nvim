local pickers = require("jira-nvim.pickers")
return require("telescope").register_extension({
	exports = {
		query = pickers.query,
	},
})

