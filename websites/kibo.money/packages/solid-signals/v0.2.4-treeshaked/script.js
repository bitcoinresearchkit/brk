// @ts-nocheck

// src/core/error.ts
var NotReadyError = class extends Error {};
var EffectError = class extends Error {
  constructor(effect, cause) {
    super("");
    this.cause = cause;
  }
};

// src/core/constants.ts
var STATE_CLEAN = 0;
var STATE_CHECK = 1;
var STATE_DIRTY = 2;
var STATE_DISPOSED = 3;
var EFFECT_PURE = 0;
var EFFECT_RENDER = 1;
var EFFECT_USER = 2;

// src/core/scheduler.ts
var clock = 0;
function getClock() {
  return clock;
}
function incrementClock() {
  clock++;
}
var scheduled = false;
function schedule() {
  if (scheduled) return;
  scheduled = true;
  if (!globalQueue.w) queueMicrotask(flushSync);
}
var Queue = class {
  w = false;
  l = [[], [], []];
  u = [];
  created = clock;
  enqueue(type, node) {
    this.l[0].push(node);
    if (type) this.l[type].push(node);
    schedule();
  }
  run(type) {
    if (this.l[type].length) {
      if (type === EFFECT_PURE) {
        runPureQueue(this.l[type]);
        this.l[type] = [];
      } else {
        const effects = this.l[type];
        this.l[type] = [];
        runEffectQueue(effects);
      }
    }
    let rerun = false;
    for (let i = 0; i < this.u.length; i++) {
      rerun = this.u[i].run(type) || rerun;
    }
    if (type === EFFECT_PURE && this.l[type].length) return true;
  }
  flush() {
    if (this.w) return;
    this.w = true;
    try {
      while (this.run(EFFECT_PURE)) {}
      incrementClock();
      scheduled = false;
      this.run(EFFECT_RENDER);
      this.run(EFFECT_USER);
    } finally {
      this.w = false;
    }
  }
  addChild(child) {
    this.u.push(child);
  }
  removeChild(child) {
    const index = this.u.indexOf(child);
    if (index >= 0) this.u.splice(index, 1);
  }
};
var globalQueue = new Queue();
function flushSync() {
  while (scheduled) {
    globalQueue.flush();
  }
}
function runTop(node) {
  const ancestors = [];
  for (let current = node; current !== null; current = current.m) {
    if (current.a !== STATE_CLEAN) {
      ancestors.push(current);
    }
  }
  for (let i = ancestors.length - 1; i >= 0; i--) {
    if (ancestors[i].a !== STATE_DISPOSED) ancestors[i].p();
  }
}
function runPureQueue(queue) {
  for (let i = 0; i < queue.length; i++) {
    if (queue[i].a !== STATE_CLEAN) runTop(queue[i]);
  }
}
function runEffectQueue(queue) {
  for (let i = 0; i < queue.length; i++) queue[i].K();
}

