defmodule Twenty48 do
  @size 4
  @range 0..(@size - 1)

  def play, do: setup() |> play

  defp play(field) do
    Map.put_new(field, :score, 0)
    show(field)

    # cond do
    #   0 in Map.values(field) or combinable?(field) ->
    #     moved = move(field, input())
    #     if moved == field, do: play(field), else: add_tile(moved) |> play

    #   true ->
    #     IO.puts("Game Over")
    #     score = Map.get(field, :score, 0)
    #     IO.puts("Final Score: #{score}")
    #     exit(:normal)
    # end
  end

  defp setup do
    for(i <- @range, j <- @range, into: %{}, do: {{i, j}, 0})
    |> add_tile
    |> add_tile
  end

  defp add_tile(field) do
    position = blank_space(field) |> Enum.random()
    tile = if :rand.uniform(10) == 1, do: 4, else: 2
    %{field | position => tile}
  end

  defp blank_space(field) do
    for {key, 0} <- field, do: key
  end

  defp input do
    uinput = IO.gets("")

    case String.first(uinput) do
      "w" -> :up
      "a" -> :left
      "s" -> :down
      "d" -> :right
      "q" -> exit(:normal)
      _ -> input()
    end
  end

  defp move(field, :up) do
    Enum.reduce(@range, field, fn cidx, acc ->
      Enum.map(@range, fn ridx -> acc[{ridx, cidx}] end)
      |> then(fn tiles -> move_and_combine(field, tiles) end)
      |> Enum.with_index()
      |> Enum.reduce(acc, fn {v, ridx}, map -> Map.put(map, {ridx, cidx}, v) end)
    end)
  end

  defp move_and_combine(field, tiles) do
    (Enum.filter(tiles, &(&1 > 0)) ++ [0, 0, 0, 0])
    |> Enum.take(@size)
    |> case do
      [a, a, b, b] ->
        Map.update!(field, :score, fn v -> v + a * 2 + b * 2 end)
        [a * 2, b * 2, 0, 0]

      [a, a, b, c] ->
        Map.update!(field, :score, fn v -> v + a * 2 end)
        [a * 2, b, c, 0]

      [a, b, b, c] ->
        Map.update!(field, :score, fn v -> v + b * 2 end)
        [a, b * 2, c, 0]

      [a, b, c, c] ->
        Map.update!(field, :score, fn v -> v + c * 2 end)
        [a, b, c * 2, 0]

      x ->
        x
    end
  end

  defp combinable?(field) do
    Enum.any?(
      for ridx <- @range,
          cidx <- 0..(@size - 2),
          do: field[{cidx, ridx}] == field[{cidx, ridx + 1}]
    ) or
      Enum.any?(
        for ridx <- @range,
            cidx <- 0..(@size - 2),
            do: field[{cidx, ridx}] == field[{cidx + 1, ridx}]
      )
  end

  defp print_board(field) do
    width =
      field
      |> Enum.map(fn {_, v} -> v end)
      |> Enum.max()
      |> to_string
      |> then(fn str -> String.length(str) + 2 end)

    spacing = String.duplicate("─", width)

    IO.puts("┌#{spacing}┬#{spacing}┬#{spacing}┬#{spacing}┐")

    Enum.each(0..(@size - 2), fn ridx ->
      row = "│"

      Enum.each(@range, fn cidx ->
        cell = field[{ridx, cidx}]

        size =
          cell
          |> to_string
          |> String.length()

        ws_size = width - size - 2
        right_ws_size = div(ws_size, 2)
        left_ws_size = ws_size - right_ws_size

        right_ws = String.duplicate(" ", right_ws_size)
        left_ws = String.duplicate(" ", left_ws_size)

        if cell == 0 do
          row = row <> " #{left_ws} #{right_ws} ^"
        else
          row = row <> " #{left_ws}#{cell}#{right_ws} ^"
        end
      end)

      IO.puts(row)
      IO.puts("├#{spacing}┼#{spacing}┼#{spacing}┼#{spacing}┤")
    end)

    row = "│"

    Enum.each(@range, fn cidx ->
      cell = field[{@size - 1, cidx}]

      size =
        cell
        |> to_string
        |> String.length()

      ws_size = width - size - 2
      right_ws_size = div(ws_size, 2)
      left_ws_size = ws_size - right_ws_size

      right_ws = String.duplicate(" ", right_ws_size)
      left_ws = String.duplicate(" ", left_ws_size)

      if cell == 0 do
        row = row <> " #{left_ws} #{right_ws} ^"
      else
        row = row <> " #{left_ws}#{cell}#{right_ws} ^"
      end
    end)

    IO.puts(row)
    IO.puts("└#{spacing}┴#{spacing}┴#{spacing}┴#{spacing}┘")
  end

  def show(field, verbose \\ false) do
    IEx.Helpers.clear()

    score = Map.get(field, :score, 0)

    IO.puts("2048")

    if verbose do
      IO.puts("---------------------")
      IO.puts("\nControls:")
      IO.puts("W - Shift cells up")
      IO.puts("A - Shift cells left")
      IO.puts("S - Shift cells down")
      IO.puts("D - Shift cells right")
      IO.puts("Q - Quit\n")
      IO.puts("Score: #{score}\n")
    else
      IO.puts("----")
      IO.puts("Score #{score}\n")
    end

    print_board(field)
  end
end

Twenty48.play()
