nums = %{
  "one" => "one1one",
  "two" => "two2two",
  "three" => "three3three",
  "four" => "four4four",
  "five" => "five5five",
  "six" => "six6six",
  "seven" => "seven7seven",
  "eight" => "eight8eight",
  "nine" => "nine9nine"
}

nums_list = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

repl = fn x ->
  Enum.map(nums_list, fn num ->
    if(String.match?(x, ~r/num/), do: String.replace(num, Map.get(nums, num)))
  end)
end

res =
  File.stream!("input.txt")
  |> Stream.map(&String.trim(&1))
  |> Enum.map(fn list ->
    repl.(list)
  end)

# |> Stream.map(&String.replace(&1, Map.keys(nums), fn match -> Map.get(nums, match) end))
# |> Enum.map(&String.split(&1, "", trim: true))

# |> Stream.map(fn row ->
#   Enum.filter(row, fn char ->
#     case Integer.parse(char) do
#       {num, _} -> num
#       _ -> nil
#     end
#   end)
# end)
# |> Enum.map(fn x ->
#   String.to_integer(List.to_string([List.first(x), List.last(x)]))
# end)

# IO.inspect(List.to_string(elem(List.to_tuple(res), 88)))
IO.inspect(res)
