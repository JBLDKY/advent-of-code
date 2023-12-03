game_id = fn line ->
  String.split(line, ":")
  |> List.first()
  |> String.split(" ")
  |> List.last()
  |> String.to_integer()
end

hands =
  fn line ->
    String.split(line, ":")
    |> List.last()
    |> String.split(";", trim: true)
    |> Enum.map(&String.split(&1, ",", trim: true))
    |> Enum.flat_map(fn colors ->
      Enum.map(colors, &String.split(&1, " ", trim: true))
    end)
  end

input = File.stream!("input.txt")

e_hands = Enum.map(input, hands)
e_id = Enum.map(input, game_id)

combined = Enum.zip(e_id, e_hands)

res =
  Enum.filter(combined, fn x ->
    id = hd(combined)
    idd = hd(id)
    IO.inspect(idd)

    # r =
    #   Enum.filter(combined, fn [amount, color] ->
    #     case [String.trim(color), String.to_integer(amount)] do
    #       ["red", x] -> x > 12
    #       ["green", x] -> x > 13
    #       ["blue", x] -> x > 14
    #     end
    #   end)
  end)

# IO.inspect(res)
