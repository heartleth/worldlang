const readline = require("readline");

module.exports = {
    print: (...arg) => process.stdout.write(arg.reduce((a,b)=>a+b,'')),
    println: (...arg) => console.log(arg.reduce((a,b)=>a+b,'')),
    input: (q) => {
        const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout
        });
        const s = await new Promise(p=>rl.question(q, (s)=>{
            p(s);
        }));            
        rl.close();
        return s;
    }
    // ...Todo
}