// src/core/owner.ts
var currentOwner = null;
var defaultContext = {};
function getOwner() {
  return currentOwner;
}
function setOwner(owner) {
  const out = currentOwner;
  currentOwner = owner;
  return out;
}
var Owner = class {
  // We flatten the owner tree into a linked list so that we don't need a pointer to .firstChild
  // However, the children are actually added in reverse creation order
  // See comment at the top of the file for an example of the _nextSibling traversal
  m = null;
  h = null;
  n = null;
  a = STATE_CLEAN;
  g = null;
  i = defaultContext;
  j = null;
  f = globalQueue;
  constructor(signal = false) {
    if (currentOwner && !signal) currentOwner.append(this);
  }
  append(child) {
    child.m = this;
    child.n = this;
    if (this.h) this.h.n = child;
    child.h = this.h;
    this.h = child;
    if (child.i !== this.i) {
      child.i = { ...this.i, ...child.i };
    }
    if (this.j) {
      child.j = !child.j ? this.j : [...child.j, ...this.j];
    }
    if (this.f) child.f = this.f;
  }
  dispose(self = true) {
    if (this.a === STATE_DISPOSED) return;
    let head = self ? this.n || this.m : this,
      current = this.h,
      next = null;
    while (current && current.m === this) {
      current.dispose(true);
      current.q();
      next = current.h;
      current.h = null;
      current = next;
    }
    if (self) this.q();
    if (current) current.n = !self ? this : this.n;
    if (head) head.h = current;
  }
  q() {
    if (this.n) this.n.h = null;
    this.m = null;
    this.n = null;
    this.i = defaultContext;
    this.j = null;
    this.a = STATE_DISPOSED;
    this.emptyDisposal();
  }
  emptyDisposal() {
    if (!this.g) return;
    if (Array.isArray(this.g)) {
      for (let i = 0; i < this.g.length; i++) {
        const callable = this.g[i];
        callable.call(callable);
      }
    } else {
      this.g.call(this.g);
    }
    this.g = null;
  }
  handleError(error) {
    if (!this.j) throw error;
    let i = 0,
      len = this.j.length;
    for (i = 0; i < len; i++) {
      try {
        this.j[i](error, this);
        break;
      } catch (e) {
        error = e;
      }
    }
    if (i === len) throw error;
  }
};
function onCleanup(fn) {
  if (!currentOwner) return fn;
  const node = currentOwner;
  if (!node.g) {
    node.g = fn;
  } else if (Array.isArray(node.g)) {
    node.g.push(fn);
  } else {
    node.g = [node.g, fn];
  }
  return fn;
}

// src/core/flags.ts
var ERROR_OFFSET = 0;
var ERROR_BIT = 1 << ERROR_OFFSET;
var LOADING_OFFSET = 1;
var LOADING_BIT = 1 << LOADING_OFFSET;
var UNINITIALIZED_OFFSET = 2;
var UNINITIALIZED_BIT = 1 << UNINITIALIZED_OFFSET;
var DEFAULT_FLAGS = ERROR_BIT;

