/*!
 * 
 * This file is an extension for the Lavendeux parser
 * https://rscarson.github.io/lavendeux/
 * 
 */

let extension = lavendeux.extend({
    'name': 'regex_utility',
    'author': '@rscarson',
    'version': '1.0.0'
});

/**
 * Function returns the first match in a string for the given regular expression
 *  Usage: matches(<string>, <pattern>)
 * Can be called from the lavendeux parser
 */
extension.addStringFunction('matches', (s, pattern) => {
    let m =  s.match(pattern);
    return m==null ? "" : m[0];
})
.requireArguments(
    lavendeux.Types.String, 
    lavendeux.Types.String
);

lavendeux.register(extension);