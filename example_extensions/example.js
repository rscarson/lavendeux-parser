"use strict";Object.defineProperty(exports,Symbol.toStringTag,{value:"Module"});const Types={Float:"Float",Integer:"Integer",Numeric:"Numeric",String:"String",Boolean:"Boolean",Array:"Array",Object:"Object",Any:""};class LavendeuxValue{static typeOf(wrappedValue){let inType=Object.keys(wrappedValue);return inType.length?inType[0]:!1}static cooerce(value,targetType){switch(targetType){case"Integer":return Math.floor(Number(value));case"Numeric":case"Float":return Number(value);case"Boolean":return!!value;case"String":return typeof value=="object"?JSON.stringify(value):`${value}`;case"Array":return Array.isArray(value)?value:typeof value=="object"?Object.values(value):[value];case"Object":return typeof value=="object"?Object.assign({},value):{0:value};default:return value}}static unwrap(wrappedValue,targetType=Types.Any){let type=this.typeOf(wrappedValue),value=Object.values(wrappedValue)[0];switch(type){case"Object":value=value.map(([k,v])=>[this.unwrap(k,Types.String),this.unwrap(v)]),value=Object.fromEntries(value);break;case"Array":value=value.map(e=>this.unwrap(e));break}return LavendeuxValue.cooerce(value,targetType)}static wrap(value,targetType=Types.Any){if(value=this.cooerce(value,targetType),Array.isArray(value))return{Array:value.map(e=>this.wrap(e))};if(typeof value=="object"){let result=[];return Object.keys(value).forEach(k=>{result.push([this.wrap(k),this.wrap(value[k])])}),{Object:result}}else return typeof value=="string"||value instanceof String?{String:value}:Number.isInteger(value)?{Integer:value}:Number(value)===value?{Float:value}:{Boolean:value==!0}}}let WARNING_STATE_UNAVAILABLE,setState,getState;if(typeof getState>"u"){WARNING_STATE_UNAVAILABLE=!0;const _lav_state={};getState=()=>_lav_state,setState=s=>Object.assign(_lav_state,s)}class LavendeuxFunction{constructor(name,returnType,callback){this.name=name.replace("@",""),this.callback=callback,this.argumentTypes=[],this.returnType=returnType,this.registeredName=LavendeuxFunction.getRegisteredName(name)}static isStateAvailable(){return WARNING_STATE_UNAVAILABLE}static getRegisteredName(name){return`lavendeuxfunction_${name}`}addArgument(type=Types.Any){return this.argumentTypes.push(type),this}addIntegerArgument(){return this.addArgument(Types.Integer)}addFloatArgument(){return this.addArgument(Types.Float)}addNumericArgument(){return this.addArgument(Types.Numeric)}addStringArgument(){return this.addArgument(Types.String)}addBooleanArgument(){return this.addArgument(Types.Boolean)}addArrayArgument(){return this.addArgument(Types.Array)}addObjectArgument(){return this.addArgument(Types.Object)}decodeArguments(argv){if(argv.length<this.argumentTypes.length)throw new Error(`Missing a parameter for ${this.name}: Expected ${this.argumentTypes.length} arguments`);return this.argumentTypes.forEach((type,i)=>{let _type=LavendeuxValue.typeOf(argv[i]);if(type==Types.Numeric&&![Types.Integer,Types.Float].includes(_type)||[Types.Integer,Types.Float].includes(type)&&type!=_type)throw new Error(`Invalid type for parameter ${i+1} of ${this.name}: Expected ${type}`)}),argv.map((wrappedValue,i)=>{let type=this.argumentTypes[i]?this.argumentTypes[i]:Types.Any;return LavendeuxValue.unwrap(wrappedValue,type)})}getState(){const state=getState();return Object.keys(state).map(k=>{state[k]=LavendeuxValue.unwrap(state[k])}),state}setState(state){Object.keys(state).map(k=>{state[k]=LavendeuxValue.wrap(state[k])}),setState(state)}call(argv){argv=this.decodeArguments(argv);let state=this.getState(),value=LavendeuxValue.wrap(this.callback(...argv,state),this.returnType);return this.setState(state),value}}class LavendeuxDecorator extends LavendeuxFunction{constructor(name,argumentType,callback){super(name,Types.String,callback),this.argumentTypes=[argumentType],this.registeredName=LavendeuxDecorator.getRegisteredName(name)}static getRegisteredName(name){return`lavendeuxdecorator_${name}`}call(arg){return LavendeuxValue.unwrap(super.call([arg]),Types.String)}}class Lavendeux{constructor(name,author,version){this.name=name,this.author=author,this.version=version,this.functions={},this.decorators={},this.allHandlers={}}static register(instance){globalThis.extension=()=>instance.definition(),globalThis.extensionInstance=instance,Object.values(instance.allHandlers).forEach(f=>{globalThis[f.registeredName]=argv=>f.call(argv)})}static registeredInstance(){return globalThis.extensionInstance}definition(){return{name:this.name,author:this.author,version:this.version,functions:this.functions,decorators:this.decorators}}getFunctionCallback(name){return this.allHandlers[this.functions[name]].callback}getDecoratorCallback(name){return this.allHandlers[this.decorators[name]].callback}addFunction(name,callback,expectedType=Types.Any){let f=new LavendeuxFunction(name,expectedType,callback);return this.allHandlers[f.registeredName]=f,this.functions[name]=f.registeredName,f}addIntegerFunction(name,callback){return this.addFunction(name,callback,Types.Integer)}addFloatFunction(name,callback){return this.addFunction(name,callback,Types.Float)}addNumericFunction(name,callback){return this.addFunction(name,callback,Types.Numeric)}addStringFunction(name,callback){return this.addFunction(name,callback,Types.String)}addBooleanFunction(name,callback){return this.addFunction(name,callback,Types.Boolean)}addArrayFunction(name,callback){return this.addFunction(name,callback,Types.Array)}addObjectFunction(name,callback){return this.addFunction(name,callback,Types.Object)}addDecorator(name,callback,expectedType=Types.Any){let f=new LavendeuxDecorator(name,expectedType,callback);return this.allHandlers[f.registeredName]=f,this.decorators[name]=f.registeredName,f}addIntegerDecorator(name,callback){return this.addDecorator(name,callback,Types.Integer)}addFloatDecorator(name,callback){return this.addDecorator(name,callback,Types.Float)}addNumericDecorator(name,callback){return this.addDecorator(name,callback,Types.Numeric)}addStringDecorator(name,callback){return this.addDecorator(name,callback,Types.String)}addBooleanDecorator(name,callback){return this.addDecorator(name,callback,Types.Boolean)}addArrayDecorator(name,callback){return this.addDecorator(name,callback,Types.Array)}addObjectDecorator(name,callback){return this.addDecorator(name,callback,Types.Object)}}exports.Lavendeux=Lavendeux;
/*!
 * 
 * This file is an extension for the Lavendeux parser
 * https://rscarson.github.io/lavendeux/
 * 
 * The contents below were autogenerated using the lavendeux npm package:
 * https://www.npmjs.com/package/lavendeux
 * 
 */
const lavendeux = {"Lavendeux": Lavendeux};
const name = "example_extension";
const version = "1.0.0";
const author = "@rscarson";
let instance = new lavendeux.Lavendeux(name, author, version);
instance.addNumericDecorator("usd", (input) => {
  let n = (Math.round(input * 100) / 100).toFixed(2);
  return `$${n}`;
});
instance.addFunction("add", (left, right) => {
  return left + right;
}).addNumericArgument().addNumericArgument();
lavendeux.Lavendeux.register(instance);



const extension = lavendeux.extend({
  'name': 'my_extension',
  'author': '@rscarson',
  'version': '1.0.0'
});

extension.addFunction("add", (left, right) => {
  return left + right;
})
.requireNumericArgument()
.requireNumericArgument();