// src/core/core.ts
var currentObserver = null;
var currentMask = DEFAULT_FLAGS;
var newSources = null;
var newSourcesIndex = 0;
var newFlags = 0;
var notStale = false;
var UNCHANGED = Symbol(0);
var Computation = class extends Owner {
  b = null;
  c = null;
  e;
  B;
  r;
  // Used in __DEV__ mode, hopefully removed in production
  O;
  // Using false is an optimization as an alternative to _equals: () => false
  // which could enable more efficient DIRTY notification
  C = isEqual;
  L;
  /** Whether the computation is an error or has ancestors that are unresolved */
  d = 0;
  /** Which flags raised by sources are handled, vs. being passed through. */
  D = DEFAULT_FLAGS;
  E = null;
  s = -1;
  x = false;
  constructor(initialValue, compute2, options) {
    super(compute2 === null);
    this.r = compute2;
    this.a = compute2 ? STATE_DIRTY : STATE_CLEAN;
    this.d = compute2 && initialValue === void 0 ? UNINITIALIZED_BIT : 0;
    this.e = initialValue;
    if (options?.equals !== void 0) this.C = options.equals;
    if (options?.unobserved) this.L = options?.unobserved;
  }
  M() {
    if (this.r) {
      if (this.d & ERROR_BIT && this.s <= getClock()) update(this);
      else this.p();
    }
    if (!this.r || this.b?.length) track(this);
    newFlags |= this.d & ~currentMask;
    if (this.d & ERROR_BIT) {
      throw this.B;
    } else {
      return this.e;
    }
  }
  /**
   * Return the current value of this computation
   * Automatically re-executes the surrounding computation when the value changes
   */
  read() {
    return this.M();
  }
  /**
   * Return the current value of this computation
   * Automatically re-executes the surrounding computation when the value changes
   *
   * If the computation has any unresolved ancestors, this function waits for the value to resolve
   * before continuing
   */
  wait() {
    if (this.r && this.d & ERROR_BIT && this.s <= getClock()) {
      update(this);
    }
    if ((notStale || this.d & UNINITIALIZED_BIT) && this.loading()) {
      throw new NotReadyError();
    }
    return this.M();
  }
  /**
   * Return true if the computation is the value is dependent on an unresolved promise
   * Triggers re-execution of the computation when the loading state changes
   *
   * This is useful especially when effects want to re-execute when a computation's
   * loading state changes
   */
  loading() {
    if (this.E === null) {
      this.E = loadingState(this);
    }
    return this.E.read();
  }
  /** Update the computation with a new value. */
  write(value, flags = 0, raw = false) {
    const newValue =
      !raw && typeof value === "function" ? value(this.e) : value;
    const valueChanged =
      newValue !== UNCHANGED &&
      (!!(this.d & UNINITIALIZED_BIT) ||
        this.C === false ||
        !this.C(this.e, newValue));
    if (valueChanged) {
      this.e = newValue;
      this.B = void 0;
    }
    const changedFlagsMask = this.d ^ flags,
      changedFlags = changedFlagsMask & flags;
    this.d = flags;
    this.s = getClock() + 1;
    if (this.c) {
      for (let i = 0; i < this.c.length; i++) {
        if (valueChanged) {
          this.c[i].k(STATE_DIRTY);
        } else if (changedFlagsMask) {
          this.c[i].N(changedFlagsMask, changedFlags);
        }
      }
    }
    return this.e;
  }
  /**
   * Set the current node's state, and recursively mark all of this node's observers as STATE_CHECK
   */
  k(state, skipQueue) {
    if (this.a >= state && !this.x) return;
    this.x = !!skipQueue;
    this.a = state;
    if (this.c) {
      for (let i = 0; i < this.c.length; i++) {
        this.c[i].k(STATE_CHECK, skipQueue);
      }
    }
  }
  /**
   * Notify the computation that one of its sources has changed flags.
   *
   * @param mask A bitmask for which flag(s) were changed.
   * @param newFlags The source's new flags, masked to just the changed ones.
   */
  N(mask, newFlags2) {
    if (this.a >= STATE_DIRTY) return;
    if (mask & this.D) {
      this.k(STATE_DIRTY);
      return;
    }
    if (this.a >= STATE_CHECK) return;
    const prevFlags = this.d & mask;
    const deltaFlags = prevFlags ^ newFlags2;
    if (newFlags2 === prevFlags);
    else if (deltaFlags & prevFlags & mask) {
      this.k(STATE_CHECK);
    } else {
      this.d ^= deltaFlags;
      if (this.c) {
        for (let i = 0; i < this.c.length; i++) {
          this.c[i].N(mask, newFlags2);
        }
      }
    }
  }
  F(error) {
    this.B = error;
    this.write(
      UNCHANGED,
      (this.d & ~LOADING_BIT) | ERROR_BIT | UNINITIALIZED_BIT,
    );
  }
  /**
   * This is the core part of the reactivity system, which makes sure that the values are updated
   * before they are read. We've also adapted it to return the loading state of the computation,
   * so that we can propagate that to the computation's observers.
   *
   * This function will ensure that the value and states we read from the computation are up to date
   */
  p() {
    if (this.a === STATE_DISPOSED) {
      throw new Error("Tried to read a disposed computation");
    }
    if (this.a === STATE_CLEAN) {
      return;
    }
    let observerFlags = 0;
    if (this.a === STATE_CHECK) {
      for (let i = 0; i < this.b.length; i++) {
        this.b[i].p();
        observerFlags |= this.b[i].d;
        if (this.a === STATE_DIRTY) {
          break;
        }
      }
    }
    if (this.a === STATE_DIRTY) {
      update(this);
    } else {
      this.write(UNCHANGED, observerFlags);
      this.a = STATE_CLEAN;
    }
  }
  /**
   * Remove ourselves from the owner graph and the computation graph
   */
  q() {
    if (this.a === STATE_DISPOSED) return;
    if (this.b) removeSourceObservers(this, 0);
    super.q();
  }
};
function loadingState(node) {
  const prevOwner = setOwner(node.m);
  const options = void 0;
  const computation = new Computation(
    void 0,
    () => {
      track(node);
      node.p();
      return !!(node.d & LOADING_BIT);
    },
    options,
  );
  computation.D = ERROR_BIT | LOADING_BIT;
  setOwner(prevOwner);
  return computation;
}
function track(computation) {
  if (currentObserver) {
    if (
      !newSources &&
      currentObserver.b &&
      currentObserver.b[newSourcesIndex] === computation
    ) {
      newSourcesIndex++;
    } else if (!newSources) newSources = [computation];
    else if (computation !== newSources[newSources.length - 1]) {
      newSources.push(computation);
    }
  }
}
function update(node) {
  const prevSources = newSources,
    prevSourcesIndex = newSourcesIndex,
    prevFlags = newFlags;
  newSources = null;
  newSourcesIndex = 0;
  newFlags = 0;
  try {
    node.dispose(false);
    node.emptyDisposal();
    const result = compute(node, node.r, node);
    node.write(result, newFlags, true);
  } catch (error) {
    if (error instanceof NotReadyError) {
      node.write(
        UNCHANGED,
        newFlags | LOADING_BIT | (node.d & UNINITIALIZED_BIT),
      );
    } else {
      node.F(error);
    }
  } finally {
    if (newSources) {
      if (node.b) removeSourceObservers(node, newSourcesIndex);
      if (node.b && newSourcesIndex > 0) {
        node.b.length = newSourcesIndex + newSources.length;
        for (let i = 0; i < newSources.length; i++) {
          node.b[newSourcesIndex + i] = newSources[i];
        }
      } else {
        node.b = newSources;
      }
      let source;
      for (let i = newSourcesIndex; i < node.b.length; i++) {
        source = node.b[i];
        if (!source.c) source.c = [node];
        else source.c.push(node);
      }
    } else if (node.b && newSourcesIndex < node.b.length) {
      removeSourceObservers(node, newSourcesIndex);
      node.b.length = newSourcesIndex;
    }
    newSources = prevSources;
    newSourcesIndex = prevSourcesIndex;
    newFlags = prevFlags;
    node.s = getClock() + 1;
    node.a = STATE_CLEAN;
  }
}
function removeSourceObservers(node, index) {
  let source;
  let swap;
  for (let i = index; i < node.b.length; i++) {
    source = node.b[i];
    if (source.c) {
      swap = source.c.indexOf(node);
      source.c[swap] = source.c[source.c.length - 1];
      source.c.pop();
      if (!source.c.length) source.L?.();
    }
  }
}
function isEqual(a, b) {
  return a === b;
}
function untrack(fn) {
  if (currentObserver === null) return fn();
  return compute(getOwner(), fn, null);
}
function latest(fn, fallback) {
  const argLength = arguments.length;
  const prevFlags = newFlags;
  const prevNotStale = notStale;
  notStale = false;
  try {
    return fn();
  } catch (err) {
    if (argLength > 1 && err instanceof NotReadyError) return fallback;
    throw err;
  } finally {
    newFlags = prevFlags;
    notStale = prevNotStale;
  }
}
function compute(owner, fn, observer) {
  const prevOwner = setOwner(owner),
    prevObserver = currentObserver,
    prevMask = currentMask,
    prevNotStale = notStale;
  currentObserver = observer;
  currentMask = observer?.D ?? DEFAULT_FLAGS;
  notStale = true;
  try {
    return fn(observer ? observer.e : void 0);
  } finally {
    setOwner(prevOwner);
    currentObserver = prevObserver;
    currentMask = prevMask;
    notStale = prevNotStale;
  }
}

