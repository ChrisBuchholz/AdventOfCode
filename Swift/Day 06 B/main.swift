let lines = try! read_input_file("day06").characters.split{$0 == "\n"}.map{String($0)}
var lights = [[Int]](count: 1000, repeatedValue: [Int](count: 1000, repeatedValue: 0))
var sumVolume = 0

for l in lines {
    var parts = l.characters.split{$0 == " "}.map{String($0)}.filter{$0 != "turn"}
    let from = parts[1].characters.split{$0 == ","}.map{Int(String($0))!}
    let to = parts[3].characters.split{$0 == ","}.map{Int(String($0))!}
    let action = parts[0]
    
    for x in from[0]...to[0] {
        for y in from[1]...to[1] {
            if action == "on" {
                lights[x][y] += 1
            } else if action == "off" && lights[x][y] > 0 {
                    lights[x][y] -= 1
            } else if action == "toggle" {
                lights[x][y] += 2
            }
        }
    }
}

for x in 0..<1000 {
    for y in 0..<1000 {
        sumVolume += lights[x][y]
    }
}

assert(sumVolume == 14110788)
print("\(sumVolume)")