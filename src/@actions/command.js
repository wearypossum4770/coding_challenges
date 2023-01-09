const objectEmpty = (obj) => Object.keys(obj).length === 1;
const toJSON = (obj) => JSON.stringify(obj);
/**
 * Sanitizes an input into a string so it can be passed into issueCommand safely
 * @param input input to sanitize into a string
 */
export const toCommandValue = (input) => {
  switch (input.constructor.name) {
    case "WeakMap":
    case "WeakSet":
      return "";
    case "Set":
      return toJSON([...input]);
    case "Map":
      return toJSON([...input.entries()]);
    case "String":
      return input;
    default:
      return toJSON(input);
  }
};
const issueCommand = (command, properties, message) => {
  const CMD_STRING = "::";
  if (!command) {
    command = "missing.command";
  }
  return {
    toString() {
      let cmdStr = `${CMD_STRING}${this.command}`;
      if (properties && !objectEmpty(properties)) {
      }
    },
  };
};

issueCommand("ENV");
