const fs = require('fs');

let _sin_cache = {};
let _sinl_cache = {};

module.exports = {
    print: (...arg) => process.stdout.write(arg.reduce((a,b)=>a+b,'')),
    println: (...arg) => console.log(arg.reduce((a,b)=>a+b,'')),
    input: (q="") => {
        process.stdout.write(q);
        let buf = Buffer.alloc(1);
        for (;/^[\s\t\r\n]$/.test(buf.toString());fs.readSync(0,buf,0,1))buf=Buffer.alloc(1);
        let ret = '';
        for (;! /^[\s\t\r\n]$/.test(buf.toString());fs.readSync(0,buf,0,1)){
            ret+=buf.toString();
            buf=Buffer.alloc(1);
        }
        return ret.substr(1).trim()
    },
    input_line: (q="") => {
        process.stdout.write(q);
        let buf = Buffer.alloc(256);
        fs.readSync(0, buf, 0, 256);
        return buf.toString();
    },
    static_input: (etag, q="") => (_sin_cache[etag]??(_sin_cache[etag]=input(q))),
    static_input_line: (etag, q="") => (_sinl_cache[etag]??(_sinl_cache[etag]=input_line(q))),
    tup: (...args)=>args,
    vec: (...args)=>args,
    until: (begin, end)=>[...Array(end-begin+1).keys()].map(e=>e+begin),
    sum: (c)=>[...c].reduce((a,b)=>(a+b)),
    max: (c)=>[Math.max(...c)],
    map: (c,fn)=>[...c].map(fn),
    cat: (c1,c2)=>[...c1,...c2],
    each: (c,fn)=>[...c].forEach(fn),
    fold: (c,fn)=>[...c].reduce(fn),
    bfold: (i,c,fn)=>[...c].reduce(fn, i),
    integrate: (c,fn)=>{
        let cc=[...c];
        let ret = [cc[0]];
        for (let i = 1; i < cc.length; i++) {
            ret.push(fn(ret[i-1],cc[i]));
        }
        return ret;
    },
    filter: (c,fn)=>[...c].filter(fn),
    wait: t=>{
        let waitTill = new Date(new Date().getTime() + t);
        while(waitTill > new Date()){}
    },
    get_time: (fn, ...args)=>{
        let st=Date.now();
        fn(...args);
        let ed=Date.now();
        this.println(`${(ed-st).toFixed(6)} second(s) spent. \n`)
    },
    make_string: (s)=>`${s}`,
    stoi: (s)=>Number.parseInt(s),
    string: String,
    vector: Array,
    i1: Number,
    i2: Number,
    i4: Number,
    i8: Number,
    u1: Number,
    u2: Number,
    u4: Number,
    u8: Number,
    f4: Number,
    f8: Number,
    ci1: Number,
    ci2: Number,
    ci4: Number,
    ci8: Number,
    cu1: Number,
    cu2: Number,
    cu4: Number,
    cu8: Number,
    cf4: Number,
    cf8: Number
}