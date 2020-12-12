function step_part_1(data)
    new_gen = deepcopy(data)
    n_rows = length(data)
    n_cols = length(data[1])
    changes = 0
    for row in 1:n_rows
        for col in 1:n_cols
            if data[row][col] != "."
                adjacents = 0
                for dr in (row > 1 ? -1 : 0):(row < n_rows ? 1 : 0)
                    for dc in (col > 1 ? -1 : 0):(col < n_cols ? 1 : 0)
                        if (dr != 0 || dc != 0) && data[row + dr][col + dc] == "#"
                            adjacents += 1
                        end
                    end
                end
                if data[row][col] == "L" && adjacents == 0
                    new_gen[row][col] = "#"
                    changes += 1
                elseif data[row][col] == "#" && adjacents >= 4
                    new_gen[row][col] = "L"
                    changes += 1
                end
            end
        end
    end
    return new_gen, changes
end

function find_final_config_part_1(data)
    gen, changes = step_part_1(data)
    while changes != 0
        gen, changes = step_part_1(gen)
    end
    return gen
end

function step_part_2(data) # I know it's terrible pls don't kill me
    new_gen = deepcopy(data)
    n_rows = length(data)
    n_cols = length(data[1])
    changes = 0
    for row in 1:n_rows
        for col in 1:n_cols
            if data[row][col] != "."
                adjacents = 0
                # Vertical
                # up
                for dr in row - 1:-1:1
                    if data[dr][col] != "."
                        if data[dr][col] == "#"
                            adjacents += 1
                        end
                        break
                    end
                end
                # down
                for dr in row + 1:n_rows
                    if data[dr][col] != "."
                        if data[dr][col] == "#"
                            adjacents += 1
                        end
                        break
                    end
                end

                # Horizontal
                # left
                for dc in col - 1:-1:1
                    if data[row][dc] != "."
                        if data[row][dc] == "#"
                            adjacents += 1
                        end
                        break
                    end
                end
                # right
                for dc in col + 1:n_cols
                    if data[row][dc] != "."
                        if data[row][dc] == "#"
                            adjacents += 1
                        end
                        break
                    end
                end

                # Diags
                # up left
                di = 1
                while row - di >= 1 && col - di >= 1
                    if data[row - di][col - di] != "."
                        if data[row - di][col - di] == "#"
                            adjacents += 1
                        end
                        break
                    end
                    di += 1
                end
                # down right
                di = 1
                while row + di <= n_rows && col + di <= n_cols
                    if data[row + di][col + di] != "."
                        if data[row + di][col + di] == "#"
                            adjacents += 1
                        end
                        break
                    end
                    di += 1
                end
                # up right
                di = 1
                while row - di >= 1 && col + di <= n_cols
                    if data[row - di][col + di] != "."
                        if data[row - di][col + di] == "#"
                            adjacents += 1
                        end
                        break
                    end
                    di += 1
                end
                # down left
                di = 1
                while row + di <= n_rows && col - di >= 1
                    if data[row + di][col - di] != "."
                        if data[row + di][col - di] == "#"
                            adjacents += 1
                        end
                        break
                    end
                    di += 1
                end

                # Eval
                if data[row][col] == "L" && adjacents == 0
                    new_gen[row][col] = "#"
                    changes += 1
                elseif data[row][col] == "#" && adjacents >= 5
                    new_gen[row][col] = "L"
                    changes += 1
                end
            end
        end
    end
    return new_gen, changes
end


function find_final_config_part_2(data)
    gen, changes = step_part_2(data)
    while changes != 0
        gen, changes = step_part_2(gen)
    end
    return gen
end

function count_occupied(data)
    occupied = 0
    for row in data
        for x in row
            if x == "#"
                occupied += 1
            end
        end
    end
    return occupied
end

data = open("2020\\d11\\input.txt") do f
    split.(readlines(f), "")
end;

t1 = @elapsed final_part_1 = find_final_config_part_1(data)
occupied_part_1 = count_occupied(final_part_1)
println("Occupied seats in part 1: $occupied_part_1 (Done in $(t1)s)")

t2 = @elapsed final_part_2 = find_final_config_part_2(data)
occupied_part_2 = count_occupied(final_part_2)
println("Occupied seats in part 2: $occupied_part_2 (Done in $(t2)s)")
println("Finished in $(t1 + t2)s")