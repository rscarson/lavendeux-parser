/*!
 * 
 * This file is an extension for the Lavendeux parser
 * https://rscarson.github.io/lavendeux/
 * 
 */

let extension = lavendeux.extend({
    'name': 'simple_extension',
    'author': '@rscarson',
    'version': '1.0.0'
});

/**
 * Function adds the 2 operands and returns the result as an integer
 *  Usage: add(<number>, <number>)
 * Can be called from the lavendeux parser
 */
extension.addIntegerFunction(
    'add', 
    (left, right) => left + right
).requireArguments(
    lavendeux.Types.Numeric, 
    lavendeux.Types.Numeric
);

/**
 * Formats an integer as a hex color code
 *  Usage: <number> @usd
 * Can be called from the lavendeux parser
 */
extension.addIntegerDecorator(
    'colour', 
    (input) => `#${(input & 0x00FFFFFF).toString(16).padEnd(6, '0')}`
);

lavendeux.register(extension);