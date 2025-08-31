### A Pluto.jl notebook ###
# v0.20.13

using Markdown
using InteractiveUtils

# ╔═╡ 4e8bdcb8-7fec-11f0-33ec-d71786297a1d
# traverse with one loop and keep track of current smallest, current largest.
# once sorted, traverse and get abs difference and sum.
inp_lines = readlines("1sample.txt")

# ╔═╡ 69ab5c43-7bf8-40a6-8b30-fb29a0ae535a
function part1!(file_path)
	fsts = Int64[]
	snds = Int64[]
	for line in eachline(file_path)
		tokens = split(line)
		push!(fsts, parse(Int64, tokens[1]))
		push!(snds, parse(Int64, tokens[2]))
	end
	sorted_fst = sort(fsts)
	sorted_snds = sort(snds)
	ret = 0
	for i in 1:length(sorted_fst)
		ret += abs(sorted_fst[i] - sorted_snds[i])
	end
	ret
end

# ╔═╡ 089ab7a6-a755-45a5-bfa2-374d1bd5abd4
@time part1!("1input.txt")

# ╔═╡ 8bc0ef8e-c4ed-4f5e-9718-eefd0a9bccb9


# ╔═╡ 00000000-0000-0000-0000-000000000001
PLUTO_PROJECT_TOML_CONTENTS = """
[deps]
"""

# ╔═╡ 00000000-0000-0000-0000-000000000002
PLUTO_MANIFEST_TOML_CONTENTS = """
# This file is machine-generated - editing it directly is not advised

julia_version = "1.11.6"
manifest_format = "2.0"
project_hash = "da39a3ee5e6b4b0d3255bfef95601890afd80709"

[deps]
"""

# ╔═╡ Cell order:
# ╠═4e8bdcb8-7fec-11f0-33ec-d71786297a1d
# ╠═69ab5c43-7bf8-40a6-8b30-fb29a0ae535a
# ╠═089ab7a6-a755-45a5-bfa2-374d1bd5abd4
# ╠═8bc0ef8e-c4ed-4f5e-9718-eefd0a9bccb9
# ╟─00000000-0000-0000-0000-000000000001
# ╟─00000000-0000-0000-0000-000000000002
