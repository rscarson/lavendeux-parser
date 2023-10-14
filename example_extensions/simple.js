let extension = lavendeux.extend({
    'name': 'simple_extension',
    'author': '@rscarson',
    'version': '1.0.0'
});

extension.addFunction('add', (left, right) => left + right, 'Float')
.requireArgument('Integer').requireArgument('Integer');

extension.addDecorator('usd', (input) => `${input}`, 'Float')

lavendeux.register(extension);