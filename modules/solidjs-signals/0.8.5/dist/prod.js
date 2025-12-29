class NotReadyError extends Error {
  cause;
  constructor(e) {
    super();
    this.cause = e;
  }
}
class NoOwnerError extends Error {
  constructor() {
    super("");
  }
}
class ContextNotFoundError extends Error {
  constructor() {
    super("");
  }
}
const REACTIVE_NONE = 0;
const REACTIVE_CHECK = 1 << 0;
const REACTIVE_DIRTY = 1 << 1;
const REACTIVE_RECOMPUTING_DEPS = 1 << 2;
const REACTIVE_IN_HEAP = 1 << 3;
const REACTIVE_IN_HEAP_HEIGHT = 1 << 4;
const REACTIVE_ZOMBIE = 1 << 5;
const REACTIVE_DISPOSED = 1 << 6;
const STATUS_NONE = 0;
const STATUS_PENDING = 1 << 0;
const STATUS_ERROR = 1 << 1;
const STATUS_UNINITIALIZED = 1 << 2;
const EFFECT_RENDER = 1;
const EFFECT_USER = 2;
const NOT_PENDING = {};
const SUPPORTS_PROXY = typeof Proxy === "function";
const defaultContext = {};
function actualInsertIntoHeap(e, t) {
  const n = (e.i?.t ? e.i.u?.o : e.i?.o) ?? -1;
  if (n >= e.o) e.o = n + 1;
  const i = e.o;
  const r = t.l[i];
  if (r === undefined) t.l[i] = e;
  else {
    const t = r.T;
    t.R = e;
    e.T = t;
    r.T = e;
  }
  if (i > t.h) t.h = i;
}
function insertIntoHeap(e, t) {
  let n = e._;
  if (n & (REACTIVE_IN_HEAP | REACTIVE_RECOMPUTING_DEPS)) return;
  if (n & REACTIVE_CHECK) {
    e._ = (n & -4) | REACTIVE_DIRTY | REACTIVE_IN_HEAP;
  } else e._ = n | REACTIVE_IN_HEAP;
  if (!(n & REACTIVE_IN_HEAP_HEIGHT)) actualInsertIntoHeap(e, t);
}
function insertIntoHeapHeight(e, t) {
  let n = e._;
  if (n & (REACTIVE_IN_HEAP | REACTIVE_RECOMPUTING_DEPS | REACTIVE_IN_HEAP_HEIGHT)) return;
  e._ = n | REACTIVE_IN_HEAP_HEIGHT;
  actualInsertIntoHeap(e, t);
}
function deleteFromHeap(e, t) {
  const n = e._;
  if (!(n & (REACTIVE_IN_HEAP | REACTIVE_IN_HEAP_HEIGHT))) return;
  e._ = n & -25;
  const i = e.o;
  if (e.T === e) t.l[i] = undefined;
  else {
    const n = e.R;
    const r = t.l[i];
    const s = n ?? r;
    if (e === r) t.l[i] = n;
    else e.T.R = n;
    s.T = e.T;
  }
  e.T = e;
  e.R = undefined;
}
function markHeap(e) {
  if (e.S) return;
  e.S = true;
  for (let t = 0; t <= e.h; t++) {
    for (let n = e.l[t]; n !== undefined; n = n.R) {
      if (n._ & REACTIVE_IN_HEAP) markNode(n);
    }
  }
}
function markNode(e, t = REACTIVE_DIRTY) {
  const n = e._;
  if ((n & (REACTIVE_CHECK | REACTIVE_DIRTY)) >= t) return;
  e._ = (n & -4) | t;
  for (let t = e.O; t !== null; t = t.p) {
    markNode(t.A, REACTIVE_CHECK);
  }
  if (e.N !== null) {
    for (let t = e.N; t !== null; t = t.I) {
      for (let e = t.O; e !== null; e = e.p) {
        markNode(e.A, REACTIVE_CHECK);
      }
    }
  }
}
function runHeap(e, t) {
  e.S = false;
  for (e.C = 0; e.C <= e.h; e.C++) {
    let n = e.l[e.C];
    while (n !== undefined) {
      if (n._ & REACTIVE_IN_HEAP) t(n);
      else adjustHeight(n, e);
      n = e.l[e.C];
    }
  }
  e.h = 0;
}
function adjustHeight(e, t) {
  deleteFromHeap(e, t);
  let n = e.o;
  for (let t = e.D; t; t = t.P) {
    const e = t.V;
    const i = e.U || e;
    if (i.m && i.o >= n) n = i.o + 1;
  }
  if (e.o !== n) {
    e.o = n;
    for (let n = e.O; n !== null; n = n.p) {
      insertIntoHeapHeight(n.A, t);
    }
  }
}
const transitions = new Set();
const dirtyQueue = { l: new Array(2e3).fill(undefined), S: false, C: 0, h: 0 };
const zombieQueue = { l: new Array(2e3).fill(undefined), S: false, C: 0, h: 0 };
let clock = 0;
let activeTransition = null;
let scheduled = false;
function schedule() {
  if (scheduled) return;
  scheduled = true;
  if (!globalQueue.k) queueMicrotask(flush);
}
class Queue {
  i = null;
  G = [[], []];
  H = [];
  created = clock;
  addChild(e) {
    this.H.push(e);
    e.i = this;
  }
  removeChild(e) {
    const t = this.H.indexOf(e);
    if (t >= 0) {
      this.H.splice(t, 1);
      e.i = null;
    }
  }
  notify(e, t, n) {
    if (this.i) return this.i.notify(e, t, n);
    return false;
  }
  run(e) {
    if (this.G[e - 1].length) {
      const t = this.G[e - 1];
      this.G[e - 1] = [];
      runQueue(t, e);
    }
    for (let t = 0; t < this.H.length; t++) {
      this.H[t].run(e);
    }
  }
  enqueue(e, t) {
    if (e) this.G[e - 1].push(t);
    schedule();
  }
  stashQueues(e) {
    e.G[0].push(...this.G[0]);
    e.G[1].push(...this.G[1]);
    this.G = [[], []];
    for (let t = 0; t < this.H.length; t++) {
      let n = this.H[t];
      let i = e.H[t];
      if (!i) {
        i = { G: [[], []], H: [] };
        e.H[t] = i;
      }
      n.stashQueues(i);
    }
  }
  restoreQueues(e) {
    this.G[0].push(...e.G[0]);
    this.G[1].push(...e.G[1]);
    for (let t = 0; t < e.H.length; t++) {
      const n = e.H[t];
      let i = this.H[t];
      if (i) i.restoreQueues(n);
    }
  }
}
class GlobalQueue extends Queue {
  k = false;
  $ = [];
  static L;
  static F;
  flush() {
    if (this.k) return;
    this.k = true;
    try {
      runHeap(dirtyQueue, GlobalQueue.L);
      if (activeTransition) {
        if (!transitionComplete(activeTransition)) {
          runHeap(zombieQueue, GlobalQueue.L);
          this.$ = [];
          this.stashQueues(activeTransition.queueStash);
          clock++;
          scheduled = false;
          runPending(activeTransition.pendingNodes, true);
          activeTransition = null;
          return;
        }
        this.$.push(...activeTransition.pendingNodes);
        this.restoreQueues(activeTransition.queueStash);
        transitions.delete(activeTransition);
        activeTransition = null;
        if (runPending(this.$, false)) runHeap(dirtyQueue, GlobalQueue.L);
      } else if (transitions.size) runHeap(zombieQueue, GlobalQueue.L);
      for (let e = 0; e < this.$.length; e++) {
        const t = this.$[e];
        if (t.W !== NOT_PENDING) {
          t.j = t.W;
          t.W = NOT_PENDING;
          if (t.K) t.M = true;
        }
        if (t.m) GlobalQueue.F(t, false, true);
      }
      this.$.length = 0;
      clock++;
      scheduled = false;
      this.run(EFFECT_RENDER);
      this.run(EFFECT_USER);
    } finally {
      this.k = false;
    }
  }
  notify(e, t, n) {
    if (t & STATUS_PENDING) {
      if (n & STATUS_PENDING) {
        if (activeTransition && !activeTransition.asyncNodes.includes(e.Y.cause)) {
          activeTransition.asyncNodes.push(e.Y.cause);
          schedule();
        }
      }
      return true;
    }
    return false;
  }
  initTransition(e) {
    if (activeTransition && activeTransition.time === clock) return;
    if (!activeTransition) {
      activeTransition = e.B ?? {
        time: clock,
        pendingNodes: [],
        asyncNodes: [],
        queueStash: { G: [[], []], H: [] }
      };
    }
    transitions.add(activeTransition);
    activeTransition.time = clock;
    for (let e = 0; e < this.$.length; e++) {
      const t = this.$[e];
      t.B = activeTransition;
      activeTransition.pendingNodes.push(t);
    }
    this.$ = activeTransition.pendingNodes;
  }
}
function runPending(e, t) {
  let n = false;
  const i = e.slice();
  for (let e = 0; e < i.length; e++) {
    const r = i[e];
    r.B = activeTransition;
    if (r.X) {
      r.X.q(t);
      n = true;
    }
    if (r.Z && r.Z.W !== NOT_PENDING) {
      r.Z.q(r.Z.W);
      r.Z.W = NOT_PENDING;
      n = true;
    }
  }
  return n;
}
const globalQueue = new GlobalQueue();
function flush() {
  while (scheduled) {
    globalQueue.flush();
  }
}
function runQueue(e, t) {
  for (let n = 0; n < e.length; n++) e[n](t);
}
function transitionComplete(e) {
  let t = true;
  for (let n = 0; n < e.asyncNodes.length; n++) {
    if (e.asyncNodes[n].J & STATUS_PENDING) {
      t = false;
      break;
    }
  }
  return t;
}
function runInTransition(e, t) {
  const n = activeTransition;
  activeTransition = e.B;
  t(e);
  activeTransition = n;
}
GlobalQueue.L = recompute;
GlobalQueue.F = disposeChildren;
let tracking = false;
let stale = false;
let pendingValueCheck = false;
let pendingCheck = null;
let context = null;
function notifySubs(e) {
  for (let t = e.O; t !== null; t = t.p) {
    const e = t.A._ & REACTIVE_ZOMBIE ? zombieQueue : dirtyQueue;
    if (e.C > t.A.o) e.C = t.A.o;
    insertIntoHeap(t.A, e);
  }
}
function recompute(e, t = false) {
  deleteFromHeap(e, e._ & REACTIVE_ZOMBIE ? zombieQueue : dirtyQueue);
  if (e.W !== NOT_PENDING || e.ee || e.te) disposeChildren(e);
  else {
    markDisposal(e);
    globalQueue.$.push(e);
    e.te = e.ne;
    e.ee = e.ie;
    e.ne = null;
    e.ie = null;
  }
  const n = context;
  context = e;
  e.re = null;
  e._ = REACTIVE_RECOMPUTING_DEPS;
  e.se = clock;
  let i = e.W === NOT_PENDING ? e.j : e.W;
  let r = e.o;
  let s = e.J;
  let o = e.Y;
  let u = tracking;
  setStatusFlags(e, STATUS_NONE | (s & STATUS_UNINITIALIZED));
  tracking = true;
  try {
    i = e.m(i);
    e.J &= ~STATUS_UNINITIALIZED;
  } catch (t) {
    if (t instanceof NotReadyError) {
      if (t.cause !== e) link(t.cause, e);
      setStatusFlags(e, (s & ~STATUS_ERROR) | STATUS_PENDING, t);
    } else setStatusFlags(e, STATUS_ERROR, t);
  } finally {
    tracking = u;
  }
  e._ = REACTIVE_NONE;
  context = n;
  const l = e.re;
  let c = l !== null ? l.P : e.D;
  if (c !== null) {
    do {
      c = unlinkSubs(c);
    } while (c !== null);
    if (l !== null) l.P = null;
    else e.D = null;
  }
  const a = e.K && e.B != activeTransition;
  const f = !e.oe || !e.oe(e.W === NOT_PENDING || e.ue || a ? e.j : e.W, i);
  const E = e.J !== s || e.Y !== o;
  e.le?.(E, s);
  if (f || E) {
    if (f) {
      if (t || e.ue || a) e.j = i;
      else {
        if (e.W === NOT_PENDING) globalQueue.$.push(e);
        e.W = i;
      }
      if (e.Z) e.Z.q(i);
    }
    for (let t = e.O; t !== null; t = t.p) {
      const n = t.A._ & REACTIVE_ZOMBIE ? zombieQueue : dirtyQueue;
      if (t.A.o < e.o && n.C > t.A.o) n.C = t.A.o;
      insertIntoHeap(t.A, n);
    }
  } else if (e.o != r) {
    for (let t = e.O; t !== null; t = t.p) {
      insertIntoHeapHeight(t.A, t.A._ & REACTIVE_ZOMBIE ? zombieQueue : dirtyQueue);
    }
  }
  if (e.B && a) runInTransition(e, recompute);
}
function updateIfNecessary(e) {
  if (e._ & REACTIVE_CHECK) {
    for (let t = e.D; t; t = t.P) {
      const n = t.V;
      const i = n.U || n;
      if (i.m) {
        updateIfNecessary(i);
      }
      if (e._ & REACTIVE_DIRTY) {
        break;
      }
    }
  }
  if (e._ & REACTIVE_DIRTY) {
    recompute(e);
  }
  e._ = REACTIVE_NONE;
}
function unlinkSubs(e) {
  const t = e.V;
  const n = e.P;
  const i = e.p;
  const r = e.ce;
  if (i !== null) i.ce = r;
  else t.ae = r;
  if (r !== null) r.p = i;
  else {
    t.O = i;
    if (i === null) {
      t.fe?.();
      t.m && unobserved(t);
    }
  }
  return n;
}
function unobserved(e) {
  deleteFromHeap(e, e._ & REACTIVE_ZOMBIE ? zombieQueue : dirtyQueue);
  let t = e.D;
  while (t !== null) {
    t = unlinkSubs(t);
  }
  e.D = null;
  runDisposal(e);
}
function link(e, t) {
  const n = t.re;
  if (n !== null && n.V === e) return;
  let i = null;
  const r = t._ & REACTIVE_RECOMPUTING_DEPS;
  if (r) {
    i = n !== null ? n.P : t.D;
    if (i !== null && i.V === e) {
      t.re = i;
      return;
    }
  }
  const s = e.ae;
  if (s !== null && s.A === t && (!r || isValidLink(s, t))) return;
  const o = (t.re = e.ae = { V: e, A: t, P: i, ce: s, p: null });
  if (n !== null) n.P = o;
  else t.D = o;
  if (s !== null) s.p = o;
  else e.O = o;
}
function isValidLink(e, t) {
  const n = t.re;
  if (n !== null) {
    let i = t.D;
    do {
      if (i === e) return true;
      if (i === n) break;
      i = i.P;
    } while (i !== null);
  }
  return false;
}
function setStatusFlags(e, t, n = null) {
  e.J = t;
  e.Y = n;
}
function markDisposal(e) {
  let t = e.ie;
  while (t) {
    t._ |= REACTIVE_ZOMBIE;
    if (t._ & REACTIVE_IN_HEAP) {
      deleteFromHeap(t, dirtyQueue);
      insertIntoHeap(t, zombieQueue);
    }
    markDisposal(t);
    t = t.Ee;
  }
}
function dispose(e) {
  let t = e.D || null;
  do {
    t = unlinkSubs(t);
  } while (t !== null);
  e.D = null;
  e.re = null;
  disposeChildren(e, true);
}
function disposeChildren(e, t = false, n) {
  if (e._ & REACTIVE_DISPOSED) return;
  if (t) e._ = REACTIVE_DISPOSED;
  let i = n ? e.ee : e.ie;
  while (i) {
    const e = i.Ee;
    if (i.D) {
      const e = i;
      deleteFromHeap(e, e._ & REACTIVE_ZOMBIE ? zombieQueue : dirtyQueue);
      let t = e.D;
      do {
        t = unlinkSubs(t);
      } while (t !== null);
      e.D = null;
      e.re = null;
    }
    disposeChildren(i, true);
    i = e;
  }
  if (n) {
    e.ee = null;
  } else {
    e.ie = null;
    e.Ee = null;
  }
  runDisposal(e, n);
}
function runDisposal(e, t) {
  let n = t ? e.te : e.ne;
  if (!n) return;
  if (Array.isArray(n)) {
    for (let e = 0; e < n.length; e++) {
      const t = n[e];
      t.call(t);
    }
  } else {
    n.call(n);
  }
  t ? (e.te = null) : (e.ne = null);
}
function getNextChildId(e) {
  if (e.id != null) return formatId(e.id, e.de++);
  throw new Error("Cannot get child id from owner without an id");
}
function formatId(e, t) {
  const n = t.toString(36),
    i = n.length - 1;
  return e + (i ? String.fromCharCode(64 + i) : "") + n;
}
function computed(e, t, n) {
  const i = {
    id: n?.id ?? (context?.id != null ? getNextChildId(context) : undefined),
    oe: n?.equals != null ? n.equals : isEqual,
    Te: !!n?.pureWrite,
    fe: n?.unobserved,
    ne: null,
    Re: context?.Re ?? globalQueue,
    he: context?.he ?? defaultContext,
    de: 0,
    m: e,
    j: t,
    o: 0,
    N: null,
    R: undefined,
    T: null,
    D: null,
    re: null,
    O: null,
    ae: null,
    i: context,
    Ee: null,
    ie: null,
    _: REACTIVE_NONE,
    J: STATUS_UNINITIALIZED,
    se: clock,
    W: NOT_PENDING,
    te: null,
    ee: null,
    B: null
  };
  if (n?._e) Object.assign(i, n._e);
  i.T = i;
  const r = context?.t ? context.u : context;
  if (context) {
    const e = context.ie;
    if (e === null) {
      context.ie = i;
    } else {
      i.Ee = e;
      context.ie = i;
    }
  }
  if (r) i.o = r.o + 1;
  recompute(i, true);
  return i;
}
function asyncComputed(e, t, n) {
  let i = undefined;
  let r = false;
  const fn = t => {
    const n = e(t, r);
    r = false;
    i = n;
    const o = n instanceof Promise;
    const u = n[Symbol.asyncIterator];
    if (!o && !u) {
      return n;
    }
    if (o) {
      n.then(e => {
        if (i !== n) return;
        globalQueue.initTransition(s);
        setSignal(s, () => e);
        flush();
      }).catch(e => {
        if (i !== n) return;
        globalQueue.initTransition(s);
        setStatusFlags(s, STATUS_ERROR, e);
        s.se = clock;
        notifySubs(s);
        schedule();
        flush();
      });
    } else {
      (async () => {
        try {
          for await (let e of n) {
            if (i !== n) return;
            globalQueue.initTransition(s);
            setSignal(s, () => e);
            flush();
          }
        } catch (e) {
          if (i !== n) return;
          globalQueue.initTransition(s);
          setStatusFlags(s, STATUS_ERROR, e);
          s.se = clock;
          notifySubs(s);
          schedule();
          flush();
        }
      })();
    }
    globalQueue.initTransition(context);
    throw new NotReadyError(context);
  };
  const s = computed(fn, t, n);
  s.Se = () => {
    r = true;
    recompute(s);
    schedule();
    flush();
  };
  return s;
}
function signal(e, t, n = null) {
  const i = {
    id: t?.id ?? (context?.id != null ? getNextChildId(context) : undefined),
    oe: t?.equals != null ? t.equals : isEqual,
    Te: !!t?.pureWrite,
    fe: t?.unobserved,
    j: e,
    O: null,
    ae: null,
    J: STATUS_NONE,
    se: clock,
    U: n,
    I: n?.N || null,
    W: NOT_PENDING
  };
  n && (n.N = i);
  return i;
}
function isEqual(e, t) {
  return e === t;
}
function untrack(e) {
  if (!tracking) return e();
  tracking = false;
  try {
    return e();
  } finally {
    tracking = true;
  }
}
function read(e) {
  let t = context;
  if (t?.t) t = t.u;
  if (t && tracking && !pendingCheck && !pendingValueCheck) {
    link(e, t);
    const n = e.U || e;
    if (n.m) {
      const i = e._ & REACTIVE_ZOMBIE;
      if (n.o >= (i ? zombieQueue.C : dirtyQueue.C)) {
        markNode(t);
        markHeap(i ? zombieQueue : dirtyQueue);
        updateIfNecessary(n);
      }
      const r = n.o;
      if (r >= t.o && e.i !== t) {
        t.o = r + 1;
      }
    }
  }
  if (pendingCheck) {
    if (!e.X) {
      e.X = signal(false);
      e.X.ue = true;
      e.X.q = t => setSignal(e.X, t);
    }
    const t = pendingCheck;
    pendingCheck = null;
    t.j = read(e.X) || t.j;
    pendingCheck = t;
  }
  if (pendingValueCheck) {
    if (!e.Z) {
      e.Z = signal(e.j);
      e.Z.ue = true;
      e.Z.q = t => setSignal(e.Z, t);
    }
    pendingValueCheck = false;
    try {
      return read(e.Z);
    } finally {
      pendingValueCheck = true;
    }
  }
  if (e.J & STATUS_PENDING && !pendingCheck) {
    if ((t && !stale) || e.J & STATUS_UNINITIALIZED) throw e.Y;
    else if (t && stale) {
      setStatusFlags(t, t.J | 1, e.Y);
    }
  }
  if (e.J & STATUS_ERROR) {
    if (e.se < clock) {
      recompute(e, true);
      return read(e);
    } else {
      throw e.Y;
    }
  }
  return !t ||
    e.ue ||
    e.W === NOT_PENDING ||
    (stale && !pendingCheck && e.B && activeTransition !== e.B)
    ? e.j
    : e.W;
}
function setSignal(e, t) {
  if (typeof t === "function") {
    t = t(e.W === NOT_PENDING ? e.j : e.W);
  }
  const n = !e.oe || !e.oe(e.W === NOT_PENDING || e.ue ? e.j : e.W, t);
  if (!n && !e.J) return t;
  if (n) {
    if (e.ue) e.j = t;
    else {
      if (e.W === NOT_PENDING) globalQueue.$.push(e);
      e.W = t;
    }
    if (e.Z) e.Z.W = t;
  }
  setStatusFlags(e, STATUS_NONE);
  e.se = clock;
  notifySubs(e);
  schedule();
  return t;
}
function getObserver() {
  return tracking ? context : null;
}
function getOwner() {
  return context;
}
function onCleanup(e) {
  if (!context) return e;
  const t = context;
  if (!t.ne) {
    t.ne = e;
  } else if (Array.isArray(t.ne)) {
    t.ne.push(e);
  } else {
    t.ne = [t.ne, e];
  }
  return e;
}
function createOwner(e) {
  const t = context;
  const n = {
    t: true,
    u: t?.t ? t.u : t,
    ie: null,
    Ee: null,
    ne: null,
    id: e?.id ?? (t?.id != null ? getNextChildId(t) : undefined),
    Re: t?.Re ?? globalQueue,
    he: t?.he || defaultContext,
    de: 0,
    te: null,
    ee: null,
    i: t,
    dispose(e = true) {
      disposeChildren(n, e);
    }
  };
  if (t) {
    const e = t.ie;
    if (e === null) {
      t.ie = n;
    } else {
      n.Ee = e;
      t.ie = n;
    }
  }
  return n;
}
function createRoot(e, t) {
  const n = createOwner(t);
  return runWithOwner(n, () => e(n.dispose));
}
function runWithOwner(e, t) {
  const n = context;
  context = e;
  try {
    return t();
  } finally {
    context = n;
  }
}
function staleValues(e, t = true) {
  const n = stale;
  stale = t;
  try {
    return e();
  } finally {
    stale = n;
  }
}
function pending(e) {
  const t = pendingValueCheck;
  pendingValueCheck = true;
  try {
    return staleValues(e, false);
  } finally {
    pendingValueCheck = t;
  }
}
function isPending(e) {
  const t = pendingCheck;
  pendingCheck = { j: false };
  try {
    staleValues(e);
    return pendingCheck.j;
  } catch (e) {
    if (!(e instanceof NotReadyError)) return false;
    throw e;
  } finally {
    pendingCheck = t;
  }
}
function createContext(e, t) {
  return { id: Symbol(t), defaultValue: e };
}
function getContext(e, t = getOwner()) {
  if (!t) {
    throw new NoOwnerError();
  }
  const n = hasContext(e, t) ? t.he[e.id] : e.defaultValue;
  if (isUndefined(n)) {
    throw new ContextNotFoundError();
  }
  return n;
}
function setContext(e, t, n = getOwner()) {
  if (!n) {
    throw new NoOwnerError();
  }
  n.he = { ...n.he, [e.id]: isUndefined(t) ? e.defaultValue : t };
}
function hasContext(e, t) {
  return !isUndefined(t?.he[e.id]);
}
function isUndefined(e) {
  return typeof e === "undefined";
}
function effect(e, t, n, i, r) {
  let s = false;
  const o = computed(e, i, {
    ...r,
    _e: {
      M: true,
      Oe: i,
      pe: t,
      Ae: n,
      Ne: undefined,
      K: r?.render ? EFFECT_RENDER : EFFECT_USER,
      le(e, t) {
        if (s) {
          const n = this.J && this.J === t && e;
          this.M = !(this.J & STATUS_ERROR) && !(this.J & STATUS_PENDING & ~t) && !n;
          if (this.M) this.Re.enqueue(this.K, runEffect.bind(this));
        }
        if (this.J & STATUS_ERROR) {
          let e = this.Y;
          this.Re.notify(this, STATUS_PENDING, 0);
          if (this.K === EFFECT_USER) {
            try {
              return this.Ae
                ? this.Ae(e, () => {
                    this.Ne?.();
                    this.Ne = undefined;
                  })
                : console.error(e);
            } catch (t) {
              e = t;
            }
          }
          if (!this.Re.notify(this, STATUS_ERROR, STATUS_ERROR)) throw e;
        } else if (this.K === EFFECT_RENDER) {
          this.Re.notify(this, STATUS_PENDING | STATUS_ERROR, this.J);
        }
      }
    }
  });
  s = true;
  if (o.K === EFFECT_RENDER) o.m = t => staleValues(() => e(t));
  !r?.defer &&
    !(o.J & (STATUS_ERROR | STATUS_PENDING)) &&
    (o.K === EFFECT_USER ? o.Re.enqueue(o.K, runEffect.bind(o)) : runEffect.call(o));
  onCleanup(() => o.Ne?.());
}
function runEffect() {
  if (!this.M || this._ & REACTIVE_DISPOSED) return;
  this.Ne?.();
  this.Ne = undefined;
  try {
    this.Ne = this.pe(this.j, this.Oe);
  } catch (e) {
    if (!this.Re.notify(this, STATUS_ERROR, STATUS_ERROR)) throw e;
  } finally {
    this.Oe = this.j;
    this.M = false;
  }
}
function createSignal(e, t, n) {
  if (typeof e === "function") {
    const i = computed(e, t, n);
    return [read.bind(null, i), setSignal.bind(null, i)];
  }
  const i = getOwner();
  const r = i?.id != null;
  const s = signal(e, r ? { id: getNextChildId(i), ...t } : t);
  return [read.bind(null, s), setSignal.bind(null, s)];
}
function createMemo(e, t, n) {
  let i = computed(e, t, n);
  return read.bind(null, i);
}
function createAsync(e, t, n) {
  const i = asyncComputed(e, t, n);
  const r = read.bind(null, i);
  r.refresh = i.Se;
  return r;
}
function createEffect(e, t, n, i) {
  void effect(e, t.effect || t, t.error, n, i);
}
function createRenderEffect(e, t, n, i) {
  void effect(e, t, undefined, n, { render: true, ...i });
}
function createTrackedEffect(e, t) {}
function createReaction(e, t) {
  let n = undefined;
  onCleanup(() => n?.());
  const i = getOwner();
  return r => {
    runWithOwner(i, () => {
      effect(
        () => (r(), getOwner()),
        t => {
          n?.();
          n = (e.effect || e)?.();
          dispose(t);
        },
        e.error,
        undefined,
        { defer: true, ...(false ? { ...t, name: t?.name ?? "effect" } : t) }
      );
    });
  };
}
function resolve(e) {
  return new Promise((t, n) => {
    createRoot(i => {
      computed(() => {
        try {
          t(e());
        } catch (e) {
          if (e instanceof NotReadyError) throw e;
          n(e);
        }
        i();
      });
    });
  });
}
function createOptimistic(e, t, n) {
  return {};
}
function onSettled(e) {
  let t;
  const n = getOwner();
  if (n) onCleanup(() => t?.());
  globalQueue.enqueue(EFFECT_USER, () => {
    t = e();
    !n && t?.();
  });
}
function unwrap(e) {
  return e?.[$TARGET]?.[STORE_NODE] ?? e;
}
function getOverrideValue(e, t, n, i) {
  return n && i in n ? read(n[i]) : t && i in t ? t[i] : e[i];
}
function getAllKeys(e, t, n) {
  const i = getKeys(e, t);
  const r = Object.keys(n);
  return Array.from(new Set([...i, ...r]));
}
function applyState(e, t, n, i) {
  const r = t?.[$TARGET];
  if (!r) return;
  const s = r[STORE_VALUE];
  const o = r[STORE_OVERRIDE];
  let u = r[STORE_NODE];
  if (e === s && !o) return;
  (r[STORE_LOOKUP] || storeLookup).set(e, r[$PROXY]);
  r[STORE_VALUE] = e;
  r[STORE_OVERRIDE] = undefined;
  if (Array.isArray(s)) {
    let t = false;
    const l = getOverrideValue(s, o, u, "length");
    if (e.length && l && e[0] && n(e[0]) != null) {
      let c, a, f, E, d, T, R, h;
      for (
        f = 0, E = Math.min(l, e.length);
        f < E && ((T = getOverrideValue(s, o, u, f)) === e[f] || (T && e[f] && n(T) === n(e[f])));
        f++
      ) {
        applyState(e[f], wrap(T, r), n, i);
      }
      const _ = new Array(e.length),
        S = new Map();
      for (
        E = l - 1, d = e.length - 1;
        E >= f &&
        d >= f &&
        ((T = getOverrideValue(s, o, u, E)) === e[d] || (T && e[d] && n(T) === n(e[d])));
        E--, d--
      ) {
        _[d] = T;
      }
      if (f > d || f > E) {
        for (a = f; a <= d; a++) {
          t = true;
          r[STORE_NODE][a] && setSignal(r[STORE_NODE][a], wrap(e[a], r));
        }
        for (; a < e.length; a++) {
          t = true;
          const s = wrap(_[a], r);
          r[STORE_NODE][a] && setSignal(r[STORE_NODE][a], s);
          applyState(e[a], s, n, i);
        }
        t && r[STORE_NODE][$TRACK] && setSignal(r[STORE_NODE][$TRACK], void 0);
        l !== e.length && r[STORE_NODE].length && setSignal(r[STORE_NODE].length, e.length);
        return;
      }
      R = new Array(d + 1);
      for (a = d; a >= f; a--) {
        T = e[a];
        h = T ? n(T) : T;
        c = S.get(h);
        R[a] = c === undefined ? -1 : c;
        S.set(h, a);
      }
      for (c = f; c <= E; c++) {
        T = getOverrideValue(s, o, u, c);
        h = T ? n(T) : T;
        a = S.get(h);
        if (a !== undefined && a !== -1) {
          _[a] = T;
          a = R[a];
          S.set(h, a);
        }
      }
      for (a = f; a < e.length; a++) {
        if (a in _) {
          const t = wrap(_[a], r);
          r[STORE_NODE][a] && setSignal(r[STORE_NODE][a], t);
          applyState(e[a], t, n, i);
        } else r[STORE_NODE][a] && setSignal(r[STORE_NODE][a], wrap(e[a], r));
      }
      if (f < e.length) t = true;
    } else if (l && e.length) {
      for (let t = 0, l = e.length; t < l; t++) {
        const l = getOverrideValue(s, o, u, t);
        isWrappable(l) && applyState(e[t], wrap(l, r), n, i);
      }
    }
    if (l !== e.length) {
      t = true;
      r[STORE_NODE].length && setSignal(r[STORE_NODE].length, e.length);
    }
    t && r[STORE_NODE][$TRACK] && setSignal(r[STORE_NODE][$TRACK], void 0);
    return;
  }
  if (u) {
    const t = u[$TRACK];
    const l = t || i ? getAllKeys(s, o, e) : Object.keys(u);
    for (let c = 0, a = l.length; c < a; c++) {
      const a = l[c];
      const f = u[a];
      const E = unwrap(getOverrideValue(s, o, u, a));
      let d = unwrap(e[a]);
      if (E === d) continue;
      if (!E || !isWrappable(E) || (n(E) != null && n(E) !== n(d))) {
        t && setSignal(t, void 0);
        f && setSignal(f, isWrappable(d) ? wrap(d, r) : d);
      } else applyState(d, wrap(E, r), n, i);
    }
  }
  if ((u = r[STORE_HAS])) {
    const t = Object.keys(u);
    for (let n = 0, i = t.length; n < i; n++) {
      const i = t[n];
      setSignal(u[i], i in e);
    }
  }
}
function reconcile(e, t, n = false) {
  return i => {
    if (i == null) throw new Error("Cannot reconcile null or undefined state");
    const r = typeof t === "string" ? e => e[t] : t;
    const s = r(i);
    if (s !== undefined && r(e) !== r(i))
      throw new Error("Cannot reconcile states with different identity");
    applyState(e, i, r, n);
  };
}
function createProjectionInternal(e, t = {}, n) {
  let i;
  const r = new WeakMap();
  const wrapProjection = e => {
    if (r.has(e)) return r.get(e);
    if (e[$TARGET]?.[STORE_WRAP] === wrapProjection) return e;
    const t = createStoreProxy(e, storeTraps, {
      [STORE_WRAP]: wrapProjection,
      [STORE_LOOKUP]: r,
      [STORE_FIREWALL]() {
        return i;
      }
    });
    r.set(e, t);
    return t;
  };
  const s = wrapProjection(t);
  i = computed(() => {
    storeSetter(s, t => {
      const i = e(t);
      if (i !== t && i !== undefined) {
        reconcile(i, n?.key || "id", n?.all)(t);
      }
    });
  });
  return { store: s, node: i };
}
function createProjection(e, t = {}, n) {
  return createProjectionInternal(e, t, n).store;
}
const $TRACK = Symbol(0),
  $DEEP = Symbol(0),
  $TARGET = Symbol(0),
  $PROXY = Symbol(0),
  $DELETED = Symbol(0);
