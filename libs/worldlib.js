const fs = require('fs');

module.exports = {
    print: (...arg) => process.stdout.write(arg.reduce((a,b)=>a+b,'')),
    println: (...arg) => console.log(arg.reduce((a,b)=>a+b,'')),
    input: (q) => {
        print(q);
        let buf = Buffer.alloc(1);
        for (;/^[\s\t\r\n]$/.test(buf.toString());fs.readSync(0,buf,0,1))buf=Buffer.alloc(1);
        let ret = '';
        for (;/^[\s\t\r\n]$/.test(buf.toString());fs.readSync(0,buf,0,1)){
            ret+=buf.toString();
            buf=Buffer.alloc(1);
        }
        return ret.substr(1).trim()
    }
    // ...Todo
}