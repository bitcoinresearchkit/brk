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
 * @param {T} args.defaultValue - Fallback when selected value is no longer in choices
 * @param {string} [args.id]
 * @param {readonly T[] | Accessor<readonly T[]>} args.choices
 * @param {boolean} [args.sorted]
 * @param {Signals} args.signals
 * @param {Signal<T>} args.selected
 * @param {(choice: T) => string} [args.toKey] - Extract string key (defaults to identity for strings)
 * @param {(choice: T) => string} [args.toLabel] - Extract display label (defaults to identity for strings)
 * @param {"radio" | "select"} [args.type] - Render as radio buttons or select dropdown
 */
export function createReactiveChoiceField({
  id,
  choices: unsortedChoices,
  defaultValue,
  signals,
  selected,
  sorted,
  toKey = /** @type {(choice: T) => string} */ ((/** @type {any} */ c) => c),
  toLabel = /** @type {(choice: T) => string} */ ((/** @type {any} */ c) => c),
  type = /** @type {const} */ ("radio"),
}) {
  const defaultKey = toKey(defaultValue);

  const choices = signals.createMemo(() => {
    /** @type {readonly T[]} */
    let c;
    if (typeof unsortedChoices === "function") {
      c = unsortedChoices();
    } else {
      c = unsortedChoices;
    }

    return sorted
      ? /** @type {readonly T[]} */ (
          /** @type {any} */ (
            c.toSorted((a, b) => toLabel(a).localeCompare(toLabel(b)))
          )
        )
      : c;
  });

  /** @param {string} key */
  const fromKey = (key) =>
    choices().find((c) => toKey(c) === key) ?? defaultValue;

  const field = window.document.createElement("div");
  field.classList.add("field");

  const div = window.document.createElement("div");
  field.append(div);

  /** @type {HTMLElement | null} */
  let remainingSmall = null;
  if (type === "select") {
    remainingSmall = window.document.createElement("small");
    field.append(remainingSmall);
  }

  signals.createScopedEffect(choices, (choices) => {
    const s = selected();
    const sKey = toKey(s);
    const keys = choices.map(toKey);
    if (!keys.includes(sKey)) {
      if (keys.includes(defaultKey)) {
        selected.set(() => defaultValue);
      } else if (choices.length) {
        selected.set(() => choices[0]);
      }
    }

    div.innerHTML = "";

    if (choices.length === 1) {
      const span = window.document.createElement("span");
      span.textContent = toLabel(choices[0]);
      div.append(span);

      if (remainingSmall) {
        remainingSmall.hidden = true;
      }
    } else if (type === "select") {
      const select = window.document.createElement("select");
      select.id = id ?? "";
      select.name = id ?? "";

      choices.forEach((choice) => {
        const option = window.document.createElement("option");
        option.value = toKey(choice);
        option.textContent = toLabel(choice);
        if (toKey(choice) === sKey) {
          option.selected = true;
        }
        select.append(option);
      });

      select.addEventListener("change", () => {
        selected.set(() => fromKey(select.value));
      });

      div.append(select);

      if (remainingSmall) {
        const remaining = choices.length - 1;
        if (remaining > 0) {
          remainingSmall.textContent = ` +${remaining}`;
          remainingSmall.hidden = false;
        } else {
          remainingSmall.hidden = true;
        }
      }
    } else {
      const fieldId = id ?? "";
      choices.forEach((choice) => {
        const choiceKey = toKey(choice);
        const choiceLabel = toLabel(choice);
        const { label } = createLabeledInput({
          inputId: `${fieldId}-${choiceKey.toLowerCase()}`,
          inputName: fieldId,
          inputValue: choiceKey,
          inputChecked: choiceKey === sKey,
          // title: choiceLabel,
          type: "radio",
        });

        const text = window.document.createTextNode(choiceLabel);
        label.append(text);
        div.append(label);
      });

      field.addEventListener("change", (event) => {
        // @ts-ignore
        const value = event.target.value;
        selected.set(() => fromKey(value));
      });
    }
  });

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
 * @param {"radio" | "select"} [args.type]
 */
export function createChoiceField({
  id,
  choices,
  initialValue,
  onChange,
  toKey = /** @type {(choice: T) => string} */ ((/** @type {any} */ c) => c),
  toLabel = /** @type {(choice: T) => string} */ ((/** @type {any} */ c) => c),
  type = "radio",
}) {
  const field = window.document.createElement("div");
  field.classList.add("field");

  const div = window.document.createElement("div");
  field.append(div);

  const initialKey = toKey(initialValue);

  /** @param {string} key */
  const fromKey = (key) =>
    choices.find((c) => toKey(c) === key) ?? initialValue;

  if (type === "select") {
    const select = window.document.createElement("select");
    select.id = id ?? "";
    select.name = id ?? "";

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

    div.append(select);
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
 * @template {string} Name
 * @template {string} Value
 * @template {Value | {name: Name; value: Value}} T
 * @param {Object} args
 * @param {string} [args.id]
 * @param {boolean} [args.deep]
 * @param {readonly ((T) | {name: string; list: T[]})[]} args.list
 * @param {Signal<T>} args.signal
 */
export function createSelect({ id, list, signal, deep = false }) {
  const select = window.document.createElement("select");

  if (id) {
    select.name = id;
    select.id = id;
  }

  /** @type {Record<string, VoidFunction>} */
  const setters = {};

  list.forEach((anyOption, index) => {
    if (typeof anyOption === "object" && "list" in anyOption) {
      const { name, list } = anyOption;
      const optGroup = window.document.createElement("optgroup");
      optGroup.label = name;
      select.append(optGroup);
      list.forEach((option) => {
        optGroup.append(createOption(option));
        const key = /** @type {string} */ (
          typeof option === "object" ? option.value : option
        );
        setters[key] = () => signal.set(() => option);
      });
    } else {
      select.append(createOption(anyOption));
      const key = /** @type {string} */ (
        typeof anyOption === "object" ? anyOption.value : anyOption
      );
      setters[key] = () => signal.set(() => anyOption);
    }
    if (deep && index !== list.length - 1) {
      select.append(window.document.createElement("hr"));
    }
  });

  select.addEventListener("change", () => {
    const callback = setters[select.value];
    // @ts-ignore
    if (callback) {
      callback();
    }
  });

  const initialSignal = signal();
  const initialValue =
    typeof initialSignal === "object" ? initialSignal.value : initialSignal;
  select.value = String(initialValue);

  return { select, signal };
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