// src/core/effect.ts
var Effect = class extends Computation {
  y;
  z;
  t;
  G = false;
  A;
  o;
  constructor(initialValue, compute2, effect, error, options) {
    super(initialValue, compute2, options);
    this.y = effect;
    this.z = error;
    this.A = initialValue;
    this.o = options?.render ? EFFECT_RENDER : EFFECT_USER;
    if (this.o === EFFECT_RENDER) {
      this.r = (p) =>
        getClock() > this.f.created && !(this.d & ERROR_BIT)
          ? latest(() => compute2(p))
          : compute2(p);
    }
    this.p();
    !options?.defer &&
      (this.o === EFFECT_USER ? this.f.enqueue(this.o, this) : this.K());
  }
  write(value, flags = 0) {
    if (this.a == STATE_DIRTY) {
      const currentFlags = this.d;
      this.d = flags;
      if (
        this.o === EFFECT_RENDER &&
        (flags & LOADING_BIT) !== (currentFlags & LOADING_BIT)
      ) {
        this.f.H?.(this);
      }
    }
    if (value === UNCHANGED) return this.e;
    this.e = value;
    this.G = true;
    return value;
  }
  k(state, skipQueue) {
    if (this.a >= state || skipQueue) return;
    if (this.a === STATE_CLEAN) this.f.enqueue(this.o, this);
    this.a = state;
  }
  F(error) {
    this.t?.();
    if (this.d & LOADING_BIT) {
      this.f.H?.(this);
    }
    this.d = ERROR_BIT;
    if (this.o === EFFECT_USER) {
      try {
        return this.z
          ? (this.t = this.z(error))
          : console.error(new EffectError(this.y, error));
      } catch (e) {
        error = e;
      }
    }
    this.handleError(error);
  }
  q() {
    if (this.a === STATE_DISPOSED) return;
    this.y = void 0;
    this.A = void 0;
    this.z = void 0;
    this.t?.();
    this.t = void 0;
    super.q();
  }
  K() {
    if (this.G && this.a !== STATE_DISPOSED) {
      this.t?.();
      try {
        this.t = this.y(this.e, this.A);
      } catch (e) {
        this.handleError(e);
      } finally {
        this.A = this.e;
        this.G = false;
      }
    }
  }
};
var EagerComputation = class extends Computation {
  constructor(initialValue, compute2, options) {
    super(initialValue, compute2, options);
    !options?.defer && this.p();
  }
  k(state, skipQueue) {
    if (this.a >= state && !this.x) return;
    if (this.a === STATE_CLEAN && !skipQueue) this.f.enqueue(EFFECT_PURE, this);
    super.k(state, skipQueue);
  }
};

