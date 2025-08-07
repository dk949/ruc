-- When using nvim -l, print goes to stderr
io.write(vim.iter({"ruc", "test", "passed"}):join(' ') .. "\n")
