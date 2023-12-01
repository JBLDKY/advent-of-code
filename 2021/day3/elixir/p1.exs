map =
  Enum.reduce(0..11, %{}, fn x, acc ->
    Map.put_new(acc, x, {})
  end)

IO.inspect(map)

flattened_bits =
  File.stream!("input.txt")
  |> Stream.map(&String.trim(&1))
  |> Stream.map(&String.split(&1, "", trim: true))
  |> Enum.to_list()
  |> List.flatten()
  |> Enum.reduce(map, fn x, acc -> 
  %{acc
      | 
    }
end)

res = for bit <- flattened_bits, idx <- 0..(length(flattened_bits) - 1), into: %{}, do: {idx, bit}
IO.inspect(res)
