const mod = require("../pkg/float_pigment_css.js");

const { compileStyleSheetToJson } = mod;

const result = compileStyleSheetToJson("test", `
.my-class {
  color: #abc;
}
@media (max-width: 800px) {
  .my-class {
      color: 200px;
      flex-flow: column-reverse wrap;
      pointer-events: auto;

    }
}`);

console.log(result);


const re = mod.compileStyleSheetToBincode("test", `
.my-class {
  color: #abc;
}
@media (max-width: 800px) {
  .my-class {
      color: 200px;
      flex-flow: column-reverse wrap;
      pointer-events: auto;

    }
}`);

console.log(re);

