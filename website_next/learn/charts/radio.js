let groupId = 0;

/**
 * @template {string} T
 * @param {Object} args
 * @param {string} args.legend
 * @param {readonly { value: T, label: string }[]} args.options
 * @param {T} args.currentValue
 * @param {(value: T) => void} args.onChange
 */
export function createRadioGroup(args) {
  const fieldset = document.createElement("fieldset");
  const legend = document.createElement("legend");
  const name = `chart-control-${(groupId += 1)}`;

  legend.append(args.legend);
  fieldset.append(legend);
  fieldset.addEventListener("change", (event) => {
    const input = /** @type {HTMLInputElement} */ (event.target);

    args.onChange(/** @type {T} */ (input.value));
  });

  for (const option of args.options) {
    const label = document.createElement("label");
    const input = document.createElement("input");
    const text = document.createElement("span");

    input.type = "radio";
    input.name = name;
    input.value = option.value;
    input.checked = option.value === args.currentValue;

    text.append(option.label);
    label.append(input, text);
    fieldset.append(label);
  }

  return fieldset;
}
