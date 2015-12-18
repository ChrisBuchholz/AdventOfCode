let directions = try! read_input_file("day01")
var floor = 0

for c in directions.characters {
    switch c {
        case "(": floor += 1
        case ")": floor -= 1
        default: break
    }
}

assert(floor == 232)
print(floor)

