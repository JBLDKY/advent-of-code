enum =
  File.stream!("input.txt")
  |> Stream.map(&String.trim(&1))
  |> Stream.map(&String.split(&1, " "))

solve = fn enum, direction ->
  Stream.filter(enum, fn [x, y] ->
    x == direction
  end)
  |> Stream.map(fn [_, val] -> String.to_integer(val) end)
  |> Enum.sum()
end

f = solve.(enum, "forward")
u = solve.(enum, "up")
d = solve.(enum, "down")

IO.inspect("answer: ")
IO.inspect((d - u) * f)