const PARENTS = new WeakMap();
const STORE_VALUE = "v",
  STORE_OVERRIDE = "o",
  STORE_NODE = "n",
  STORE_HAS = "h",
  STORE_WRAP = "w",
  STORE_LOOKUP = "l",
  STORE_FIREWALL = "f";
function createStoreProxy(e, t = storeTraps, n) {
  let i;
  if (Array.isArray(e)) {
    i = [];
    i.v = e;
  } else i = { v: e };
  n && Object.assign(i, n);
  return (i[$PROXY] = new Proxy(i, t));
}
const storeLookup = new WeakMap();
function wrap(e, t) {
  if (t?.[STORE_WRAP]) return t[STORE_WRAP](e, t);
  let n = e[$PROXY] || storeLookup.get(e);
  if (!n) storeLookup.set(e, (n = createStoreProxy(e)));
  return n;
}
function isWrappable(e) {
  return e != null && typeof e === "object" && !Object.isFrozen(e);
}
function getNodes(e, t) {
  let n = e[t];
  if (!n) e[t] = n = Object.create(null);
  return n;
}
function getNode(e, t, n, i, r = isEqual) {
  if (e[t]) return e[t];
  return (e[t] = signal(
    n,
    {
      equals: r,
      unobserved() {
        delete e[t];
      }
    },
    i
  ));
}
function trackSelf(e, t = $TRACK) {
  getObserver() &&
    read(getNode(getNodes(e, STORE_NODE), t, undefined, e[STORE_FIREWALL]?.(), false));
}
function getKeys(e, t, n = true) {
  const i = untrack(() => (n ? Object.keys(e) : Reflect.ownKeys(e)));
  if (!t) return i;
  const r = new Set(i);
  const s = Reflect.ownKeys(t);
  for (const e of s) {
    if (t[e] !== $DELETED) r.add(e);
    else r.delete(e);
  }
  return Array.from(r);
}
function getPropertyDescriptor(e, t, n) {
  let i = e;
  if (t && n in t) {
    if (i[n] === $DELETED) return void 0;
    if (!(n in i)) i = t;
  }
  return Reflect.getOwnPropertyDescriptor(i, n);
}
let Writing = null;
const storeTraps = {
  get(e, t, n) {
    if (t === $TARGET) return e;
    if (t === $PROXY) return n;
    if (t === $TRACK || t === $DEEP) {
      trackSelf(e, t);
      return n;
    }
    const i = getNodes(e, STORE_NODE);
    const r = i[t];
    const s = e[STORE_OVERRIDE] && t in e[STORE_OVERRIDE];
    const o = !!e[STORE_VALUE][$TARGET];
    const u = s ? e[STORE_OVERRIDE] : e[STORE_VALUE];
    if (!r) {
      const e = Object.getOwnPropertyDescriptor(u, t);
      if (e && e.get) return e.get.call(n);
    }
    if (Writing?.has(n)) {
      let n = r && (s || !o) ? (r.W !== NOT_PENDING ? r.W : r.j) : u[t];
      n === $DELETED && (n = undefined);
      if (!isWrappable(n)) return n;
      const i = wrap(n, e);
      Writing.add(i);
      return i;
    }
    let l = r ? (s || !o ? read(i[t]) : (read(i[t]), u[t])) : u[t];
    l === $DELETED && (l = undefined);
    if (!r) {
      if (!s && typeof l === "function" && !u.hasOwnProperty(t)) {
        let t;
        return !Array.isArray(e[STORE_VALUE]) &&
          (t = Object.getPrototypeOf(e[STORE_VALUE])) &&
          t !== Object.prototype
          ? l.bind(u)
          : l;
      } else if (getObserver()) {
        return read(getNode(i, t, isWrappable(l) ? wrap(l, e) : l, e[STORE_FIREWALL]?.()));
      }
    }
    return isWrappable(l) ? wrap(l, e) : l;
  },
  has(e, t) {
    if (t === $PROXY || t === $TRACK || t === "__proto__") return true;
    const n =
      e[STORE_OVERRIDE] && t in e[STORE_OVERRIDE]
        ? e[STORE_OVERRIDE][t] !== $DELETED
        : t in e[STORE_VALUE];
    getObserver() && read(getNode(getNodes(e, STORE_HAS), t, n, e[STORE_FIREWALL]?.()));
    return n;
  },
  set(e, t, n) {
    const i = e[$PROXY];
    if (Writing?.has(e[$PROXY])) {
      untrack(() => {
        const r = e[STORE_VALUE];
        const s = r[t];
        const o = e[STORE_OVERRIDE] && t in e[STORE_OVERRIDE] ? e[STORE_OVERRIDE][t] : s;
        const u = n?.[$TARGET]?.[STORE_VALUE] ?? n;
        if (o === u) return true;
        const l = e[STORE_OVERRIDE]?.length || r.length;
        if (u !== undefined && u === s) delete e[STORE_OVERRIDE][t];
        else (e[STORE_OVERRIDE] || (e[STORE_OVERRIDE] = Object.create(null)))[t] = u;
        const c = isWrappable(u);
        if (isWrappable(o)) {
          const e = PARENTS.get(o);
          e && (e instanceof Set ? e.delete(i) : PARENTS.delete(o));
        }
        if (recursivelyNotify(i, storeLookup) && c) recursivelyAddParent(u, i);
        e[STORE_HAS]?.[t] && setSignal(e[STORE_HAS][t], true);
        const a = getNodes(e, STORE_NODE);
        a[t] && setSignal(a[t], () => (c ? wrap(u, e) : u));
        if (Array.isArray(r)) {
          const e = parseInt(t) + 1;
          if (e > l) a.length && setSignal(a.length, e);
        }
        a[$TRACK] && setSignal(a[$TRACK], undefined);
      });
    }
    return true;
  },
  deleteProperty(e, t) {
    if (Writing?.has(e[$PROXY]) && e[STORE_OVERRIDE]?.[t] !== $DELETED) {
      untrack(() => {
        const n =
          e[STORE_OVERRIDE] && t in e[STORE_OVERRIDE] ? e[STORE_OVERRIDE][t] : e[STORE_VALUE][t];
        if (t in e[STORE_VALUE]) {
          (e[STORE_OVERRIDE] || (e[STORE_OVERRIDE] = Object.create(null)))[t] = $DELETED;
        } else if (e[STORE_OVERRIDE] && t in e[STORE_OVERRIDE]) {
          delete e[STORE_OVERRIDE][t];
        } else return true;
        if (isWrappable(n)) {
          const t = PARENTS.get(n);
          t && (t instanceof Set ? t.delete(e) : PARENTS.delete(n));
        }
        if (e[STORE_HAS]?.[t]) setSignal(e[STORE_HAS][t], false);
        const i = getNodes(e, STORE_NODE);
        i[t] && setSignal(i[t], undefined);
        i[$TRACK] && setSignal(i[$TRACK], undefined);
      });
    }
    return true;
  },
  ownKeys(e) {
    trackSelf(e);
    return getKeys(e[STORE_VALUE], e[STORE_OVERRIDE], false);
  },
  getOwnPropertyDescriptor(e, t) {
    if (t === $PROXY) return { value: e[$PROXY], writable: true, configurable: true };
    return getPropertyDescriptor(e[STORE_VALUE], e[STORE_OVERRIDE], t);
  },
  getPrototypeOf(e) {
    return Object.getPrototypeOf(e[STORE_VALUE]);
  }
};
function storeSetter(e, t) {
  const n = Writing;
  Writing = new Set();
  Writing.add(e);
  try {
    const n = t(e);
    if (n !== e && n !== undefined) {
      if (Array.isArray(n)) {
        for (let t = 0, i = n.length; t < i; t++) e[t] = n[t];
        e.length = n.length;
      } else {
        const t = new Set([...Object.keys(e), ...Object.keys(n)]);
        t.forEach(t => {
          if (t in n) e[t] = n[t];
          else delete e[t];
        });
      }
    }
  } finally {
    Writing.clear();
    Writing = n;
  }
}
function createStore(e, t, n) {
  const i = typeof e === "function",
    r = i ? createProjectionInternal(e, t, n).store : wrap(e);
  return [r, e => storeSetter(r, e)];
}
function recursivelyNotify(e, t) {
  let n = e[$TARGET] || t?.get(e)?.[$TARGET];
  let i = false;
  if (n) {
    const e = getNodes(n, STORE_NODE)[$DEEP];
    if (e) {
      setSignal(e, undefined);
      i = true;
    }
    t = n[STORE_LOOKUP] || t;
  }
  const r = PARENTS.get(n?.[STORE_VALUE] || e);
  if (!r) return i;
  if (r instanceof Set) {
    for (let e of r) i = recursivelyNotify(e, t) || i;
  } else i = recursivelyNotify(r, t) || i;
  return i;
}
function recursivelyAddParent(e, t) {
  let n;
  const i = e[$TARGET];
  if (i) {
    n = i[STORE_OVERRIDE];
    e = i[STORE_VALUE];
  }
  if (t) {
    let n = PARENTS.get(e);
    if (!n) PARENTS.set(e, t);
    else if (n !== t) {
      if (!(n instanceof Set)) PARENTS.set(e, (n = new Set([n])));
      else if (n.has(t)) return;
      n.add(t);
    } else return;
  }
  if (Array.isArray(e)) {
    const t = n?.length || e.length;
    for (let i = 0; i < t; i++) {
      const t = n && i in n ? n[i] : e[i];
      isWrappable(t) && recursivelyAddParent(t, e);
    }
  } else {
    const t = getKeys(e, n);
    for (let i = 0; i < t.length; i++) {
      const r = t[i];
      const s = n && r in n ? n[r] : e[r];
      isWrappable(s) && recursivelyAddParent(s, e);
    }
  }
}
function deep(e) {
  recursivelyAddParent(e);
  return e[$DEEP];
}
function createOptimisticStore(e, t, n) {
  return [];
}
function snapshot(e, t, n) {
  let i, r, s, o, u, l;
  if (!isWrappable(e)) return e;
  if (t && t.has(e)) return t.get(e);
  if (!t) t = new Map();
  if ((i = e[$TARGET] || n?.get(e)?.[$TARGET])) {
    s = i[STORE_OVERRIDE];
    r = Array.isArray(i[STORE_VALUE]);
    t.set(
      e,
      s ? (o = r ? [] : Object.create(Object.getPrototypeOf(i[STORE_VALUE]))) : i[STORE_VALUE]
    );
    e = i[STORE_VALUE];
    n = storeLookup;
  } else {
    r = Array.isArray(e);
    t.set(e, e);
  }
  if (r) {
    const i = s?.length || e.length;
    for (let r = 0; r < i; r++) {
      l = s && r in s ? s[r] : e[r];
      if (l === $DELETED) continue;
      if ((u = snapshot(l, t, n)) !== l || o) {
        if (!o) t.set(e, (o = [...e]));
        o[r] = u;
      }
    }
  } else {
    const i = getKeys(e, s);
    for (let r = 0, c = i.length; r < c; r++) {
      let c = i[r];
      const a = getPropertyDescriptor(e, s, c);
      if (a.get) continue;
      l = s && c in s ? s[c] : e[c];
      if ((u = snapshot(l, t, n)) !== e[c] || o) {
        if (!o) {
          o = Object.create(Object.getPrototypeOf(e));
          Object.assign(o, e);
        }
        o[c] = u;
      }
    }
  }
  return o || e;
}
function trueFn() {
  return true;
}
const propTraps = {
  get(e, t, n) {
    if (t === $PROXY) return n;
    return e.get(t);
  },
  has(e, t) {
    if (t === $PROXY) return true;
    return e.has(t);
  },
  set: trueFn,
  deleteProperty: trueFn,
  getOwnPropertyDescriptor(e, t) {
    return {
      configurable: true,
      enumerable: true,
      get() {
        return e.get(t);
      },
      set: trueFn,
      deleteProperty: trueFn
    };
  },
  ownKeys(e) {
    return e.keys();
  }
};
function resolveSource(e) {
  return !(e = typeof e === "function" ? e() : e) ? {} : e;
}
const $SOURCES = Symbol(0);
function merge(...e) {
  if (e.length === 1 && typeof e[0] !== "function") return e[0];
  let t = false;
  const n = [];
  for (let i = 0; i < e.length; i++) {
    const r = e[i];
    t = t || (!!r && $PROXY in r);
    const s = !!r && r[$SOURCES];
    if (s) n.push(...s);
    else n.push(typeof r === "function" ? ((t = true), createMemo(r)) : r);
  }
  if (SUPPORTS_PROXY && t) {
    return new Proxy(
      {
        get(e) {
          if (e === $SOURCES) return n;
          for (let t = n.length - 1; t >= 0; t--) {
            const i = resolveSource(n[t]);
            if (e in i) return i[e];
          }
        },
        has(e) {
          for (let t = n.length - 1; t >= 0; t--) {
            if (e in resolveSource(n[t])) return true;
          }
          return false;
        },
        keys() {
          const e = [];
          for (let t = 0; t < n.length; t++) e.push(...Object.keys(resolveSource(n[t])));
          return [...new Set(e)];
        }
      },
      propTraps
    );
  }
  const i = Object.create(null);
  let r = false;
  let s = n.length - 1;
  for (let e = s; e >= 0; e--) {
    const t = n[e];
    if (!t) {
      e === s && s--;
      continue;
    }
    const o = Object.getOwnPropertyNames(t);
    for (let n = o.length - 1; n >= 0; n--) {
      const u = o[n];
      if (u === "__proto__" || u === "constructor") continue;
      if (!i[u]) {
        r = r || e !== s;
        const n = Object.getOwnPropertyDescriptor(t, u);
        i[u] = n.get ? { enumerable: true, configurable: true, get: n.get.bind(t) } : n;
      }
    }
  }
  if (!r) return n[s];
  const o = {};
  const u = Object.keys(i);
  for (let e = u.length - 1; e >= 0; e--) {
    const t = u[e],
      n = i[t];
    if (n.get) Object.defineProperty(o, t, n);
    else o[t] = n.value;
  }
  o[$SOURCES] = n;
  return o;
}
function omit(e, ...t) {
  const n = new Set(t);
  if (SUPPORTS_PROXY && $PROXY in e) {
    return new Proxy(
      {
        get(t) {
          return n.has(t) ? undefined : e[t];
        },
        has(t) {
          return !n.has(t) && t in e;
        },
        keys() {
          return Object.keys(e).filter(e => !n.has(e));
        }
      },
      propTraps
    );
  }
  const i = {};
  for (const t of Object.getOwnPropertyNames(e)) {
    if (!n.has(t)) {
      const n = Object.getOwnPropertyDescriptor(e, t);
      !n.get && !n.set && n.enumerable && n.writable && n.configurable
        ? (i[t] = n.value)
        : Object.defineProperty(i, t, n);
    }
  }
  return i;
}
function mapArray(e, t, n) {
  const i = typeof n?.keyed === "function" ? n.keyed : undefined;
  return createMemo(
    updateKeyedMap.bind({
      Ie: createOwner(),
      ge: 0,
      ye: e,
      Ce: [],
      De: t,
      Pe: [],
      we: [],
      be: i,
      Ve: i || n?.keyed === false ? [] : undefined,
      Ue: t.length > 1 ? [] : undefined,
      me: n?.fallback
    })
  );
}
const pureOptions = { pureWrite: true };
function updateKeyedMap() {
  const e = this.ye() || [],
    t = e.length;
  e[$TRACK];
  runWithOwner(this.Ie, () => {
    let n,
      i,
      r = this.Ve
        ? () => {
            this.Ve[i] = signal(e[i], pureOptions);
            this.Ue && (this.Ue[i] = signal(i, pureOptions));
            return this.De(
              read.bind(null, this.Ve[i]),
              this.Ue ? read.bind(null, this.Ue[i]) : undefined
            );
          }
        : this.Ue
          ? () => {
              const t = e[i];
              this.Ue[i] = signal(i, pureOptions);
              return this.De(() => t, read.bind(null, this.Ue[i]));
            }
          : () => {
              const t = e[i];
              return this.De(() => t);
            };
    if (t === 0) {
      if (this.ge !== 0) {
        this.Ie.dispose(false);
        this.we = [];
        this.Ce = [];
        this.Pe = [];
        this.ge = 0;
        this.Ve && (this.Ve = []);
        this.Ue && (this.Ue = []);
      }
      if (this.me && !this.Pe[0]) {
        this.Pe[0] = runWithOwner((this.we[0] = createOwner()), this.me);
      }
    } else if (this.ge === 0) {
      if (this.we[0]) this.we[0].dispose();
      this.Pe = new Array(t);
      for (i = 0; i < t; i++) {
        this.Ce[i] = e[i];
        this.Pe[i] = runWithOwner((this.we[i] = createOwner()), r);
      }
      this.ge = t;
    } else {
      let s,
        o,
        u,
        l,
        c,
        a,
        f,
        E = new Array(t),
        d = new Array(t),
        T = this.Ve ? new Array(t) : undefined,
        R = this.Ue ? new Array(t) : undefined;
      for (
        s = 0, o = Math.min(this.ge, t);
        s < o && (this.Ce[s] === e[s] || (this.Ve && compare(this.be, this.Ce[s], e[s])));
        s++
      ) {
        if (this.Ve) setSignal(this.Ve[s], e[s]);
      }
      for (
        o = this.ge - 1, u = t - 1;
        o >= s &&
        u >= s &&
        (this.Ce[o] === e[u] || (this.Ve && compare(this.be, this.Ce[o], e[u])));
        o--, u--
      ) {
        E[u] = this.Pe[o];
        d[u] = this.we[o];
        T && (T[u] = this.Ve[o]);
        R && (R[u] = this.Ue[o]);
      }
      a = new Map();
      f = new Array(u + 1);
      for (i = u; i >= s; i--) {
        l = e[i];
        c = this.be ? this.be(l) : l;
        n = a.get(c);
        f[i] = n === undefined ? -1 : n;
        a.set(c, i);
      }
      for (n = s; n <= o; n++) {
        l = this.Ce[n];
        c = this.be ? this.be(l) : l;
        i = a.get(c);
        if (i !== undefined && i !== -1) {
          E[i] = this.Pe[n];
          d[i] = this.we[n];
          T && (T[i] = this.Ve[n]);
          R && (R[i] = this.Ue[n]);
          i = f[i];
          a.set(c, i);
        } else this.we[n].dispose();
      }
      for (i = s; i < t; i++) {
        if (i in E) {
          this.Pe[i] = E[i];
          this.we[i] = d[i];
          if (T) {
            this.Ve[i] = T[i];
            setSignal(this.Ve[i], e[i]);
          }
          if (R) {
            this.Ue[i] = R[i];
            setSignal(this.Ue[i], i);
          }
        } else {
          this.Pe[i] = runWithOwner((this.we[i] = createOwner()), r);
        }
      }
      this.Pe = this.Pe.slice(0, (this.ge = t));
      this.Ce = e.slice(0);
    }
  });
  return this.Pe;
}
function repeat(e, t, n) {
  return updateRepeat.bind({
    Ie: createOwner(),
    ge: 0,
    ke: 0,
    ve: e,
    De: t,
    we: [],
    Pe: [],
    xe: n?.from,
    me: n?.fallback
  });
}
function updateRepeat() {
  const e = this.ve();
  const t = this.xe?.() || 0;
  runWithOwner(this.Ie, () => {
    if (e === 0) {
      if (this.ge !== 0) {
        this.Ie.dispose(false);
        this.we = [];
        this.Pe = [];
        this.ge = 0;
      }
      if (this.me && !this.Pe[0]) {
        this.Pe[0] = runWithOwner((this.we[0] = createOwner()), this.me);
      }
      return;
    }
    const n = t + e;
    const i = this.ke + this.ge;
    if (this.ge === 0 && this.we[0]) this.we[0].dispose();
    for (let e = n; e < i; e++) this.we[e - this.ke].dispose();
    if (this.ke < t) {
      let e = this.ke;
      while (e < t && e < this.ge) this.we[e++].dispose();
      this.we.splice(0, t - this.ke);
      this.Pe.splice(0, t - this.ke);
    } else if (this.ke > t) {
      let n = i - this.ke - 1;
      let r = this.ke - t;
      this.we.length = this.Pe.length = e;
      while (n >= r) {
        this.we[n] = this.we[n - r];
        this.Pe[n] = this.Pe[n - r];
        n--;
      }
      for (let e = 0; e < r; e++) {
        this.Pe[e] = runWithOwner((this.we[e] = createOwner()), () => this.De(e + t));
      }
    }
    for (let e = i; e < n; e++) {
      this.Pe[e - t] = runWithOwner((this.we[e - t] = createOwner()), () => this.De(e));
    }
    this.Pe = this.Pe.slice(0, e);
    this.ke = t;
    this.ge = e;
  });
  return this.Pe;
}
function compare(e, t, n) {
  return e ? e(t) === e(n) : true;
}
function boundaryComputed(e, t) {
  const n = computed(e, undefined, {
    _e: {
      le() {
        let e = this.J;
        this.J &= ~this.Ge;
        if (this.Ge & STATUS_PENDING && !(this.J & STATUS_UNINITIALIZED)) {
          e &= ~STATUS_PENDING;
        }
        this.Re.notify(this, this.Ge, e);
      },
      Ge: t
    }
  });
  n.Ge = t;
  return n;
}
function createBoundChildren(e, t, n, i) {
  const r = e.Re;
  r.addChild((e.Re = n));
  onCleanup(() => r.removeChild(e.Re));
  return runWithOwner(e, () => {
    const e = computed(t);
    return boundaryComputed(() => staleValues(() => flatten(read(e))), i);
  });
}
class ConditionalQueue extends Queue {
  He;
  Qe = new Set();
  $ = new Set();
  constructor(e) {
    super();
    this.He = e;
  }
  run(e) {
    if (!e || read(this.He)) return;
    return super.run(e);
  }
  notify(e, t, n) {
    if (read(this.He)) {
      if (t & STATUS_PENDING) {
        if (n & STATUS_PENDING) {
          this.$.add(e);
          t &= ~STATUS_PENDING;
        } else if (this.$.delete(e)) t &= ~STATUS_PENDING;
      }
      if (t & STATUS_ERROR) {
        if (n & STATUS_ERROR) {
          this.Qe.add(e);
          t &= ~STATUS_ERROR;
        } else if (this.Qe.delete(e)) t &= ~STATUS_ERROR;
      }
    }
    return t ? super.notify(e, t, n) : true;
  }
}
class CollectionQueue extends Queue {
  $e;
  we = new Set();
  He = signal(false, { pureWrite: true });
  Le = false;
  constructor(e) {
    super();
    this.$e = e;
  }
  run(e) {
    if (!e || read(this.He)) return;
    return super.run(e);
  }
  notify(e, t, n) {
    if (!(t & this.$e) || (this.$e & STATUS_PENDING && this.Le)) return super.notify(e, t, n);
    if (n & this.$e) {
      this.we.add(e);
      if (this.we.size === 1) setSignal(this.He, true);
    } else if (this.we.size > 0) {
      this.we.delete(e);
      if (this.we.size === 0) setSignal(this.He, false);
    }
    t &= ~this.$e;
    return t ? super.notify(e, t, n) : true;
  }
}
var BoundaryMode;
(function (e) {
  e["VISIBLE"] = "visible";
  e["HIDDEN"] = "hidden";
})(BoundaryMode || (BoundaryMode = {}));
function createBoundary(e, t) {
  const n = createOwner();
  const i = new ConditionalQueue(computed(() => t() === BoundaryMode.HIDDEN));
  const r = createBoundChildren(n, e, i, 0);
  computed(() => {
    const e = read(i.He);
    r.Ge = e ? STATUS_ERROR | STATUS_PENDING : 0;
    if (!e) {
      i.$.forEach(e => i.notify(e, STATUS_PENDING, STATUS_PENDING));
      i.Qe.forEach(e => i.notify(e, STATUS_ERROR, STATUS_ERROR));
      i.$.clear();
      i.Qe.clear();
    }
  });
  return () => (read(i.He) ? undefined : read(r));
}
function createCollectionBoundary(e, t, n) {
  const i = createOwner();
  const r = new CollectionQueue(e);
  const s = createBoundChildren(i, t, r, e);
  const o = computed(() => {
    if (!read(r.He)) {
      const e = read(s);
      if (!untrack(() => read(r.He))) r.Le = true;
      return e;
    }
    return n(r);
  });
  return read.bind(null, o);
}
function createLoadBoundary(e, t) {
  return createCollectionBoundary(STATUS_PENDING, e, () => t());
}
function collectErrorSources(e, t) {
  let n = true;
  let i = e.D;
  while (i !== null) {
    const e = i.V;
    if (e.D && e.J & STATUS_ERROR) {
      n = false;
      collectErrorSources(e, t);
    }
    i = i.P;
  }
  n && t.push(e);
}
function createErrorBoundary(e, t) {
  return createCollectionBoundary(STATUS_ERROR, e, e => {
    let n = e.we.values().next().value;
    return t(n.Y, () => {
      const t = [];
      for (const n of e.we) collectErrorSources(n, t);
      for (const e of t) recompute(e);
      schedule();
    });
  });
}
function flatten(e, t) {
  if (typeof e === "function" && !e.length) {
    if (t?.doNotUnwrap) return e;
    do {
      e = e();
    } while (typeof e === "function" && !e.length);
  }
  if (t?.skipNonRendered && (e == null || e === true || e === false || e === "")) return;
  if (Array.isArray(e)) {
    let n = [];
    if (flattenArray(e, n, t)) {
      return () => {
        let e = [];
        flattenArray(n, e, { ...t, doNotUnwrap: false });
        return e;
      };
    }
    return n;
  }
  return e;
}
function flattenArray(e, t = [], n) {
  let i = null;
  let r = false;
  for (let s = 0; s < e.length; s++) {
    try {
      let i = e[s];
      if (typeof i === "function" && !i.length) {
        if (n?.doNotUnwrap) {
          t.push(i);
          r = true;
          continue;
        }
        do {
          i = i();
        } while (typeof i === "function" && !i.length);
      }
      if (Array.isArray(i)) {
        r = flattenArray(i, t, n);
      } else if (n?.skipNonRendered && (i == null || i === true || i === false || i === "")) {
      } else t.push(i);
    } catch (e) {
      if (!(e instanceof NotReadyError)) throw e;
      i = e;
    }
  }
  if (i) throw i;
  return r;
}
export {
  $PROXY,
  $TARGET,
  $TRACK,
  ContextNotFoundError,
  NoOwnerError,
  NotReadyError,
  SUPPORTS_PROXY,
  createAsync,
  createBoundary,
  createContext,
  createEffect,
  createErrorBoundary,
  createLoadBoundary,
  createMemo,
  createOptimistic,
  createOptimisticStore,
  createProjection,
  createReaction,
  createRenderEffect,
  createRoot,
  createSignal,
  createStore,
  createTrackedEffect,
  deep,
  flatten,
  flush,
  getContext,
  getNextChildId,
  getObserver,
  getOwner,
  isEqual,
  isPending,
  isWrappable,
  mapArray,
  merge,
  omit,
  onCleanup,
  onSettled,
  pending,
  reconcile,
  repeat,
  resolve,
  runWithOwner,
  setContext,
  snapshot,
  untrack
};
