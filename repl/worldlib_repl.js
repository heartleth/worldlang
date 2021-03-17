

let _sin_cache = {};
let _sinl_cache = {};

const print = (...arg) => process.stdout.write(arg.reduce((a,b)=>a+b,''));
const println = (...arg) => console.log(arg.reduce((a,b)=>a+b,''));
const input = (q="") => {
    process.stdout.write(q);
    let buf = Buffer.alloc(1);
    for (;/^[\s\t\r\n]$/.test(buf.toString());fs.readSync(0,buf,0,1))buf=Buffer.alloc(1);
    let ret = '';
    for (;! /^[\s\t\r\n]$/.test(buf.toString());fs.readSync(0,buf,0,1)){
        ret+=buf.toString();
        buf=Buffer.alloc(1);
    }
    return ret.substr(1).trim()
};
const input_line = (q="") => {
    process.stdout.write(q);
    let buf = Buffer.alloc(256);
    fs.readSync(0, buf, 0, 256);
    return buf.toString();
};
const static_input = (etag, q="") => (_sin_cache[etag]??(_sin_cache[etag]=input(q)));
const static_input_line = (etag, q="") => (_sinl_cache[etag]??(_sinl_cache[etag]=input_line(q)));
const tup = (...args)=>args;
const vec = (...args)=>args;
const until = (begin, end)=>[...Array(end-begin+1).keys()].map(e=>e+begin);
const sum = (c)=>[...c].reduce((a,b)=>(a+b));
const max = (c)=>[Math.max(...c)];
const map = (c,fn)=>[...c].map(fn);
const cat = (c1,c2)=>[...c1,...c2];
const each = (c,fn)=>[...c].forEach(fn);
const fold = (c,fn)=>[...c].reduce(fn);
const bfold = (i,c,fn)=>[...c].reduce(fn, i);
const integrate = (c,fn)=>{
    let cc=[...c];
    let ret = [cc[0]];
    for (let i = 1; i < cc.length; i++) {
        ret.push(fn(ret[i-1],cc[i]));
    }
    return ret;
};
const filter = (c,fn)=>[...c].filter(fn);
const wait = t=>{
    let waitTill = new Date(new Date().getTime() + t);
    while(waitTill > new Date()){}
};
const get_time = (fn, ...args)=>{
    let st=Date.now();
    fn(...args);
    let ed=Date.now();
    this.println(`${(ed-st).toFixed(6)} second(s) spent. \n`)
};
const make_string = (s)=>`${s}`;
const stoi = (s)=>Number.parseInt(s);
const string = String;
const vector = Array;
const i1 = Number;
const i2 = Number;
const i4 = Number;
const i8 = Number;
const u1 = Number;
const u2 = Number;
const u4 = Number;
const u8 = Number;
const f4 = Number;
const f8 = Number;
const ci1 = Number;
const ci2 = Number;
const ci4 = Number;
const ci8 = Number;
const cu1 = Number;
const cu2 = Number;
const cu4 = Number;
const cu8 = Number;
const cf4 = Number;
const cf8 = Number;