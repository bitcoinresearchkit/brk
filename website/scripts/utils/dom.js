/**
 * @param {string} id
 * @returns {HTMLElement}
 */
export function getElementById(id) {
  const element = window.document.getElementById(id);
  if (!element) throw `Element with id = "${id}" should exist`;
  return element;
}

/**
 * @param {HTMLElement} element
 */
export function isHidden(element) {
  return element.tagName !== "BODY" && !element.offsetParent;
}

/**
 *
 * @param {HTMLElement} element
 * @param {VoidFunction} callback
 */
export function onFirstIntersection(element, callback) {
  const observer = new IntersectionObserver((entries) => {
    for (let i = 0; i < entries.length; i++) {
      if (entries[i].isIntersecting) {
        callback();
        observer.disconnect();
      }
    }
  });
  observer.observe(element);
}

/**
 * @param {string} name
 */
export function createSpanName(name) {
  const spanName = window.document.createElement("span");
  spanName.classList.add("name");
  const [first, second, third] = name.split(" - ");
  spanName.innerHTML = first;

  if (second) {
    const smallRest = window.document.createElement("small");
    smallRest.innerHTML = ` — ${second}`;
    spanName.append(smallRest);

    if (third) {
      throw "Shouldn't have more than one dash";
    }
  }

  return spanName;
}

/**
 * @param {Object} arg
 * @param {string} arg.href
 * @param {string} arg.title
 * @param {string} [arg.text]
 * @param {boolean} [arg.blank]
 * @param {VoidFunction} [arg.onClick]
 * @param {boolean} [arg.preventDefault]
 */
export function createAnchorElement({
  text,
  href,
  blank,
  onClick,
  title,
  preventDefault,
}) {
  const anchor = window.document.createElement("a");
  anchor.href = href;
  anchor.title = title.toUpperCase();

  if (text) {
    anchor.innerText = text;
  }

  if (blank) {
    anchor.target = "_blank";
    anchor.rel = "noopener noreferrer";
  }

  if (onClick || preventDefault) {
    if (onClick) {
      anchor.addEventListener("click", (event) => {
        event.preventDefault();
        onClick();
      });
    }
  }

  return anchor;
}

/**
 * @param {Object} arg
 * @param {string | HTMLElement} arg.inside
 * @param {string} arg.title
 * @param {(event: MouseEvent) => void} arg.onClick
 */
export function createButtonElement({ inside: text, onClick, title }) {
  const button = window.document.createElement("button");

  button.append(text);

  button.title = title.toUpperCase();

  button.addEventListener("click", onClick);

  return button;
}

/**
 * @param {Object} args
 * @param {string} args.inputName
 * @param {string} args.inputId
 * @param {string} args.inputValue
 * @param {boolean} [args.inputChecked=false]
 * @param {string} [args.title]
 * @param {'radio' | 'checkbox'} args.type
 * @param {(event: MouseEvent) => void} [args.onClick]
 */
export function createLabeledInput({
  inputId,
  inputName,
  inputValue,
  inputChecked = false,
  title,
  onClick,
  type,
}) {
  const label = window.document.createElement("label");

  inputId = inputId.toLowerCase();

  const input = window.document.createElement("input");
  if (type === "radio") {
    input.type = "radio";
    input.name = inputName;
  } else {
    input.type = "checkbox";
  }
  input.id = inputId;
  input.value = inputValue;
  input.checked = inputChecked;
  label.append(input);

  label.id = `${inputId}-label`;
  if (title) {
    label.title = title;
  }
  label.htmlFor = inputId;

  if (onClick) {
    label.addEventListener("click", onClick);
  }

  return {
    label,
    input,
  };
}

/**
 * @param {HTMLElement} parent
 * @param {HTMLElement} child
 * @param {number} index
 */
export function insertElementAtIndex(parent, child, index) {
  if (!index) index = 0;
  if (index >= parent.children.length) {
    parent.appendChild(child);
  } else {
    parent.insertBefore(child, parent.children[index]);
  }
}

/**
 * @param {string} url
 * @param {boolean} [targetBlank]
 */
export function open(url, targetBlank) {
  console.log(`open: ${url}`);
  const a = window.document.createElement("a");
  window.document.body.append(a);
  a.href = url;

  if (targetBlank) {
    a.target = "_blank";
    a.rel = "noopener noreferrer";
  }

  a.click();
  a.remove();
}

/**
 * @param {string} href
 */
export function importStyle(href) {
  const link = document.createElement("link");
  link.href = href;
  link.type = "text/css";
  link.rel = "stylesheet";
  link.media = "screen,print";
  const head = window.document.getElementsByTagName("head")[0];
  head.appendChild(link);
  return link;
}

