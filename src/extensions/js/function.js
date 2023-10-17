import {LavendeuxValue, Types} from 'ext:lavendeux/value.js';

export class LavendeuxFunction {
    constructor(name, type, callback, returns) {
        this.callback = callback;
        this.properties = {
            'fname': name,
            'ftype': type,
            'returns': returns,
            'argument_types': []
        };
    }

    requireArgument(type = Types.Any) {
        this.properties.argument_types.push(type);
        return this;
    }

    requireStringArgument(type = Types.String) {
        return this.requireArgument(type);
    }

    requireIntegerArgument(type = Types.Integer) {
        return this.requireArgument(type);
    }

    requireFloatArgument(type = Types.Float) {
        return this.requireArgument(type);
    }

    requireNumericArgument(type = Types.Numeric) {
        return this.requireArgument(type);
    }

    requireArrayArgument(type = Types.Array) {
        return this.requireArgument(type);
    }

    requireObjectArgument(type = Types.Array) {
        return this.requireArgument(type);
    }

    requireArguments(...types) {
        for (const type of types) {
            this.requireArgument(type);
        }
        return this;
    }

    static unwrapLavendeuxFunctionArguments(expectedArgumentTypes, args) {
        let types = args.map(a => LavendeuxValue.typeOf(a));
        if (expectedArgumentTypes.length > args.length) {
            throw new Error(`function expected ${expectedArgumentTypes.length} parameters, but only received ${args.length}`);
        }
        for (const expectedTypeIndex in expectedArgumentTypes) {
            let expectedType = expectedArgumentTypes[expectedTypeIndex];
            let actualType = types[expectedTypeIndex];
    
            // No cooersion needed - the function does not care about type
            if (expectedType == Types.Any) continue;
    
            // This case is not valid as only numeric types can be cooerced to to numeric
            if (
                (expectedType == Types.Numeric && ![Types.Integer, Types.Float].includes(actualType)) ||
                ([Types.Integer, Types.Float].includes(expectedType) && expectedType != actualType)
            ) {
                throw new Error(`Argument ${expectedTypeIndex+1}: expected ${expectedType}, but received ${actualType}`);
            }
        }
    
        // In all other cases we can use type cooersion
        return args.map((a,i) => LavendeuxValue.unwrap(a, expectedArgumentTypes[i]));
    }

    static call(functionProperties, ...args) {
        let state = getLavendeuxState();
        for (const key of Object.keys(state)) {
            state[key] = LavendeuxValue.unwrap(state[key]);
        }

        let js_args = LavendeuxFunction.unwrapLavendeuxFunctionArguments(functionProperties.argument_types, args);
        let callback = lavendeux.retrieveFunction(functionProperties.fname, functionProperties.ftype);
        if (!callback) throw new Error(`Could not find a function named ${functionProperties.fname} to call`);
    
        let value = LavendeuxValue.wrap(
            callback(...js_args, state),
            functionProperties.returns
        );
    
        for (const key of Object.keys(state)) {
            state[key] = LavendeuxValue.wrap(state[key]);
        }
        setLavendeuxState(state);

        return value;
    }
}