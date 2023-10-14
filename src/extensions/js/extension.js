import { LavendeuxFunction } from 'ext:lavendeux/function.js';
import { Types } from 'ext:lavendeux/value.js'; 

export class LavendeuxExtension {
    constructor(properties) {
        if (false && !['name', 'author', 'version'].every(k => {
            properties.hasOwnProperty(k)
        })) {
            throw new Error("Properties given are missing one of ['name', 'author', 'version']");
        }
        
        this.properties = properties;
        this.functions = {};
        this.decorators = {};
    }

    addFunction(name, callback, returns = 'Any') {
        this.functions[name] = new LavendeuxFunction(name, 'function', callback, returns);
        return this.functions[name];
    }

    addDecorator(name, callback, accepts = 'Any') {
        this.decorators[name] = new LavendeuxFunction(name, 'decorator', callback, Types.String)
            .requireArgument(accepts);        
    }

    export() {
        let properties = {
            'function_definitions': {},
            'decorator_definitions': {}
        };
        Object.assign(properties, this.properties);

        for (const name in this.functions) {
            properties.function_definitions[name] = this.functions[name].properties;
        }

        for (const name in this.decorators) {
            properties.decorator_definitions[name] = this.decorators[name].properties;
        }

        return properties;
    }

    name() { return this.properties.name; }
    author() { return this.properties.author; }
    version() { return this.properties.version; }
}