/**
 * @template T
 * @param {Object} args
 * @param {T} args.initialValue
 * @param {string} [args.id]
 * @param {readonly T[]} args.choices
 * @param {(value: T) => void} [args.onChange]
 * @param {(choice: T) => string} [args.toKey]
 * @param {(choice: T) => string} [args.toLabel]
 */
export function createRadios({
  id,
  choices,
  initialValue,
  onChange,
  toKey = /** @type {(choice: T) => string} */ ((/** @type {any} */ c) => c),
  toLabel = /** @type {(choice: T) => string} */ ((/** @type {any} */ c) => c),
}) {
  const field = window.document.createElement("div");
  field.classList.add("field");

  const div = window.document.createElement("div");
  field.append(div);

  const initialKey = toKey(initialValue);

  /** @param {string} key */
  const fromKey = (key) =>
    choices.find((c) => toKey(c) === key) ?? initialValue;

  if (choices.length === 1) {
    const span = window.document.createElement("span");
    span.textContent = toLabel(choices[0]);
    div.append(span);
  } else {
    const fieldId = id ?? "";
    choices.forEach((choice) => {
      const choiceKey = toKey(choice);
      const choiceLabel = toLabel(choice);
      const { label } = createLabeledInput({
        inputId: `${fieldId}-${choiceKey.toLowerCase()}`,
        inputName: fieldId,
        inputValue: choiceKey,
        inputChecked: choiceKey === initialKey,
        type: "radio",
      });

      const text = window.document.createTextNode(choiceLabel);
      label.append(text);
      div.append(label);
    });

    field.addEventListener("change", (event) => {
      // @ts-ignore
      onChange?.(fromKey(event.target.value));
    });
  }

  return field;
}

/**
 * @template T
 * @param {Object} args
 * @param {T} args.initialValue
 * @param {string} [args.id]
 * @param {readonly T[]} args.choices
 * @param {(value: T) => void} [args.onChange]
 * @param {(choice: T) => string} [args.toKey]
 * @param {(choice: T) => string} [args.toLabel]
 * @param {boolean} [args.sorted]
 */
export function createSelect({
  id,
  choices: unsortedChoices,
  initialValue,
  onChange,
  sorted,
  toKey = /** @type {(choice: T) => string} */ ((/** @type {any} */ c) => c),
  toLabel = /** @type {(choice: T) => string} */ ((/** @type {any} */ c) => c),
}) {
  const choices = sorted
    ? unsortedChoices.toSorted((a, b) => toLabel(a).localeCompare(toLabel(b)))
    : unsortedChoices;

  const select = window.document.createElement("select");
  select.id = id ?? "";
  select.name = id ?? "";

  const initialKey = toKey(initialValue);

  /** @param {string} key */
  const fromKey = (key) =>
    choices.find((c) => toKey(c) === key) ?? initialValue;

  choices.forEach((choice) => {
    const option = window.document.createElement("option");
    option.value = toKey(choice);
    option.textContent = toLabel(choice);
    if (toKey(choice) === initialKey) {
      option.selected = true;
    }
    select.append(option);
  });

  select.addEventListener("change", () => {
    onChange?.(fromKey(select.value));
  });

  return select;
}

/**
 * @param {string} [title]
 * @param {1 | 2 | 3} [level]
 */
export function createHeader(title = "", level = 1) {
  const headerElement = window.document.createElement("header");

  const headingElement = window.document.createElement(`h${level}`);
  headingElement.innerHTML = title;
  headerElement.append(headingElement);
  headingElement.style.display = "block";

  return {
    headerElement,
    headingElement,
  };
}

/**
 * @template {string} Name
 * @template {string} Value
 * @template {Value | {name: Name; value: Value}} T
 * @param {T} arg
 */
export function createOption(arg) {
  const option = window.document.createElement("option");
  if (typeof arg === "object") {
    option.value = arg.value;
    option.innerText = arg.name;
  } else {
    option.value = arg;
    option.innerText = arg;
  }
  return option;
}


/**
 * @param {Object} args
 * @param {string} args.title
 * @param {string} args.description
 * @param {HTMLElement} args.input
 */
export function createFieldElement({ title, description, input }) {
  const div = window.document.createElement("div");

  const label = window.document.createElement("label");
  div.append(label);

  const titleElement = window.document.createElement("span");
  titleElement.innerHTML = title;
  label.append(titleElement);

  const descriptionElement = window.document.createElement("small");
  descriptionElement.innerHTML = description;
  label.append(descriptionElement);

  div.append(input);

  const forId = input.id || input.firstElementChild?.id;

  if (!forId) {
    console.log(input);
    throw `Input should've an ID`;
  }

  label.htmlFor = forId;

  return div;
}

/**
 * @param {'left' | 'bottom' | 'top' | 'right'} position
 */
export function createShadow(position) {
  const div = window.document.createElement("div");
  div.classList.add(`shadow-${position}`);
  return div;
}
