// https://www.netsparker.com/web-vulnerability-scanner/vulnerabilities/
// https://www.netsparker.com/blog/web-security/csrf-vulnerability-grammarly-cloud-based-service/
export default class Guitar {
  guitarProps = {};
  strings = (strings = false) => {
    this.guitarProps.strings = strings;
    return this;
  };
  type = (type) => {
    this.guitarProps.type = type;
    return this;
  };
}
let b = new Guitar();
console.log(b);
const { strings, guitarProps } = new Guitar();

strings(true);

console.log(guitarProps); // { strings: true }