// src/signals.ts
function createSignal(first, second, third) {
  if (typeof first === "function") {
    const memo = createMemo((p) => {
      const node2 = new Computation(
        first(p ? untrack(p[0]) : second),
        null,
        third,
      );
      return [node2.read.bind(node2), node2.write.bind(node2)];
    });
    return [() => memo()[0](), (value) => memo()[1](value)];
  }
  const node = new Computation(first, null, second);
  return [node.read.bind(node), node.write.bind(node)];
}
function createMemo(compute2, value, options) {
  let node = new Computation(value, compute2, options);
  let resolvedValue;
  return () => {
    if (node) {
      resolvedValue = node.wait();
      if (!node.b?.length && node.h?.m !== node) {
        node.dispose();
        node = void 0;
      } else if (!node.m && !node.c?.length) {
        node.dispose();
        node.a = STATE_DIRTY;
      }
    }
    return resolvedValue;
  };
}
function createEffect(compute2, effect, error, value, options) {
  void new Effect(value, compute2, effect, error, options);
}
function createRoot(init) {
  const owner = new Owner();
  return compute(
    owner,
    !init.length ? init : () => init(() => owner.dispose()),
    null,
  );
}
function runWithOwner(owner, run) {
  return compute(owner, run, null);
}
function resolve(fn) {
  return new Promise((res, rej) => {
    createRoot((dispose) => {
      new EagerComputation(void 0, () => {
        try {
          res(fn());
        } catch (err) {
          if (err instanceof NotReadyError) throw err;
          rej(err);
        }
        dispose();
      });
    });
  });
}

export {
  Owner,
  createEffect,
  createMemo,
  createRoot,
  createSignal,
  flushSync,
  getOwner,
  onCleanup,
  resolve,
  runWithOwner,
  untrack,
};
