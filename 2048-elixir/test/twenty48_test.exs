defmodule Twenty48Test do
  use ExUnit.Case
  doctest Twenty48

  test "greets the world" do
    assert Twenty48.main() == :ok
  end
end
