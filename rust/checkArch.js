const axios = require('axios');
const cherio = require("cherio");

let now = JSON.parse( process.argv[2]);
console.log(now)
axios.get('https://github.com/inkroomtemp/util/pkgs/container/rust')
.then(res=>res.data)
.then(res=>{
    const $ = cherio.load(res);


    let m4 = $('.m-4>small');
    let m = [];
    for(let i = 0;i<m4.length;i++){
        let text = m4.eq(i).text();
        if(text.indexOf('unknown')==-1)
        m.push(text)
    }

    if(m.length != now.length){
        console.log('false');
        return;
    }

    for(let i = 0;i<m.length;i++){
        
        if(now.indexOf(m[i])===-1){
            console.log('false')
            return;
        }
    }
    console.log('true')

})


