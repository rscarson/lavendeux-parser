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
 * @param {Value} args 
 * @returns {Value} result
 */
function functionSet(args) {
    if (args.length != 2) {
        throw new Error("set(<string>, <any>): expected 2 arguments");
    } else if (!args[0].String) {
        throw "set(<string>, <any>): expected a string value";
    }

    let name = args[0].String, value = args[1];
    const state = getState();
    state[name] = value;
    setState(state);

    return value;
}