let lines = try! read_input_file("day06").characters.split{$0 == "\n"}.map{String($0)}
var lights = [[Bool]](count: 1000, repeatedValue: [Bool](count: 1000, repeatedValue: false))
var litLights = 0

for l in lines {
    var parts = l.characters.split{$0 == " "}.map{String($0)}.filter{$0 != "turn"}
    let from = parts[1].characters.split{$0 == ","}.map{Int(String($0))!}
    let to = parts[3].characters.split{$0 == ","}.map{Int(String($0))!}
    let action = parts[0]
    
    for x in from[0]...to[0] {
        for y in from[1]...to[1] {
            if action == "on" {
                lights[x][y] = true
            } else if action == "off" {
                lights[x][y] = false
            } else {
                lights[x][y] = !lights[x][y]
            }
        }
    }
}

for x in 0..<1000 {
    for y in 0..<1000 {
        if lights[x][y] {
            litLights += 1
        }
    }
}

assert(litLights == 377891)
print("\(litLights)")