let extension = lavendeux.extend({
    'name': 'stateful_extension',
    'author': '@rscarson',
    'version': '1.0.0'
});

extension.addFunction('put', (name, value, state) => {
    state[name] = value;
    return value;
})
.requireArgument('String')
.requireArgument();

extension.addFunction('get', (name, state) => {
    return state[name];
})
.requireArgument('String');

lavendeux.register(extension);