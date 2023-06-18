defmodule Twenty48 do
  @size 4
  @range 0..(@size - 1)

  def play, do: setup() |> play

  defp play(board, score \\ 0) do
    show(board, score)

    cond do
      0 in Map.values(board) or combinable?(board) ->
        {moved, score} = move(board, score, input(board, score))
        if moved == board, do: play(board, score), else: add_tile(moved) |> play(score)

      true ->
        IO.puts("Game Over!")
        IO.puts("Final Score: #{score}")
        exit(:normal)
    end
  end

  defp setup do
    for(i <- @range, j <- @range, into: %{}, do: {{i, j}, 0})
    |> add_tile
    |> add_tile
  end

  defp add_tile(board) do
    position = blank_space(board) |> Enum.random()
    tile = if :rand.uniform(10) == 1, do: 4, else: 2
    %{board | position => tile}
  end

  defp blank_space(board) do
    for {key, 0} <- board, do: key
  end

  defp input(board, score) do
    uinput = IO.gets("")

    case String.first(uinput) do
      "w" ->
        :up

      "a" ->
        :left

      "s" ->
        :down

      "d" ->
        :right

      "q" ->
        IO.puts("Game Over!")
        IO.puts("Final Score: #{score}")
        exit(:normal)

      x ->
        IO.puts("Invalid input: #{x}. Valid inputs are w-a-s-d and q.")
        input(board, score)
    end
  end

  defp move(board, score, :up) do
    Enum.reduce(@range, {board, score}, fn cidx, {acc, score} ->
      Enum.map(@range, fn ridx -> acc[{ridx, cidx}] end)
      |> move_and_combine(score)
      |> then(fn {v, score} ->
        {v
         |> Enum.with_index()
         |> Enum.reduce(acc, fn {v, ridx}, map -> Map.put(map, {ridx, cidx}, v) end), score}
      end)
    end)
  end

  defp move(board, score, :left) do
    Enum.reduce(@range, {board, score}, fn ridx, {acc, score} ->
      Enum.map(@range, fn cidx -> acc[{ridx, cidx}] end)
      |> move_and_combine(score)
      |> then(fn {v, score} ->
        {v
         |> Enum.with_index()
         |> Enum.reduce(acc, fn {v, cidx}, map -> Map.put(map, {ridx, cidx}, v) end), score}
      end)
    end)
  end

  defp move(board, score, :down) do
    Enum.reduce(@range, {board, score}, fn cidx, {acc, score} ->
      Enum.map((@size - 1)..0, fn ridx -> acc[{ridx, cidx}] end)
      |> move_and_combine(score)
      |> then(fn {v, score} ->
        {v
         |> Enum.reverse()
         |> Enum.with_index()
         |> Enum.reduce(acc, fn {v, ridx}, map -> Map.put(map, {ridx, cidx}, v) end), score}
      end)
    end)
  end

  defp move(board, score, :right) do
    Enum.reduce(@range, {board, score}, fn ridx, {acc, score} ->
      Enum.map((@size - 1)..0, fn cidx -> acc[{ridx, cidx}] end)
      |> move_and_combine(score)
      |> then(fn {v, score} ->
        {v
         |> Enum.reverse()
         |> Enum.with_index()
         |> Enum.reduce(acc, fn {v, cidx}, map -> Map.put(map, {ridx, cidx}, v) end), score}
      end)
    end)
  end

  defp move_and_combine(tiles, score) do
    (Enum.filter(tiles, &(&1 > 0)) ++ [0, 0, 0, 0])
    |> Enum.take(@size)
    |> case do
      [a, a, b, b] ->
        score = score + a * 2 + b * 2
        {[a * 2, b * 2, 0, 0], score}

      [a, a, b, c] ->
        score = score + a * 2
        {[a * 2, b, c, 0], score}

      [a, b, b, c] ->
        score = score + b * 2
        {[a, b * 2, c, 0], score}

      [a, b, c, c] ->
        score = score + c * 2
        {[a, b, c * 2, 0], score}

      x ->
        {x, score}
    end
  end

  defp combinable?(board) do
    Enum.any?(
      for ridx <- @range,
          cidx <- 0..(@size - 2),
          do: board[{cidx, ridx}] == board[{cidx, ridx + 1}]
    ) or
      Enum.any?(
        for ridx <- @range,
            cidx <- 0..(@size - 2),
            do: board[{cidx, ridx}] == board[{cidx + 1, ridx}]
      )
  end

  defp print_board(board) do
    width =
      board
      |> Enum.map(fn {_, v} -> v end)
      |> Enum.max()
      |> to_string
      |> then(fn str -> String.length(str) + 2 end)

    spacing = String.duplicate("─", width)

    IO.puts("┌#{spacing}┬#{spacing}┬#{spacing}┬#{spacing}┐")

    Enum.each(0..(@size - 2), fn ridx ->
      row =
        Enum.reduce(@range, "", fn cidx, row ->
          cell = board[{ridx, cidx}]

          size =
            cell
            |> to_string
            |> String.length()

          ws_size = width - size - 2
          right_ws_size = div(ws_size, 2)
          left_ws_size = ws_size - right_ws_size

          right_ws = String.duplicate(" ", right_ws_size)
          left_ws = String.duplicate(" ", left_ws_size)

          row =
            if cell == 0 do
              row <> " #{left_ws} #{right_ws} │"
            else
              row <> " #{left_ws}#{cell}#{right_ws} │"
            end

          row
        end)

      IO.puts("│" <> row)
      IO.puts("├#{spacing}┼#{spacing}┼#{spacing}┼#{spacing}┤")
    end)

    row =
      Enum.reduce(@range, "", fn cidx, row ->
        cell = board[{@size - 1, cidx}]

        size =
          cell
          |> to_string
          |> String.length()

        ws_size = width - size - 2
        right_ws_size = div(ws_size, 2)
        left_ws_size = ws_size - right_ws_size

        right_ws = String.duplicate(" ", right_ws_size)
        left_ws = String.duplicate(" ", left_ws_size)

        row =
          if cell == 0 do
            row <> " #{left_ws} #{right_ws} │"
          else
            row <> " #{left_ws}#{cell}#{right_ws} │"
          end

        row
      end)

    IO.puts("│" <> row)
    IO.puts("└#{spacing}┴#{spacing}┴#{spacing}┴#{spacing}┘")
  end

  def show(board, score, verbose \\ false) do
    IEx.Helpers.clear()

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

    print_board(board)
  end
end

Twenty48.play()
