defmodule Ockam.Stream.Index.Internal do
  @moduledoc """
    In-memory stream index storage
  """

  @behaviour Ockam.Stream.Index

  @impl true
  def init(_options) do
    {:ok, %{}}
  end

  @impl true
  def get_index(client_id, stream_name, _partition, state) do
    id = {client_id, stream_name}
    index = Map.get(state, id, :undefined)

    {{:ok, index}, state}
  end

  @impl true
  def save_index(client_id, stream_name, _partition, index, state) do
    id = {client_id, stream_name}
    state = Map.update(state, id, index, fn previous -> max(previous, index) end)

    {:ok, state}
  end
end
