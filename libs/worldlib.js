module.exports = {
    print: (...arg)=>process.stdout.write(arg.reduce((a,b)=>a+b,'')),
    println: (...arg)=>console.log(arg.reduce((a,b)=>a+b,'')),
    // ...Todo
}