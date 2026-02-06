export interface Game {
  id: number
  name: string
  display_name: string
}

export async function fetchGames(): Promise<Game[]> {
  const response = await fetch('/api/games')
  if (!response.ok) {
    throw new Error('Failed to fetch games')
  }
  return response.json()
}

export async function fetchGame(name: string): Promise<Game> {
  const response = await fetch(`/api/games/${name}`)
  if (!response.ok) {
    throw new Error('Failed to fetch game')
  }
  return response.json()
}
