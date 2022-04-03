local M = {}

function M.clear_buffer(bufnr)
    vim.api.nvim_buf_set_lines(bufnr, 0, -1, false, {})
end

function M.write_lines(bufnr, lines)
    vim.api.nvim_buf_set_lines(bufnr, -1, -1, false, lines)
end

--- @param line integer #First line index (inclusive)
function M.write_lines_at(bufnr, line, lines)
    if line < 1 then
        line = 0
    else
        line = line - 1
    end
    vim.api.nvim_buf_set_lines(bufnr, line, line + #lines, false, lines)
end

function M.write_block(bufnr, lines)
    table.insert(lines, "")
    M.write_lines(bufnr, lines)
end

return M
