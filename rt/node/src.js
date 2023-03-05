const importObj = {
    Math: {
        random: () => Math.random(),
    }
};

// Node
const data = require("fs").readFileSync("./target/wasm32-unknown-unknown/release/rt.wasm");
WebAssembly.instantiate(data, importObj).then(({instance})=>{
    console.log(instance.exports.add(40, 2)) // returns 42
});

