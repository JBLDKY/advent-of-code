
res_map =
  File.stream!("input.txt")
  |> Stream.map(&String.trim(&1))
  |> Stream.map(&String.split(&1, " "))
  |> Stream.map(fn [dir, val] -> [dir, String.to_integer(val)] end)
  |> Enum.reduce(%{"depth" => 0, "horizontal" => 0, "aim" => 0}, fn [dir, val], acc ->
    case dir do
      "up" ->
        Map.update!(acc, "aim", fn prev -> prev - val end)

      "down" ->
        Map.update!(acc, "aim", fn prev -> prev + val end)

      "forward" ->
        %{
          acc
          | "horizontal" => val + acc["horizontal"],
            "depth" => acc["depth"] + val * acc["aim"]
        }
    end
  end)

IO.inspect(res_map["depth"] * res_map["horizontal"])
