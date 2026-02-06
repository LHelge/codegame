export interface Game {
  id: number
  name: string
  wasm_filename: string
}

export async function fetchGames(): Promise<Game[]> {
  const response = await fetch('/api/games')
  if (!response.ok) {
    throw new Error('Failed to fetch games')
  }
  return response.json()
}

export async function fetchGame(id: number): Promise<Game> {
  const response = await fetch(`/api/games/${id}`)
  if (!response.ok) {
    throw new Error('Failed to fetch game')
  }
  return response.json()
}
