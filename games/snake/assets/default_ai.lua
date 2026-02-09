-- Default snake AI: chase the food while avoiding walls and self.
--
-- The `think` function is called every tick with the current game state:
--   state.snake     — array of {x, y} positions (index 1 = head)
--   state.food      — {x, y} position of the food
--   state.direction  — current direction ("north", "south", "east", "west")
--   state.grid_size — width/height of the grid (32)
--   state.score     — current score
--
-- Return one of: "north", "south", "east", "west"

function think(state)
    local head = state.snake[1]
    local food = state.food

    -- Try to move toward the food, but never reverse into ourselves.
    if food.x > head.x and state.direction ~= "west" then
        return "east"
    elseif food.x < head.x and state.direction ~= "east" then
        return "west"
    elseif food.y > head.y and state.direction ~= "south" then
        return "north"
    elseif food.y < head.y and state.direction ~= "north" then
        return "south"
    end

    -- No better option — keep going.
    return state.direction
end
