/**
 * Valid types for function/decorator arguments
 */
export const Types = {
    Float:"Float", Integer:"Integer", Numeric:"Numeric",
    
    // These can be converted from any type
    String:"String", Boolean:"Boolean", Array:"Array", Object:"Object",
    Any:""
}
    
/**
 * A value for use with Lavendeux
 */
export class LavendeuxValue {
    /**
     * Determine the type of an incoming value
     * @param {Object} wrappedValue 
     * @returns Type of the value given
     */
    static typeOf(wrappedValue) {
        let inType = Object.keys(wrappedValue);
        if (inType.length) return inType[0];
        throw new Error('Received an invalid value from Lavendeux');
    }

    /**
     * Cooerce a value to a given type
     * Should mimic Lavendeux's type cooersion
     * @param {Any} value 
     * @param {String} targetType Type to wrap as
     * @returns New value
     */
    static cooerce(value, targetType) {
        switch (targetType) {
            case 'Integer': return Math.floor( Number(value) );
            case 'Numeric':
            case 'Float': 
                return Number(value);
            case 'Boolean': return !!value;
            case 'String':
                if (typeof value === 'object') {
                    return JSON.stringify(value);
                } else {
                    return `${value}`;
                }
            case 'Array': 
                if (Array.isArray(value)) {
                    return value;
                } else if (typeof value === 'object') {
                    return Object.values(value);
                } else {
                    return [value];
                }
            case 'Object':
                if (typeof value === 'object') {
                    return Object.assign({}, value);
                } else {
                    return {0: value};
                }
            default: return value;
        }
    }

    /**
     * Return a raw value
     * @param {Object} wrappedValue 
     * @returns value
     */
    static unwrap(wrappedValue, targetType=Types.Any) {
        let type = this.typeOf(wrappedValue);
        let value = Object.values(wrappedValue)[0];
        switch (type) {
            case 'Object':
                value = value.map(([k,v]) => [
                    this.unwrap(k, Types.String),
                    this.unwrap(v)
                ]);
                value = Object.fromEntries(value);
                break;
            case 'Array':
                value = value.map(e => this.unwrap(e));
                break;
        }

        return LavendeuxValue.cooerce(value, targetType);
    }

    /**
     * Wrap a value for returning to Lavendeux
     * @param {Any} value 
     * @param {String} targetType Type to wrap as
     * @returns Wrapped value
     */
    static wrap(value, targetType=Types.Any) {
        value = this.cooerce(value, targetType);

        if (Array.isArray(value)) {
            return {'Array': value.map(e => this.wrap(e))}
        } else if (typeof value === 'object') {
            let result = [];
            Object.keys(value).forEach(k => {
                result.push([
                    this.wrap(k),
                    this.wrap(value[k])
                ])
            });
            return {'Object': result};
        } else if (typeof value === 'string' || value instanceof String) {
            return {'String': value};
        } else if (Number.isInteger(value)) {
            return {'Integer': value};
        } else if (Number(value) === value) {
            return {'Float': value};
        } else return {'Boolean': value == true};
    }
}
