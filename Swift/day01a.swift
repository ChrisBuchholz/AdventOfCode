import Foundation

let data = NSFileHandle.fileHandleWithStandardInput().availableData
let directions = String(data: data, encoding:NSUTF8StringEncoding)!
var floor = 0

for c in directions.characters {
  switch c {
    case "(": floor += 1
    case ")": floor -= 1
    default: break
  }
}

print(floor)
