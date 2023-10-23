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

    addStringFunction(name, callback, returns = Types.String) {
        return this.addFunction(name, callback, returns);
    }

    addIntegerFunction(name, callback, returns = Types.Integer) {
        return this.addFunction(name, callback, returns);
    }

    addFloatFunction(name, callback, returns = Types.Float) {
        return this.addFunction(name, callback, returns);
    }

    addNumericFunction(name, callback, returns = Types.Numeric) {
        return this.addFunction(name, callback, returns);
    }

    addArrayFunction(name, callback, returns = Types.Array) {
        return this.addFunction(name, callback, returns);
    }

    addObjectFunction(name, callback, returns = Types.Array) {
        return this.addFunction(name, callback, returns);
    }

    addDecorator(name, callback, accepts = Types.Any) {
        this.decorators[name] = new LavendeuxFunction(name, 'decorator', callback, Types.String)
            .requireArgument(accepts);        
    }

    addStringDecorator(name, callback, accepts = Types.String) {
        return this.addDecorator(name, callback, accepts);
    }

    addIntegerDecorator(name, callback, accepts = Types.Integer) {
        return this.addDecorator(name, callback, accepts);
    }

    addFloatDecorator(name, callback, accepts = Types.Float) {
        return this.addDecorator(name, callback, accepts);
    }

    addNumericDecorator(name, callback, accepts = Types.Numeric) {
        return this.addDecorator(name, callback, accepts);
    }

    addArrayDecorator(name, callback, accepts = Types.Array) {
        return this.addDecorator(name, callback, accepts);
    }

    addObjectDecorator(name, callback, accepts = Types.Array) {
        return this.addDecorator(name, callback, accepts);
    }

    export() {
        let properties = {
            'functions': {},
            'decorators': {}
        };
        Object.assign(properties, this.properties);

        for (const name in this.functions) {
            properties.functions[name] = this.functions[name].properties;
        }

        for (const name in this.decorators) {
            properties.decorators[name] = this.decorators[name].properties;
        }

        return properties;
    }

    name() { return this.properties.name; }
    author() { return this.properties.author; }
    version() { return this.properties.version; }
}