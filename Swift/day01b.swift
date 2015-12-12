import Foundation

let data = NSFileHandle.fileHandleWithStandardInput().availableData
let directions = String(data: data, encoding:NSUTF8StringEncoding)!
var floor = 0, pos = 0

for c in directions.characters {
  pos += 1
  switch c {
    case "(": floor += 1
    case ")": floor -= 1
    default: break
  }
  if floor == -1 {
    break
  }
}

assert(pos == 1783)
print(pos)

