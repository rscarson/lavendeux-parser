/**
 * This function tells Lavendeux about this extension.
 * It must return an object similar to the one below.
 * @returns Object
 */
 function extension() {
    return {
        name: "Regex Utilities",
        author: "@rscarson",
        version: "0.2.0",

        functions: {
            "matches": "function_matches"
        },
    };
}

/**
 * Function returns the first match in a string for the given regular expression
 *  Usage: color(<string>, <pattern>)
 * Can be called from the lavendeux parser
 * It takes in an array of value objects, which will have one of the following properties:
 *  Integer, Float, String
 * 
 * It then returns a value object
 * @param {Value} args 
 * @returns {Value} result
 */
function function_matches(args) {
    if (args.length != 2) {
        throw "color(<string>, <pattern>): expected 2 arguments";
    } else if (!args[0].String || !args[1].String) {
        throw "color(<string>, <pattern>): expected a string value";
    }

    let m = args[0].String.match(args[1].String);
    return {
        "String": m==null ? "" : m[0]
    };
}