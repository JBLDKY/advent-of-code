enum =
  File.stream!("input.txt")
  |> Stream.map(&String.trim(&1))
  |> Stream.map(&String.split(&1, " "))

forward =
  Stream.filter(enum, fn [x, y] ->
    x == "forward"
  end)
  |> Stream.map(fn [_, val] -> String.to_integer(val) end)
  |> Enum.sum()

down =
  Stream.filter(enum, fn [x, y] ->
    x == "down"
  end)
  |> Stream.map(fn [_, val] -> String.to_integer(val) end)
  |> Enum.sum()

up =
  Stream.filter(enum, fn [x, y] ->
    x == "up"
  end)
  |> Stream.map(fn [_, val] -> String.to_integer(val) end)
  |> Enum.sum()

IO.inspect(forward)
IO.inspect(down)
IO.inspect(up)

