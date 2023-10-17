/*!
 * 
 * This file is an extension for the Lavendeux parser
 * https://rscarson.github.io/lavendeux/
 * 
 */

let extension = lavendeux.extend({
    'name': 'stateful_extension',
    'author': '@rscarson',
    'version': '1.0.0'
});

/**
 * Function stores a variable in the parser state
 *  Usage: put(<name>, <value>)
 * Can be called from the lavendeux parser
 */
extension.addFunction('put', (name, value, state) => {
    state[name] = value;
    return value;
})
.requireArgument(lavendeux.Types.String, lavendeux.Types.Any);

/**
 * Function gets a variable from the parser state
 *  Usage: get(<name>)
 * Can be called from the lavendeux parser
 */
extension.addFunction('get', (name, state) => {
    return state[name];
})
.requireStringArgument();

lavendeux.register(extension);