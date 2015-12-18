import Foundation

enum InputError: ErrorType {
    case NotFound
}

func read_input_file(file_name: String) throws -> String {
    guard
        let path = NSBundle.mainBundle().pathForResource(file_name, ofType: "input")
    else {
            throw InputError.NotFound
    }
    return try! String(contentsOfFile: path, encoding: NSUTF8StringEncoding)
}