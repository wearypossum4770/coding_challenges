export default function USPhoneValidator(action, input_string) {
  let regExMatch = input_string.match(/[0-9]{3}[-]{1}[0-9]{3}[-]{1}[0-9]{4}/g);
  switch (action) {
    default:
      return "No action given";
    case "HAS":
      return regExMatch ?? false;
    case "GET":
      return regExMatch ?? undefined;
    case "GET_ALL":
      return regExMatch ?? [];
    case "HIDE":
      return regExMatch.map((value) =>
        value.replace(/[0-9]{3}[-]{1}[0-9]{3}[-]{1}/g, " XXX-XXX-")
      ); //.trim()
    case "FORMAT":
      return;
  }
}
