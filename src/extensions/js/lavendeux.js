import { LavendeuxFunction } from 'ext:lavendeux/function.js';
import { LavendeuxExtension } from 'ext:lavendeux/extension.js';
import { applyToGlobal, nonEnumerable } from 'ext:js_playground/js_playground.js';

class Lavendeux {
    constructor() {
        this.state = {};
        this.functionCache = {
            'function': {},
            'decorator': {}
        }
    }

    storeFunction(name, type, callback) {
        this.functionCache[type][name] = callback;
    }

    retrieveFunction(name, type) {
        return this.functionCache[type][name];
    }

    setState(s) {
        this.state = s;
    }

    getState() {
        return this.state;
    }

    extend(properties) {
        return new LavendeuxExtension(properties);
    }

    register(extension) {
        globalThis._registered_lavendeux_extension = extension.export();
        let functions = Object.values(extension.functions);
        for (const entry of functions) {
            lavendeux.storeFunction(entry.properties.fname, entry.properties.ftype, entry.callback);
        }
        js_playground.register_entrypoint(() => globalThis._registered_lavendeux_extension);
    }
}

applyToGlobal({
    lavendeux: nonEnumerable(
      new Lavendeux(),
    ),

    setLavendeuxState: nonEnumerable(
        (s) => globalThis.lavendeux.setState(s)
    ),
    getLavendeuxState: nonEnumerable(
        () => globalThis.lavendeux.getState()
    ),
    callLavendeuxFunction: nonEnumerable(
        (p, ...a) => LavendeuxFunction.call(p, ...a)
    ),
});