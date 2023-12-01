File.stream!("input.txt")
|> Stream.map(&String.trim(&1))
|> Stream.map(&String.split(&1, "", trim: true))
|> Stream.map(fn row ->
  Enum.filter(row, fn char ->
    case Integer.parse(char) do
      {num, _} -> num
      _ -> nil
    end
  end)
end)
|> Stream.map(fn x ->
  String.to_integer(List.to_string([List.first(x), List.last(x)]))
end)
|> Enum.sum()
|> IO.inspect()

