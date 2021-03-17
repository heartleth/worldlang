// Repl for enpp-node.js

const CMD = filenamewithoutext => `worldlang ${filenamewithoutext} nodejs.json`;

const cp = require('child_process');
const fs = require('fs');

function transpile(code) {
    fs.writeFileSync('enpprepl.epp', code);
    const exec = cp.exec(CMD('enpprepl'));
    return new Promise(p=>{
        exec.on('exit', code=>{
            if (code != 0) {
                console.error(exec.stderr);
                p();
            }
            else {
                const contents = fs.readFileSync('enpprepl.js').toString();
                fs.unlinkSync('enpprepl.epp');
                fs.unlinkSync('enpprepl.js');
                p(contents);
            }
        });
    });
}

(async ()=>{
    const stdlib = fs.readFileSync(__dirname + '/worldlib_repl.js');
    let code_remains = false;
    let to_remain = false;
    let is_first = true;
    let remains = '';
    let indents = 0;
    let mem = '';
    
    while (1) {
        let evaluate = false;
        console.log();
        if (is_first) process.stdout.write(' >> ');
        else for (let i=0;i<=indents;i++) process.stdout.write(i==indents?' .. ':'    ');

        const line = (await new Promise(e=>process.stdin.on('data', data=>{
            e(data.toString());
        }))).trim();
        
        if (line.length > 0) {
            const keyword = line.split(' ')[0];
            mem += '    '.repeat(indents) + line + '\n';
            if (/^(let|set|class|when|import|library)$/i.test(keyword)) {
                to_remain = true;
                code_remains = true;
            }
            if (/^(if|when|class|for|else|while|use|public|private|protected)$/i.test(keyword)) {
                is_first = false;
                indents++;
                code_remains = false;
            }
            else if (is_first) {
                evaluate = true;
            }
        }
        else if (!is_first) {
            if (indents > 1) indents--;
            else {
                evaluate = !to_remain;
                code_remains = true;
            }
        }
        
        if (evaluate) {
            const code = stdlib + '(()=>{' + (await transpile(remains + mem)) + '})()';
            (new Function(code))();
            if (code_remains) {
                remains += mem;
            }
            
            to_remain = false;
            code_remains = false;
            indents = 0;
            is_first = true;
            mem = '';
        }
        if (code_remains) {
            remains += mem;
            to_remain = false;
            code_remains = false;
            indents = 0;
            is_first = true;
            mem = '';
        }
    }
})();