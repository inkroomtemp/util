const axios = require('axios');
// 获取 docker镜像 platforms
axios.get(`https://hub.docker.com/v2/namespaces/${process.argv[2]}/repositories/${process.argv[3]}/tags/${process.argv[4]}`)
    .then(res => {
        let out = '';
        for (let index = 0; index < res.data.images.length; index++) {
            const element = res.data.images[index];
            out += element.os +'/'+element.architecture;
            if(element.variant){
                out+='/'+element.variant;
            }
            out +=','
        }

        console.log(out.substring(0,out.length-1));
    })



