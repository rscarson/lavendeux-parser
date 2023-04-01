/**
 * This function tells Lavendeux about this extension.
 * It must return an object similar to the one below.
 * @returns Object
 */
function extension() {
    return {
        name: "Stateful function demo",
        author: "@rscarson",
        version: "0.2.0",

        functions: {
            "set": "functionSet"
        },
    };
}

/**
 * Functions can also be stateful, gaining access to the parser's variables
 * It takes in arguments and a state, a hash of strings and values
 * 
 * It then returns a single value, or a [value, state] pair to mutate the parser state
 * @param {Value} args 
 * @returns {Value} result
 */
function functionSet(args, state) {
    if (args.length != 2) {
        throw new Error("set(<string>, <any>): expected 2 arguments");
    } else if (!args[0].String) {
        throw "set(<string>, <any>): expected a string value";
    }

    let name = args[0].String, value = args[1];
    state[name] = value;
    return [value, state];
}