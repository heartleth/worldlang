const{print,println}=require('./worldlib');
(async ()=>{ let a = 10;println(a);([a][0] = 20);println(a); })();